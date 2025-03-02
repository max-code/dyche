use std::collections::BTreeMap;
use std::time::Instant;

use crate::autocompletes::{autocomplete_league_or_user, autocomplete_league_or_user_value};
use crate::utils::common::{check_discord_user_registered, get_not_registered_title_and_message};
use crate::utils::embed::Embed;
use crate::{log_call, log_timer, start_timer};
use crate::{Context, Error};

use fpl_common::types::LeagueId;
use fpl_db::queries::mini_league::get_league_name;
use fpl_db::queries::team::get_team_name_from_discord_id;
use tracing::{debug, info};

const COMMAND: &str = "/hits";

#[poise::command(slash_command, user_cooldown = 1)]
pub async fn hits(
    ctx: Context<'_>,
    #[description = "Hits for a single user or entire league."]
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

    let value: i64 = league_or_user_value.parse::<i64>()?;

    let rows = match league_or_user.as_str() {
        "User" => match check_discord_user_registered(&ctx.data().pool, value).await? {
            true => {
                let user_hits = get_user_hits(ctx, &timer, value).await?;
                log_timer!(timer, COMMAND, ctx, "got user hits");
                user_hits
            }
            false => {
                let (title, message) = get_not_registered_title_and_message(value);
                Embed::from_ctx(ctx)?
                    .error()
                    .title(title)
                    .body(message)
                    .send()
                    .await?;
                return Ok(());
            }
        },
        "League" => {
            let league_hits = get_league_hits(ctx, &timer, LeagueId::from(value as i32)).await?;
            log_timer!(timer, COMMAND, ctx, "got league hits");
            league_hits
        }
        _ => {
            return Err("Unknown league_or_user type".into());
        }
    };

    let league_or_user_string = match league_or_user.as_str() {
        "User" => {
            let team_name = get_team_name_from_discord_id(&ctx.data().pool, value).await?;
            log_timer!(timer, COMMAND, ctx, "fetched team_name");
            team_name
        }
        "League" => {
            let league_name =
                get_league_name(&ctx.data().pool, LeagueId::new(value as i32)).await?;
            log_timer!(timer, COMMAND, ctx, "fetched league_name");
            league_name
        }
        _ => {
            return Err("Unknown league_or_user_type".into());
        }
    };

    Embed::from_ctx(ctx)?
        .success()
        .title(format!("Hits for {league_or_user_string}"))
        .add_pages_from_strings(rows, None)
        .send()
        .await?;

    Ok(())
}

pub async fn get_user_hits(
    ctx: Context<'_>,
    timer: &Instant,
    discord_id: i64,
) -> Result<Vec<String>, Error> {
    let user_hits = sqlx::query!(
        "
        SELECT tgw.game_week_id, tgw.event_transfers_cost 
        FROM discord_users du 
        JOIN teams t ON du.team_id = t.id 
        JOIN team_game_weeks tgw ON tgw.team_id = t.id 
        WHERE event_transfers_cost > 0 and du.discord_id = $1
        ORDER BY tgw.game_week_id ASC;
        ",
        discord_id
    )
    .fetch_all(&*ctx.data().pool)
    .await
    .map(|rows| {
        // First collect all rows
        rows.into_iter()
            .map(|row| (row.game_week_id, row.event_transfers_cost))
            .collect::<Vec<(i16, i16)>>()
    })?;

    log_timer!(timer, COMMAND, ctx, "fetched hits team");

    let hit_rows = match user_hits.len() {
        0 => {
            vec!["Has not taken a hit".to_string()]
        }
        _ => {
            let mut result = Vec::new();
            result.push("**__Hits__**\n".to_string());
            let mut total_hits = 0;

            result.extend(user_hits.into_iter().map(|(game_week, hits_cost)| {
                let hits = hits_cost / 4;
                total_hits += hits;
                format!("**GW{}** - {}", game_week, hits)
            }));

            result.push(format!(
                "\n**Total:** {total_hits} (-{} points)",
                total_hits * 4
            ));

            result
        }
    };
    log_timer!(timer, COMMAND, ctx, "grouped and formatted rows");
    Ok(hit_rows)
}

pub async fn get_league_hits(
    ctx: Context<'_>,
    timer: &Instant,
    league_id: LeagueId,
) -> Result<Vec<String>, Error> {
    let league_hits = sqlx::query!(
        "
        SELECT tgw.game_week_id, mls.player_name, mls.entry_name, tgw.event_transfers_cost FROM team_game_weeks tgw 
        JOIN mini_league_standings mls ON mls.team_id = tgw.team_id 
        WHERE tgw.event_transfers_cost > 0 AND mls.league_id = $1;
        ",
        i32::from(league_id)
    )
    .fetch_all(&*ctx.data().pool)
    .await
    .map(|rows| {
        // First collect all rows
        rows.into_iter()
            .map(|row| {
                (
                    row.game_week_id,
                    row.player_name,
                    row.entry_name,
                    row.event_transfers_cost,
                )
            })
            .collect::<Vec<(i16, String, String, i16)>>()
    })?;

    log_timer!(timer, COMMAND, ctx, "fetched hits league");

    let hit_rows = match league_hits.len() {
        0 => {
            vec!["No one has taken a hit".to_string()]
        }
        _ => {
            // Group by game week
            type TeamRows = Vec<(String, String, i16)>;
            let mut hits_game_week: BTreeMap<i16, TeamRows> = BTreeMap::new();
            for row in league_hits {
                hits_game_week
                    .entry(row.0)
                    .or_default()
                    .push((row.1, row.2, row.3));
            }

            let mut result = Vec::new();
            result.push("**__Hits__**\n".to_string());

            result.extend(hits_game_week.into_iter().map(|(game_week, users)| {
                let users_formatted = users
                    .into_iter()
                    .map(|(user_name, team_name, hits_cost)| {
                        format!("{user_name} ({team_name}) **{}**", hits_cost / 4)
                    })
                    .collect::<Vec<_>>()
                    .join(", ");

                format!("**GW{}** - {}", game_week, users_formatted)
            }));

            result
        }
    };
    log_timer!(timer, COMMAND, ctx, "grouped and formatted rows");
    Ok(hit_rows)
}
