use crate::utils::autocomplete::{
    self, autocomplete_league_or_player, autocomplete_league_or_player_stub,
    autocomplete_mini_league,
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
    #[description = "arg1"]
    #[autocomplete = "autocomplete_league_or_player"]
    arg1: String,
    #[description = "arg2"]
    #[autocomplete = "autocomplete_league_or_player_stub"]
    arg2: String,
) -> Result<(), Error> {
    info!(
        "{} called by {} with arg1({}) arg2({})",
        COMMAND,
        ctx.author().id,
        arg1,
        arg2
    );

    ctx.say(format!("Arg1: {} Arg2: {}", arg1, arg2)).await?;
    Ok(())
}
