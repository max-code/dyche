use crate::utils::autocomplete::{autocomplete_mini_league, autocomplete_player};
use crate::utils::paginator::maybe_paginate_rows;
use crate::{Context, Error};

use fpl_common::types::{LeagueId, PlayerId};
use fpl_db::queries::game_week::get_current_game_week;
use tracing::info;

const COMMAND: &str = "/whohas";

#[poise::command(slash_command)]
pub async fn whohas(
    ctx: Context<'_>,
    #[description = "Mini League"]
    #[autocomplete = "autocomplete_mini_league"]
    league_id: LeagueId,
    #[description = "Player"]
    #[autocomplete = "autocomplete_player"]
    player_id: PlayerId,
) -> Result<(), Error> {
    info!(
        "{} called by {} with league_id({}) and player_id({})",
        COMMAND,
        ctx.author().id,
        league_id,
        player_id
    );

    let whohas = get_whohas(&ctx, league_id, player_id).await?;
    maybe_paginate_rows(ctx, whohas, COMMAND).await
}

pub async fn get_whohas(
    ctx: &Context<'_>,
    league_id: LeagueId,
    player_id: PlayerId,
) -> Result<Vec<String>, Error> {
    let current_game_week = get_current_game_week(&ctx.data().pool).await?;
    let whohas = sqlx::query!(
        r#"
        SELECT
            mls.player_name,
            mls.entry_name
        FROM team_game_week_picks tgwp
        JOIN mini_league_standings mls ON tgwp.team_id = mls.team_id
        WHERE tgwp.game_week_id = $1
        AND tgwp.player_id = $2
        AND mls.league_id = $3
        "#,
        i16::from(current_game_week.id),
        i16::from(player_id),
        i32::from(league_id)
    )
    .fetch_all(&*ctx.data().pool)
    .await
    .map(|rows| {
        rows.into_iter()
            .map(|row| (row.player_name, row.entry_name))
            .collect::<Vec<(String, String)>>()
    })?;

    match whohas.len() {
        0 => Ok(vec!["No one.".to_string()]),
        _ => Ok(whohas
            .into_iter()
            .map(|(player_name, entry_name)| format!("**{}** - {}", player_name, entry_name))
            .collect::<Vec<String>>()),
    }
}
