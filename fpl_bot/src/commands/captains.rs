use crate::autocompletes::autocomplete_mini_league;
use crate::utils::embed::Embed;
use crate::{log_call, log_timer, start_timer, Context, Error};
use std::time::Instant;
use tracing::{debug, info};

use fpl_common::types::{LeagueId, PlayerId};
use fpl_db::queries::game_week::get_current_game_week;

const COMMAND: &str = "/captains";

#[poise::command(slash_command)]
pub async fn captains(
    ctx: Context<'_>,
    #[description = "Mini League"]
    #[autocomplete = "autocomplete_mini_league"]
    league_id: LeagueId,
) -> Result<(), Error> {
    log_call!(COMMAND, ctx, "league_id", league_id);
    let timer: Instant = start_timer!();

    let current_game_week = get_current_game_week(&ctx.data().pool).await?;
    let captains = sqlx::query!(
        r#"
        SELECT
            mls.player_name,
            p.id as "id!",
            p.web_name
        FROM mini_league_standings mls
        JOIN mini_leagues ml ON mls.league_id = ml.id
        JOIN team_game_week_picks tgwp ON tgwp.team_id = mls.team_id
        JOIN players p ON p.id = tgwp.player_id
        WHERE tgwp.game_week_id = $1
        AND ml.id = $2
        AND tgwp.is_captain = true
        ORDER BY p.id ASC
        "#,
        i16::from(current_game_week.id),
        i32::from(league_id)
    )
    .fetch_all(&*ctx.data().pool)
    .await
    .map(|rows| {
        rows.into_iter()
            .map(|row| (row.player_name, PlayerId::from(row.id), row.web_name))
            .collect::<Vec<(String, PlayerId, String)>>()
    })?;

    log_timer!(timer, COMMAND, ctx, "fetched captains");

    let captains_rows = captains
        .into_iter()
        .map(|(player_name, _, web_name)| format!("**{}** captained **{}**", player_name, web_name))
        .collect::<Vec<String>>();

    Embed::from_ctx(ctx)?
        .success()
        .title("Captains".to_string())
        .add_pages_from_strings(captains_rows, None)
        .send()
        .await?;

    Ok(())
}
