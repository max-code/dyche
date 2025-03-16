use crate::autocompletes::{
    autocomplete_mini_league, autocomplete_player_or_club, autocomplete_player_or_club_value,
};
use crate::utils::embed::Embed;
use crate::{handle_async_fallible, handle_parse_value, Context, Error};
use fpl_db::models::GameWeek;
use fpl_db::queries::mini_league::get_league_name;
use std::collections::{BTreeMap, HashMap};
use std::time::Instant;
use tracing::{debug, info};

use crate::{log_call, log_timer, start_timer};
use fpl_common::types::{ClubId, LeagueId, PlayerId};
use fpl_db::queries::game_week::get_current_game_week;

const COMMAND: &str = "/whohas";

#[poise::command(slash_command)]
pub async fn whohas(
    ctx: Context<'_>,
    #[description = "Mini League"]
    #[autocomplete = "autocomplete_mini_league"]
    league_id: LeagueId,
    #[description = "Player"]
    #[autocomplete = "autocomplete_player_or_club"]
    player_or_club: String,
    #[description = "Player"]
    #[autocomplete = "autocomplete_player_or_club_value"]
    player_or_club_value: String,
) -> Result<(), Error> {
    log_call!(
        COMMAND,
        ctx,
        "league_id",
        league_id,
        "player_or_club",
        player_or_club,
        "player_or_club_value",
        player_or_club_value
    );
    let timer: Instant = start_timer!();

    let current_game_week: fpl_db::models::GameWeek =
        get_current_game_week(&ctx.data().pool).await?;

    let value: i16 = handle_parse_value!(
        ctx,
        player_or_club_value,
        i16,
        "Bad Player/Club value provided."
    );

    let rows = match player_or_club.as_str() {
        "Player" => {
            let whohas_player = get_whohas_player(
                ctx,
                &timer,
                current_game_week,
                PlayerId::from(value),
                league_id,
            )
            .await?;
            log_timer!(timer, COMMAND, ctx, "got whohas player");
            whohas_player
        }
        "Club" => {
            let club_id = match ClubId::try_from(value) {
                Ok(v) => v,
                Err(e) => {
                    Embed::from_ctx(ctx)?
                        .error()
                        .body("Bad Club value provided")
                        .send()
                        .await?;

                    return Err(e.into());
                }
            };

            let whohas_club = handle_async_fallible!(
                ctx,
                get_whohas_club(ctx, &timer, current_game_week, club_id, league_id),
                "Error calling get_whohas_club"
            );
            log_timer!(timer, COMMAND, ctx, "got whohas club");
            whohas_club
        }
        _ => {
            return Err("Unknown player_or_club_value".into());
        }
    };

    let league_name = handle_async_fallible!(
        ctx,
        get_league_name(&ctx.data().pool, league_id),
        "Error calling get_league_name"
    );
    log_timer!(timer, COMMAND, ctx, "fetched league_name");

    Embed::from_ctx(ctx)?
        .success()
        .title(format!("Who has in {league_name}"))
        .add_pages_from_strings(rows, None)
        .send()
        .await?;

    Ok(())
}

async fn get_whohas_player(
    ctx: Context<'_>,
    timer: &Instant,
    current_game_week: GameWeek,
    player_id: PlayerId,
    league_id: LeagueId,
) -> Result<Vec<String>, Error> {
    // Fetch
    let whohas = sqlx::query!(
        r#"
        SELECT
            mls.player_name,
            mls.entry_name,
            tgwp.is_captain,
            tgwp.is_vice_captain,
            tgwp.multiplier
        FROM team_game_week_picks tgwp
        JOIN mini_league_standings mls ON tgwp.team_id = mls.team_id
        WHERE tgwp.game_week_id = $1
        AND tgwp.player_id = $2
        AND mls.league_id = $3
        "#,
        i16::from(current_game_week.id),
        i16::from(player_id),
        i32::from(league_id)
    )
    .fetch_all(&*ctx.data().pool)
    .await
    .map(|rows| {
        rows.into_iter()
            .map(|row| {
                (
                    row.player_name,
                    row.entry_name,
                    row.is_captain,
                    row.is_vice_captain,
                    row.multiplier,
                )
            })
            .collect::<Vec<(String, String, bool, bool, i16)>>()
    })?;

    log_timer!(timer, COMMAND, ctx, "fetched whohas players");

    // Fetch 2
    let player_name = sqlx::query!(
        "SELECT web_name FROM players WHERE id = $1",
        i16::from(player_id)
    )
    .fetch_one(&*ctx.data().pool)
    .await?
    .web_name;
    log_timer!(timer, COMMAND, ctx, "fetched player names");

    let whohas_rows = match whohas.len() {
        0 => vec![format!(
            "No one has {player_name} in **GW{}**.",
            current_game_week.id
        )],
        _ => {
            // Group by (is_captain, is_vice_captian) ->
            // - (true, _) = captained
            // - (_, true) = VC
            // - (_, _) = other
            type CaptainsBools = (bool, bool);
            type WhoHasRows = Vec<(String, String, i16)>;
            let mut grouped: HashMap<CaptainsBools, WhoHasRows> = HashMap::new();
            for row in whohas {
                grouped
                    .entry((row.2, row.3))
                    .or_default()
                    .push((row.0, row.1, row.4));
            }

            let mut result: Vec<String> = Vec::new();
            result.push(format!(
                "**__{player_name} owners in GW{}__**\n",
                current_game_week.id
            ));

            let group_order = [
                ((true, false), "**__Captained__**"),
                ((false, true), "**__Vice Captained__**"),
                ((false, false), "**__Regular__**"),
            ];

            for (key, label) in group_order.iter() {
                if let Some(group) = grouped.get(key) {
                    result.push(label.to_string());
                    for (player_name, team_name, multiplier) in group {
                        let status = match multiplier {
                            0 => "(Benched)",
                            3 => "(Triple Captain)",
                            _ => "",
                        };
                        result.push(format!("- **{}** ({}) {}", player_name, team_name, status));
                    }
                }
            }

            result
        }
    };
    log_timer!(timer, COMMAND, ctx, "grouped and formatted rows");
    Ok(whohas_rows)
}

async fn get_whohas_club(
    ctx: Context<'_>,
    timer: &Instant,
    current_game_week: GameWeek,
    club_id: ClubId,
    league_id: LeagueId,
) -> Result<Vec<String>, Error> {
    // Fetch
    let whohas = sqlx::query!(
        r#"
        SELECT
            mls.player_name,
            mls.entry_name,
            p.web_name
        FROM team_game_week_picks tgwp
        JOIN mini_league_standings mls ON tgwp.team_id = mls.team_id
        JOIN players p ON tgwp.player_id = p.id
        WHERE tgwp.game_week_id = $1
        AND p.team = $2
        AND mls.league_id = $3
        ORDER BY web_name ASC
        "#,
        i16::from(current_game_week.id),
        i16::from(club_id),
        i32::from(league_id)
    )
    .fetch_all(&*ctx.data().pool)
    .await
    .map(|rows| {
        rows.into_iter()
            .map(|row| (row.web_name, row.player_name, row.entry_name))
            .collect::<Vec<(String, String, String)>>()
    })?;

    log_timer!(timer, COMMAND, ctx, "fetched whohas club");

    // Fetch 2
    let club_name = sqlx::query!("SELECT name FROM clubs WHERE id = $1", i16::from(club_id))
        .fetch_one(&*ctx.data().pool)
        .await?
        .name;
    log_timer!(timer, COMMAND, ctx, "fetched club name");

    let whohas_rows = match whohas.len() {
        0 => vec![format!(
            "No one has anyone from {club_name} in **GW{}**.",
            current_game_week.id
        )],
        _ => {
            let mut player_users = BTreeMap::<String, Vec<(String, String)>>::new();
            for row in whohas {
                player_users.entry(row.0).or_default().push((row.1, row.2));
            }

            let mut result = Vec::new();
            result.push(format!(
                "**__{club_name} assets in GW{}__**\n",
                current_game_week.id
            ));

            result.extend(player_users.into_iter().map(|(player, users)| {
                let users_formatted = users
                    .into_iter()
                    .map(|(user_name, team_name)| format!("{user_name} ({team_name})"))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("- **{}:** {}", player, users_formatted)
            }));

            result
        }
    };
    log_timer!(timer, COMMAND, ctx, "grouped and formatted rows");
    Ok(whohas_rows)
}
