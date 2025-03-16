use std::time::Instant;

use crate::{
    handle_async_fallible,
    images::{UniquePlayers, UniqueRenderer},
    render,
};
use fpl_common::types::{GameWeekId, LeagueId};
use fpl_db::queries::{
    game_week::get_current_game_week, mini_league::get_league_name,
    team::get_team_name_from_discord_id,
};
use serenity::all::User;
use tracing::debug;

use crate::{
    autocompletes::autocomplete_mini_league,
    commands::get_image_file_path,
    log_call, log_timer, start_timer,
    utils::embed::{Embed, EmbedPage},
    Context, Error,
};

const COMMAND: &str = "/unique";

#[poise::command(slash_command)]
pub async fn unique(
    ctx: Context<'_>,
    #[description = "Mini League"]
    #[autocomplete = "autocomplete_mini_league"]
    league_id: LeagueId,
    #[description = "User"] user: Option<User>,
    #[description = "Game Week"] game_week: Option<GameWeekId>,
) -> Result<(), Error> {
    log_call!(COMMAND, ctx, "user", user, "game_week", game_week);
    let timer: Instant = start_timer!();

    let embed = Embed::from_ctx(ctx)?
        .processing()
        .title("Processing unique request")
        .send()
        .await?;

    let game_week_id: i16 = match game_week {
        Some(gw) => i16::from(gw),
        None => {
            let current_gw = handle_async_fallible!(
                ctx,
                embed,
                get_current_game_week(&ctx.data().pool),
                "Error calling get_current_game_week"
            );
            i16::from(current_gw.id)
        }
    };
    let user_id = match user {
        Some(u) => i64::from(u.id),
        None => i64::from(ctx.author().id),
    };

    let unique_players = handle_async_fallible!(
        ctx,
        embed,
        get_unique_player_names_for_team_in_league(ctx, league_id, user_id, game_week_id),
        "Error calling get_unique_player_names_for_team_in_league"
    );

    let team_name = handle_async_fallible!(
        ctx,
        embed,
        get_team_name_from_discord_id(&ctx.data().pool, user_id),
        "Error calling get_team_name_from_discord_id"
    );
    log_timer!(timer, COMMAND, ctx, "fetched team_name");

    let league_name = handle_async_fallible!(
        ctx,
        embed,
        get_league_name(&ctx.data().pool, league_id),
        "Error calling get_league_name"
    );
    log_timer!(timer, COMMAND, ctx, "fetched league_name");

    let file_name = get_image_file_path(COMMAND, &ctx);
    let renderer = UniqueRenderer::default();
    render!(
        ctx,
        embed,
        renderer,
        unique_players,
        &file_name,
        "Failed to render unique players"
    );
    log_timer!(timer, COMMAND, ctx, "rendered image");

    embed
        .success()
        .title(format!(
            "Unique players for {team_name} in Gameweek {game_week_id} among {league_name}"
        ))
        .add_page(EmbedPage::new().with_image(file_name))
        .send()
        .await?;

    Ok(())
}

async fn get_unique_player_names_for_team_in_league(
    ctx: Context<'_>,
    league_id: LeagueId,
    discord_id: i64,
    game_week_id: i16,
) -> Result<UniquePlayers, Error> {
    let records = sqlx::query!(
        r#"
        WITH 
        -- Get the team ID for the specified Discord user
        user_team AS (
            SELECT team_id
            FROM discord_users
            WHERE discord_id = $1
        ),
        -- Get all teams in the specified league
        league_teams AS (
            SELECT team_id
            FROM mini_league_standings
            WHERE league_id = $2
        ),
        -- Find players used by other teams in the league (not the user's team)
        other_league_team_players AS (
            SELECT DISTINCT player_id
            FROM team_game_week_picks tgwp
            JOIN user_team ut ON tgwp.team_id != ut.team_id
            WHERE tgwp.game_week_id = $3
            AND tgwp.team_id IN (SELECT team_id FROM league_teams)
        )
        -- Select player names that are only on the user's team
        SELECT p.web_name, p.code, tgwp.multiplier, tgwp.is_captain, tgwp.is_vice_captain, po.opponents
        FROM team_game_week_picks tgwp
        JOIN players p ON p.id = tgwp.player_id
        JOIN user_team ut ON tgwp.team_id = ut.team_id
        join player_opponents po on p.id = po.player_id and po.game_week_id = tgwp.game_week_id
        WHERE tgwp.game_week_id = $3
        AND tgwp.player_id NOT IN (
            SELECT player_id FROM other_league_team_players
        )
        "#,
        discord_id,
        i32::from(league_id),
        game_week_id
    )
    .fetch_all(&*ctx.data().pool)
    .await?;

    let mut unique_players = UniquePlayers::new();

    for row in records {
        unique_players = unique_players.add_player(
            row.web_name,
            row.code as u32,
            row.multiplier,
            row.is_captain,
            row.is_vice_captain,
            row.opponents.unwrap_or("N/A".to_string()),
        );
    }

    Ok(unique_players)
}
