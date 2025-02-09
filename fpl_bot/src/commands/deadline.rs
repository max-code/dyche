use chrono::Datelike;
use fpl_common::types::GameWeekId;
use fpl_db::queries::game_week::get_current_game_week;
use ordinal::Ordinal;
use sqlx::types::chrono::{DateTime, Utc};
use tracing::info;

use crate::{utils::paginator::paginate, Context, Error};

const COMMAND: &str = "/deadline";

#[poise::command(slash_command, prefix_command)]
pub async fn deadline(
    ctx: Context<'_>,
    #[description = "Game Week"] game_week_id: Option<GameWeekId>,
) -> Result<(), Error> {
    info!(
        "{} called by {} with game_week_id({:?})",
        COMMAND,
        ctx.author().id,
        game_week_id
    );

    let game_week_deadlines = sqlx::query!(
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
    })?;

    let mut pages = game_week_deadlines
        .into_iter()
        .map(|(name, deadline_time)| {
            let day = deadline_time.day();
            format!(
                "**{}** Deadline: {}",
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
            pages.rotate_left(index);
        }
        None => {
            let current_gw = get_current_game_week(&ctx.data().pool).await?;
            let next_gw_id = match current_gw.id.next() {
                Some(gw_id) => gw_id,
                None => current_gw.id,
            };
            let index = (next_gw_id.0 - 1) as usize;
            pages.rotate_left(index);
        }
    }

    paginate(
        ctx,
        COMMAND,
        &pages.iter().map(|p| p.as_str()).collect::<Vec<_>>(),
    )
    .await?;

    Ok(())
}
