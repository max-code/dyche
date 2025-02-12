use crate::utils::autocomplete::{
    self, autocomplete_league_or_user, autocomplete_league_or_user_value, autocomplete_mini_league,
};
use crate::utils::paginator::maybe_paginate_rows;
use crate::{Context, Error};

use fpl_common::types::{LeagueId, PlayerId};
use fpl_db::queries::game_week::get_current_game_week;
use tracing::info;

const COMMAND: &str = "/chips";

#[poise::command(slash_command)]
pub async fn chips(
    ctx: Context<'_>,
    #[description = "Chips for a single user or entire league."]
    #[autocomplete = "autocomplete_league_or_user"]
    league_or_user: String,
    #[description = "User/League"]
    #[autocomplete = "autocomplete_league_or_user_value"]
    league_or_user_value: String,
) -> Result<(), Error> {
    info!(
        "{} called by {} with league_or_user({}) league_or_user_value({})",
        COMMAND,
        ctx.author().id,
        league_or_user,
        league_or_user_value
    );

    ctx.say(format!(
        "league_or_user: {} Arleague_or_user_valueg2: {}",
        league_or_user, league_or_user_value
    ))
    .await?;
    Ok(())
}
