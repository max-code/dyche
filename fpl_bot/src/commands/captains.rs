use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn captains(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Captain command executed!").await?;
    Ok(())
}
