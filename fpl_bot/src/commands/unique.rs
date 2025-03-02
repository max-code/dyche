use std::time::Instant;

use fpl_common::types::{GameWeekId, LeagueId};
use fpl_db::queries::{
    game_week::get_current_game_week, mini_league::get_league_name,
    team::get_team_name_from_discord_id,
};
use serenity::all::User;
use tracing::debug;

use crate::{
    autocompletes::autocomplete_mini_league, log_call, log_timer, start_timer, utils::embed::Embed,
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
        None => i16::from(get_current_game_week(&ctx.data().pool).await?.id),
    };

    let user_id = match user {
        Some(u) => i64::from(u.id),
        None => i64::from(ctx.author().id),
    };

    let unique_players =
        get_unique_player_names_for_team_in_league(ctx, league_id, user_id, game_week_id).await?;

    let team_name = get_team_name_from_discord_id(&ctx.data().pool, user_id).await?;
    log_timer!(timer, COMMAND, ctx, "fetched team_name");

    let league_name = get_league_name(&ctx.data().pool, league_id).await?;
    log_timer!(timer, COMMAND, ctx, "fetched league_name");

    embed
        .success()
        .title(format!(
            "Unique players for {team_name} in Gameweek {game_week_id} among {league_name}"
        ))
        .add_pages_from_strings(unique_players, Some(10))
        .send()
        .await?;

    Ok(())
}

async fn get_unique_player_names_for_team_in_league(
    ctx: Context<'_>,
    league_id: LeagueId,
    discord_id: i64,
    game_week_id: i16,
) -> Result<Vec<String>, Error> {
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
        SELECT p.web_name
        FROM team_game_week_picks tgwp
        JOIN players p ON p.id = tgwp.player_id
        JOIN user_team ut ON tgwp.team_id = ut.team_id
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

    let player_names: Vec<String> = records
        .into_iter()
        .map(|row| format!("- {}", row.web_name))
        .collect();

    Ok(player_names)
}
