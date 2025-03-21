use crate::autocompletes::{autocomplete_mini_league, autocomplete_overall_or_week};
use crate::commands::get_image_file_path;
use crate::images::{TableData, TableRenderer};
use crate::utils::embed::{Embed, EmbedPage};
use crate::{handle_async_fallible, log_call, log_timer, render, start_timer, Context, Error};
use fpl_db::queries::mini_league::get_league_name;
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

    let embed = Embed::from_ctx(ctx)?
        .processing()
        .title("Processing table request")
        .send()
        .await?;

    let mut live_points = handle_async_fallible!(
        ctx,
        embed,
        get_points(&ctx, league_id),
        "Error calling get_points"
    );

    live_points.sort_by_key(|lp| {
        Reverse(match overall_or_week.as_str() {
            "Overall" => lp.calculated_overall_points,
            "Current Gameweek" => lp.calculated_week_points,
            _ => lp.calculated_overall_points,
        })
    });
    log_timer!(timer, COMMAND, ctx, "fetched live points");

    let league_name = handle_async_fallible!(
        ctx,
        embed,
        get_league_name(&ctx.data().pool, league_id),
        "Error calling get_get_league_namepoints"
    );
    log_timer!(timer, COMMAND, ctx, "fetched league_name");

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

    let file_name = get_image_file_path(COMMAND, &ctx);
    let renderer: TableRenderer = TableRenderer::default();
    render!(
        ctx,
        embed,
        renderer,
        data,
        &file_name,
        "Failed to render table"
    );
    log_timer!(timer, COMMAND, ctx, "rendered image");

    embed
        .success()
        .title(format!(
            "{overall_or_week} League standings for {league_name}"
        ))
        .add_page(EmbedPage::new().with_image(file_name))
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
