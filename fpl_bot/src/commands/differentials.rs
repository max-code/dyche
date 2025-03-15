use std::time::Instant;

use crate::images::{DifferentialKey, Differentials, DifferentialsRenderer};
use fpl_common::types::{GameWeekId, LeagueId};
use fpl_db::queries::{
    game_week::get_current_game_week,
    mini_league::{get_league_name, get_team_ids_from_league_id},
    team::{get_team_ids_from_discord_ids, get_team_name_from_discord_id},
};
use tracing::debug;

use crate::{
    autocompletes::{autocomplete_league_or_user, autocomplete_league_or_user_value},
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

    let team_ids = match league_or_user.as_str() {
        "User" => {
            let ids = vec![value, ctx.author().id.get() as i64];
            let team_ids = get_team_ids_from_discord_ids(&ctx.data().pool, &ids).await?;
            log_timer!(timer, COMMAND, ctx, "got team_ids from discord_ids");
            team_ids
        }
        "League" => {
            let team_ids =
                get_team_ids_from_league_id(&ctx.data().pool, LeagueId::new(value as i32)).await?;
            log_timer!(timer, COMMAND, ctx, "got team_ids for ml");
            team_ids
        }
        _ => {
            return Err("Unknown league_or_user_type".into());
        }
    };

    let user_or_league_name = match league_or_user.as_str() {
        "User" => {
            let caller_name = get_team_name_from_discord_id(&ctx.data().pool, value).await?;
            let other_user_name =
                get_team_name_from_discord_id(&ctx.data().pool, ctx.author().id.get() as i64)
                    .await?;
            log_timer!(timer, COMMAND, ctx, "got team names from discord_ids");
            format!("{caller_name} and {other_user_name}")
        }
        "League" => {
            let league_name =
                get_league_name(&ctx.data().pool, LeagueId::new(value as i32)).await?;
            log_timer!(timer, COMMAND, ctx, "got league name for ml");
            league_name
        }
        _ => {
            return Err("Unknown league_or_user_type".into());
        }
    };

    let differentials = get_differentials_for_user_ids(ctx, team_ids, game_week_id).await?;

    let file_name = get_image_file_path(COMMAND, &ctx);
    let renderer = DifferentialsRenderer::default();
    renderer.render(differentials, &file_name).await?;
    log_timer!(timer, COMMAND, ctx, "rendered differentials image");

    embed
        .success()
        .title(format!(
            "Differentials for {} in GW{}",
            user_or_league_name, game_week_id
        ))
        .add_page(EmbedPage::new().with_image(file_name))
        .send()
        .await?;

    Ok(())
}

async fn get_differentials_for_user_ids(
    ctx: Context<'_>,
    team_ids: Vec<i32>,
    game_week_id: i16,
) -> Result<Differentials, Error> {
    let records = sqlx::query!(
        r#"
            SELECT name, player_first_name as user_first_name, player_last_name as user_last_name, player_name, code, is_captain, is_vice_captain, opponents
            FROM (
                SELECT 
                t."name", t.player_first_name , t.player_last_name,
                    p.web_name as player_name,
                    p.code,
                    tgwp.is_captain, 
                    tgwp.is_vice_captain,
                    po.opponents,
                    COUNT(*) OVER (PARTITION BY tgwp.player_id) as player_count
                FROM teams t
                JOIN team_game_week_picks tgwp ON t.id = tgwp.team_id
                JOIN players p ON p.id = tgwp.player_id
                JOIN player_opponents po on p.id = po.player_id and po.game_week_id = tgwp.game_week_id
                WHERE tgwp.game_week_id = $1 
                AND t.id = ANY($2)
                AND tgwp.multiplier > 0
            ) AS filtered_players
            WHERE player_count = 1;        
        "#,
        game_week_id,
        &team_ids
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
            row.is_captain,
            row.is_vice_captain,
            row.opponents.unwrap_or("N/A".to_string()),
        );
    }

    Ok(differentials)
}
