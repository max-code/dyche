use std::collections::HashMap;
use std::str::FromStr;
use std::time::Instant;

use crate::autocompletes::{autocomplete_league_or_user, autocomplete_league_or_user_value};
use crate::utils::common::{check_discord_user_registered, get_not_registered_title_and_message};
use crate::utils::embed::Embed;
use crate::{handle_async_fallible, log_call, log_timer, handle_parse_value, start_timer};
use crate::{Context, Error};

use fpl_common::types::{Chip, LeagueId, TeamId};
use fpl_db::queries::mini_league::get_league_name;
use fpl_db::queries::team::get_team_name_from_discord_id;
use tracing::{debug, info};

const COMMAND: &str = "/chips";

#[poise::command(slash_command, user_cooldown = 1)]
pub async fn chips(
    ctx: Context<'_>,
    #[description = "Chips for a single user or entire league."]
    #[autocomplete = "autocomplete_league_or_user"]
    league_or_user: String,
    #[description = "User/League"]
    #[autocomplete = "autocomplete_league_or_user_value"]
    league_or_user_value: String,
) -> Result<(), Error> {
    log_call!(
        COMMAND,
        ctx,
        "league_or_user",
        league_or_user,
        "league_or_user_value",
        league_or_user_value
    );
    let timer = start_timer!();

    let value: i64 = handle_parse_value!(ctx, league_or_user_value, i64, "Bad User/League value provided.");

    let rows = match league_or_user.as_str() {
        "User" => match check_discord_user_registered(&ctx.data().pool, value).await {
            Ok(true) => {
                let user_chips = handle_async_fallible!(ctx, get_user_chips(ctx, value), "Error calling get_user_chips");
                log_timer!(timer, COMMAND, ctx, "got user chips");
                user_chips
            }
            Ok(false) => {
                let (title, message) = get_not_registered_title_and_message(value);
                Embed::from_ctx(ctx)?
                    .error()
                    .title(title)
                    .body(message)
                    .send()
                    .await?;
                return Ok(());
            }
            Err(e) => {
                Embed::from_ctx(ctx)?
                    .error()
                    .body(format!("Error when calling {}", COMMAND))
                    .send()
                    .await?;
                return Err(format!(
                    "Unknown error when calling check_discord_user_registered: {}",
                    e
                )
                .into());
            }
        },
        "League" => {
            let league_chips = handle_async_fallible!(ctx, get_league_chips(ctx, LeagueId::new(value as i32)), "Error calling get_league_chips");
            log_timer!(timer, COMMAND, ctx, "got league chips");
            league_chips
        }
        _ => {
            return Err("Unknown league_or_user_type".into());
        }
    };

    let league_or_user_string = match league_or_user.as_str() {
        "User" => {
            let team_name = handle_async_fallible!(ctx, get_team_name_from_discord_id(&ctx.data().pool, value), "Error calling get_team_name_from_discord_id");
            log_timer!(timer, COMMAND, ctx, "fetched team_name");
            team_name
        }
        "League" => {
            let league_name = handle_async_fallible!(ctx, get_league_name(&ctx.data().pool, LeagueId::new(value as i32)), "Error calling get_league_name");
            log_timer!(timer, COMMAND, ctx, "fetched league_name");
            league_name
        }
        _ => {
            return Err("Unknown league_or_user_type".into());
        }
    };

    Embed::from_ctx(ctx)?
        .success()
        .title(format!("Chips for {league_or_user_string}"))
        .add_pages_from_strings(rows, None)
        .send()
        .await?;

    Ok(())
}

pub async fn get_league_chips(ctx: Context<'_>, league_id: LeagueId) -> Result<Vec<String>, Error> {
    let league_chips = sqlx::query!(
        "
        SELECT tgw.team_id, mls.player_name, mls.entry_name, tgw.active_chip, tgw.game_week_id FROM team_game_weeks tgw 
        JOIN mini_league_standings mls ON mls.team_id = tgw.team_id 
        WHERE tgw.active_chip IS NOT NULL AND mls.league_id = $1;
        ",
        i32::from(league_id)
    )
    .fetch_all(&*ctx.data().pool)
    .await
    .map(|rows| {
        // First collect all rows
        let rows = rows.into_iter()
            .map(|row| {
                (
                    TeamId::from(row.team_id),
                    row.player_name,
                    row.entry_name,
                    row.active_chip.unwrap(),
                    row.game_week_id,
                )
            })
            .collect::<Vec<_>>();

        // Group by team_id
        type TeamRows = Vec<(TeamId, String, String, String, i16)>;
        let mut grouped: HashMap<TeamId, TeamRows> = HashMap::new();
        for row in rows {
            grouped.entry(row.0)
                .or_default()
                .push(row);
        }

        // Format each group into a string
        grouped.into_values()
            .map(|mut team_rows| {
                team_rows.sort_by_key(|row| row.4);
                
                let first_row = &team_rows[0];
                let player_name = &first_row.1;
                let entry_name = &first_row.2;
                
                // Format chip usage
                let chips = team_rows.iter()
                    .map(|row| {
                        match Chip::from_str(row.3.as_str()) {
                            Ok(chip) => format!("**GW{}** {}", row.4, chip.pretty_name()),
                            Err(_) => format!("**GW{}** {}", row.4, row.3) // Fallback to raw string if parsing fails
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ");

                format!("**{}** ({})\n- {}", player_name, entry_name, chips)
            })
            .collect::<Vec<String>>()
    })?;

    Ok(league_chips)
}

pub async fn get_user_chips(ctx: Context<'_>, discord_id: i64) -> Result<Vec<String>, Error> {
    let user_chips = sqlx::query!(
        "
        SELECT tgw.game_week_id, tgw.active_chip 
        FROM discord_users du 
        JOIN teams t ON du.team_id = t.id 
        JOIN team_game_weeks tgw ON tgw.team_id = t.id 
        WHERE active_chip IS NOT NULL and du.discord_id = $1;
        ",
        discord_id
    )
    .fetch_all(&*ctx.data().pool)
    .await
    .map(|rows| {
        rows.into_iter()
            .map(|row| (row.active_chip.unwrap(), row.game_week_id))
            .collect::<Vec<(String, i16)>>()
    })?;

    let formatted_chips = user_chips
        .into_iter()
        .map(|(chip, gw)| {
            Chip::from_str(chip.as_str())
                .map(|parsed_chip| format!("**GW{}**: {}", gw, parsed_chip.pretty_name()))
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(formatted_chips)
}
