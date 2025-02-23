use crate::autocompletes::{autocomplete_mini_league, autocomplete_overall_or_week};
use crate::images::{TableData, TableRenderer};
use crate::utils::embed_builder_v2::Embed;
use crate::{log_call, log_timer, start_timer, Context, Error};
use sqlx::FromRow;
use std::cmp::Reverse;
use std::time::Instant;
use tracing::{debug, info};

use fpl_common::types::LeagueId;

#[derive(FromRow)]
pub struct LivePoints {
    pub player_first_name: String,
    pub player_last_name: String,
    pub name: String,
    pub discord_id: Option<i64>,
    pub week_points: i16,
    pub calculated_week_points: i64,
    pub overall_points: i16,
    pub calculated_overall_points: i64,
}

const COMMAND: &str = "/table";

#[poise::command(slash_command)]
pub async fn table(
    ctx: Context<'_>,
    #[description = "Mini League"]
    #[autocomplete = "autocomplete_mini_league"]
    league_id: LeagueId,
    #[description = "Overall or Current Game Week"]
    #[autocomplete = "autocomplete_overall_or_week"]
    overall_or_week: String,
) -> Result<(), Error> {
    log_call!(
        COMMAND,
        ctx,
        "league_id",
        league_id,
        "overall_or_week",
        overall_or_week
    );
    let timer: Instant = start_timer!();

    let mut embed = Embed::new(ctx)?
        .title("Processing table request")
        .processing();

    embed.send().await?;

    let mut live_points = get_points(&ctx, league_id).await?;
    live_points.sort_by_key(|lp| {
        Reverse(match overall_or_week.as_str() {
            "Overall" => lp.calculated_overall_points,
            "Current Gameweek" => lp.calculated_week_points,
            _ => lp.calculated_overall_points,
        })
    });
    log_timer!(timer, COMMAND, ctx, "fetched live points");

    let title = format!("{} League Standings", overall_or_week);
    let mut data: TableData = TableData::new(title.to_string());
    for lp in live_points {
        let is_caller = match lp.discord_id {
            Some(id) => id == i64::from(ctx.author().id),
            None => false,
        };
        match overall_or_week.as_str() {
            "Overall" => {
                data.add_row(
                    lp.name,
                    format!("{} {}", lp.player_first_name, lp.player_last_name),
                    lp.overall_points as u16,
                    lp.calculated_overall_points as u16,
                    is_caller,
                );
            }
            "Current Gameweek" => {
                data.add_row(
                    lp.name,
                    format!("{} {}", lp.player_first_name, lp.player_last_name),
                    lp.week_points as u16,
                    lp.calculated_week_points as u16,
                    is_caller,
                );
            }
            _ => return Err("Unknown overall_or_week".into()),
        }
    }

    let renderer = TableRenderer::default();
    renderer
        .render(data, "/Users/maxjordan/code/dyche/fpl_bot/table.png")
        .await?;
    log_timer!(timer, COMMAND, ctx, "rendered image");

    embed
        .image("/Users/maxjordan/code/dyche/fpl_bot/table.png")
        .success()
        .send()
        .await?;
    Ok(())
}

pub async fn get_points(ctx: &Context<'_>, league_id: LeagueId) -> Result<Vec<LivePoints>, Error> {
    Ok(sqlx::query_as!(
        LivePoints,
        r#"
        SELECT 
            player_first_name as "player_first_name!",
            player_last_name as "player_last_name!",
            name as "name!",
            discord_id,
            week_points as "week_points!",
            calculated_week_points as "calculated_week_points!",
            overall_points as "overall_points!",
            calculated_overall_points as "calculated_overall_points!"
        FROM live_points
        WHERE team_id IN (
            SELECT team_id 
            FROM mini_league_standings 
            WHERE league_id = $1
        )
        "#,
        i32::from(league_id)
    )
    .fetch_all(&*ctx.data().pool)
    .await?)
}
