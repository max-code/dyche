use std::time::Instant;

use fpl_common::types::GameWeekId;
use serenity::all::User;

use crate::{
    autocompletes, log_call, log_timer, start_timer, utils::embed_builder_v2::Embed, Context, Error,
};

const COMMAND: &str = "/team";

#[poise::command(slash_command)]
pub async fn team(
    ctx: Context<'_>,
    #[description = "User"] user: Option<User>,
    #[description = "Game Week"] game_week: Option<GameWeekId>,
) -> Result<(), Error> {
    log_call!(COMMAND, ctx, "user", user, "game_week", game_week);
    let timer: Instant = start_timer!();

    let mut embed = Embed::new(ctx)?
        .title("Processing team request")
        .processing();

    embed.send().await?;

    Ok(())
}
