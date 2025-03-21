use chrono::Datelike;
use ordinal::Ordinal;
use sqlx::types::chrono::{DateTime, Utc};
use std::time::Instant;
use tracing::{debug, info};

use crate::utils::embed::Embed;
use crate::{log_call, log_timer, start_timer};
use crate::{Context, Error};
use fpl_common::types::GameWeekId;
use fpl_db::queries::game_week::get_current_game_week;

const COMMAND: &str = "/deadline";

#[poise::command(slash_command, prefix_command)]
pub async fn deadline(
    ctx: Context<'_>,
    #[description = "Game Week"] game_week_id: Option<GameWeekId>,
) -> Result<(), Error> {
    log_call!(COMMAND, ctx, "game_week_id", game_week_id);
    let timer = start_timer!();

    let game_week_deadlines = match sqlx::query!(
        r#"
        SELECT
            name, deadline_time
        FROM
            game_weeks
        "#,
    )
    .fetch_all(&*ctx.data().pool)
    .await
    .map(|rows| {
        rows.into_iter()
            .map(|row| (row.name, row.deadline_time))
            .collect::<Vec<(String, DateTime<Utc>)>>()
    }) {
        Ok(values) => values,
        Err(e) => {
            Embed::from_ctx(ctx)?
                .error()
                .body(format!("Error when calling {}", COMMAND))
                .send()
                .await?;

            return Err(e.into());
        }
    };
    log_timer!(timer, COMMAND, ctx, "fetched deadlines");

    let mut deadline_rows = game_week_deadlines
        .into_iter()
        .map(|(name, deadline_time)| {
            let day = deadline_time.day();
            format!(
                "**{}**: {}",
                name,
                deadline_time.format(&format!("%B {}, %l:%M %p, %Y", Ordinal(day)))
            )
        })
        .collect::<Vec<String>>();

    // If user provided a GW, use that. Otherwise, try
    // and get the next GW (if that doesnt exist e.g. gw38 use the current deadline)
    match game_week_id {
        Some(gw) => {
            let index = (gw.0 - 1) as usize;
            deadline_rows.rotate_left(index);
        }
        None => {
            let current_gw = get_current_game_week(&ctx.data().pool).await?;
            let next_gw_id = match current_gw.id.next() {
                Some(gw_id) => gw_id,
                None => current_gw.id,
            };
            let index = (next_gw_id.0 - 1) as usize;
            deadline_rows.rotate_left(index);
        }
    }
    log_timer!(timer, COMMAND, ctx, "formatted deadlines");

    Embed::from_ctx(ctx)?
        .success()
        .title("Deadline".to_string())
        .add_pages_from_strings(deadline_rows, Some(1))
        .send()
        .await?;

    Ok(())
}
