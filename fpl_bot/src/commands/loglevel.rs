use std::time::Instant;
use tracing::{debug, info};

use crate::{log_call, log_timer, start_timer};
use crate::{Context, Error};

use tracing_subscriber::EnvFilter;

const COMMAND: &str = "/loglevel";
const OWNER_ID: u64 = 254938708741062657; // Your Discord ID as a constant instead of env var

#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR",
    hide_in_help = true
)]
pub async fn loglevel(
    ctx: Context<'_>,
    #[description = "Module and level (e.g. fpl_bot=debug)"] directive: String,
) -> Result<(), Error> {
    if ctx.author().id.to_string() != OWNER_ID.to_string() {
        return Err("This command is restricted to the bot owner".into());
    }

    log_call!(COMMAND, ctx, "directive", directive);
    let timer = start_timer!();

    let new_directive = match directive.parse() {
        Ok(d) => d,
        Err(e) => {
            log_timer!(timer, COMMAND, ctx, "failed to parse directive");
            return Err(format!("Invalid directive format: {}", e).into());
        }
    };

    let handle = ctx.data().log_levels.clone();

    handle
        .modify(|filter| {
            // Create new filter with base directives
            let new_filter = EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::default())
                .add_directive(new_directive);
            *filter = new_filter;
        })
        .map_err(|e| format!("Failed to update log level: {}", e))?;

    log_timer!(timer, COMMAND, ctx, "updated log level");
    ctx.say(format!("Successfully updated log level: {}", directive))
        .await?;
    Ok(())
}
