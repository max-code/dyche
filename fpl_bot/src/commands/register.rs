use crate::{Context, Error};
use fpl_common::types::{FixtureId, TeamId};
use tracing::info;

#[poise::command(slash_command)]
pub async fn register(
    ctx: Context<'_>,
    #[description = "Team ID from the FPL website"] team_id: TeamId,
) -> Result<(), Error> {
    info!("Register command called");
    ctx.say(format!("Register team id {}", team_id)).await?;
    Ok(())
}
