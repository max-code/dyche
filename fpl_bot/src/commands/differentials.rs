use std::time::Instant;

use fpl_bot::images::{DifferentialKey, Differentials, UniquePlayers, UniqueRenderer};
use fpl_common::types::{game_week_id, GameWeekId, LeagueId};
use fpl_db::queries::{
    game_week::get_current_game_week,
    mini_league::{get_discord_users_from_league_id, get_league_name},
    team::get_team_name_from_discord_id,
};
use serenity::all::User;
use tracing::debug;

use crate::{
    autocompletes::{
        autocomplete_league_or_user, autocomplete_league_or_user_value, league_or_user,
    },
    commands::get_image_file_path,
    log_call, log_timer, start_timer,
    utils::embed::{Embed, EmbedPage},
    Context, Error,
};

const COMMAND: &str = "/differentials";

#[poise::command(slash_command)]
pub async fn differentials(
    ctx: Context<'_>,
    #[description = "Differential players for a single user or entire league."]
    #[autocomplete = "autocomplete_league_or_user"]
    league_or_user: String,
    #[description = "User/League"]
    #[autocomplete = "autocomplete_league_or_user_value"]
    league_or_user_value: String,
    #[description = "Game Week"] game_week: Option<GameWeekId>,
) -> Result<(), Error> {
    log_call!(
        COMMAND,
        ctx,
        "league_or_user",
        league_or_user,
        "league_or_user_value",
        league_or_user_value,
        "game_week",
        game_week
    );
    let timer: Instant = start_timer!();

    let embed = Embed::from_ctx(ctx)?
        .processing()
        .title("Processing differentials request")
        .send()
        .await?;

    let game_week_id: i16 = match game_week {
        Some(gw) => i16::from(gw),
        None => i16::from(get_current_game_week(&ctx.data().pool).await?.id),
    };

    let value: i64 = league_or_user_value.parse::<i64>()?;

    let discord_ids = match league_or_user.as_str() {
        "User" => {
            vec![value, ctx.author().id.get() as i64]
        }
        "League" => {
            let discord_ids =
                get_discord_users_from_league_id(&ctx.data().pool, LeagueId::new(value as i32))
                    .await?;
            log_timer!(timer, COMMAND, ctx, "got discord_ids for ml");
            discord_ids
        }
        _ => {
            return Err("Unknown league_or_user_type".into());
        }
    };

    let differentials = get_differentials_for_user_ids(ctx, discord_ids, game_week_id).await?;
    println!("{:#?}", differentials);

    // let file_name = get_image_file_path(COMMAND, &ctx);
    // let renderer = UniqueRenderer::default();
    // renderer.render(unique_players, &file_name).await?;
    // log_timer!(timer, COMMAND, ctx, "rendered image");

    // embed
    //     .success()
    //     .title(format!(
    //         "Unique players for {team_name} in Gameweek {game_week_id} among {league_name}"
    //     ))
    //     .add_page(EmbedPage::new().with_image(file_name))
    //     .send()
    //     .await?;

    Ok(())
}

async fn get_differentials_for_user_ids(
    ctx: Context<'_>,
    discord_ids: Vec<i64>,
    game_week_id: i16,
) -> Result<Differentials, Error> {
    let records = sqlx::query!(
        r#"
            SELECT name, player_first_name as user_first_name, player_last_name as user_last_name, discord_id, player_name, code, multiplier, is_captain, is_vice_captain
            FROM (
                SELECT 
                t."name", t.player_first_name , t.player_last_name,
                    du.discord_id, 
                    p.web_name as player_name,
                    p.code,
                    tgwp.multiplier,
                    tgwp.is_captain, 
                    tgwp.is_vice_captain,
                    COUNT(*) OVER (PARTITION BY tgwp.player_id) as player_count
                FROM discord_users du
                JOIN teams t ON du.team_id = t.id
                JOIN team_game_week_picks tgwp ON t.id = tgwp.team_id
                JOIN players p ON p.id = tgwp.player_id
                WHERE tgwp.game_week_id = $1 
                AND du.discord_id = ANY($2)
                AND tgwp.multiplier > 0
            ) AS filtered_players
            WHERE player_count = 1
            order by discord_id;        
        "#,
        game_week_id,
        &discord_ids as &[i64]
    )
    .fetch_all(&*ctx.data().pool)
    .await?;

    let mut differentials = Differentials::new();

    for row in records {
        let key = DifferentialKey {
            team_name: row.name,
            user_first_name: row.user_first_name,
            user_last_name: row.user_last_name,
        };

        differentials = differentials.add_differential(
            key,
            row.player_name,
            row.code,
            row.multiplier,
            row.is_captain,
            row.is_vice_captain,
        );
    }

    Ok(differentials)
}
