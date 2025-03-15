use std::{collections::HashMap, str::FromStr, time::Instant};

use crate::images::{
    GameStatus, PlayerGameInfo, PlayerInfo, TeamData, TeamDataBuilder, TeamRenderer, TransferInfo,
};
use fpl_common::types::{Chip, GameWeekId, PlayerPosition};
use fpl_db::queries::{game_week::get_current_game_week, team::get_team_name_from_discord_id};
use serenity::all::User;
use tracing::debug;

use crate::{
    commands::get_image_file_path,
    log_call, log_timer, start_timer,
    utils::embed::{Embed, EmbedPage},
    Context, Error,
};

const COMMAND: &str = "/team";

#[poise::command(slash_command)]
pub async fn team(
    ctx: Context<'_>,
    #[description = "User"] user: Option<User>,
    #[description = "Game Week"] game_week: Option<GameWeekId>,
) -> Result<(), Error> {
    log_call!(COMMAND, ctx, "user", user, "game_week", game_week);
    let timer: Instant = start_timer!();

    let embed = Embed::from_ctx(ctx)?
        .processing()
        .title("Processing team request")
        .send()
        .await?;

    let game_week_id: i16 = match game_week {
        Some(gw) => i16::from(gw),
        None => i16::from(get_current_game_week(&ctx.data().pool).await?.id),
    };

    let user_id = match user {
        Some(u) => i64::from(u.id),
        None => i64::from(ctx.author().id),
    };

    let data: TeamData = get_team_data(ctx, user_id, game_week_id, &timer).await?;
    let file_name = get_image_file_path(COMMAND, &ctx);

    let team_name = get_team_name_from_discord_id(&ctx.data().pool, user_id).await?;
    log_timer!(timer, COMMAND, ctx, "fetched team_name");

    let renderer = TeamRenderer::default();
    renderer.render(data, &file_name).await?;
    log_timer!(timer, COMMAND, ctx, "rendered image");

    embed
        .success()
        .title(format!("Team for {team_name} in Gameweek {game_week_id}"))
        .add_page(EmbedPage::new().with_image(file_name))
        .send()
        .await?;

    Ok(())
}

async fn get_team_data(
    ctx: Context<'_>,
    user_id: i64,
    game_week_id: i16,
    timer: &Instant,
) -> Result<TeamData, Error> {
    let mut data = TeamData::builder();
    data = get_basic_team_data(ctx, user_id, game_week_id, data).await?;
    log_timer!(timer, COMMAND, ctx, "Got basic team data");
    data = get_player_data(ctx, user_id, game_week_id, data).await?;
    log_timer!(timer, COMMAND, ctx, "Got player data");
    data = get_transfers_data(ctx, user_id, game_week_id, data).await?;
    log_timer!(timer, COMMAND, ctx, "Got transfers data");
    Ok(data.build()?)
}

#[derive(sqlx::FromRow)]
struct TeamQueryResult {
    team_name: String,
    gw_rank: Option<i32>,
    overall_rank: i32,
    chip: Option<String>,
    points: Option<i16>,
}

async fn get_basic_team_data(
    ctx: Context<'_>,
    user_id: i64,
    game_week: i16,
    mut team_data: TeamDataBuilder,
) -> Result<TeamDataBuilder, Error> {
    let current_gw = get_current_game_week(&ctx.data().pool).await?.id;
    let is_current = i16::from(current_gw) == game_week;

    let result = if is_current {
        sqlx::query_as!(
            TeamQueryResult,
            r#"
            SELECT t.name AS team_name, 
                   tgw.rank AS gw_rank, 
                   tgw.overall_rank AS overall_rank, 
                   tgw.active_chip AS chip, 
                   lp.calculated_week_points::smallint AS points
            FROM team_game_weeks tgw
            JOIN teams t ON t.id = tgw.team_id
            JOIN discord_users du ON du.team_id = t.id
            JOIN live_points lp ON lp.team_id = t.id
            WHERE tgw.game_week_id = $1
            AND du.discord_id = $2
            LIMIT 1;
            "#,
            game_week,
            user_id
        )
        .fetch_one(&*ctx.data().pool)
        .await?
    } else {
        sqlx::query_as!(
            TeamQueryResult,
            r#"
            SELECT t.name AS team_name, 
                   tgw.rank AS gw_rank, 
                   tgw.overall_rank AS overall_rank, 
                   tgw.active_chip AS chip, 
                   tgw.points AS points
            FROM team_game_weeks tgw
            JOIN teams t ON t.id = tgw.team_id
            JOIN discord_users du ON du.team_id = t.id
            WHERE tgw.game_week_id = $1
            AND du.discord_id = $2
            LIMIT 1;
            "#,
            game_week,
            user_id
        )
        .fetch_one(&*ctx.data().pool)
        .await?
    };

    team_data = team_data
        .points(result.points.unwrap_or_default() as i64)
        .team_name(result.team_name)
        .gw_rank(result.gw_rank.unwrap_or_default() as i64)
        .overall_rank(result.overall_rank.into())
        .game_week(GameWeekId::new(game_week)?);

    if let Some(chip_str) = result.chip {
        if let Ok(chip) = Chip::from_str(chip_str.as_str()) {
            team_data = team_data.add_chip(chip);
        }
    }

    Ok(team_data)
}

async fn get_player_data(
    ctx: Context<'_>,
    user_id: i64,
    game_week: i16,
    mut team_data: TeamDataBuilder,
) -> Result<TeamDataBuilder, Error> {
    let mut results = sqlx::query!(
        r#"
        WITH combined_player_fixtures AS (
            SELECT
                ph.player_id,
                ph.fixture_id,
                ph.was_home AS is_home
            FROM player_history ph
            JOIN fixtures f ON ph.fixture_id = f.id AND f.started = true
            UNION ALL
            SELECT
                pf.player_id,
                pf.fixture_id,
                pf.is_home
            FROM player_fixtures pf
            JOIN fixtures f ON pf.fixture_id = f.id AND f.started = false
        )
        SELECT
            p.web_name as name,
            p.code as code,
            CASE
                WHEN bwc.bonus = 0 AND bwc.bps > 0 THEN gwp.total_points + bwc.calculated_bonus
                ELSE gwp.total_points
            END as "points!",
            tgwp.is_captain as captain,
            tgwp.is_vice_captain as vice_captain,
            tgwp.multiplier as multiplier,
            tgwp.position as "position!",
            tgwp.element_type as player_position,
            f.started as "started!",
            gwp.minutes as minutes,
            tgwp.player_id as player_id,
            c.short_name as short_name,
            cpf.is_home as "is_home!"
        FROM team_game_week_picks tgwp
        JOIN discord_users du ON du.team_id = tgwp.team_id
        JOIN game_week_players gwp ON tgwp.player_id = gwp.player_id AND gwp.game_week_id = tgwp.game_week_id
        JOIN players p ON gwp.player_id = p.id
        JOIN combined_player_fixtures cpf ON cpf.player_id = tgwp.player_id
        JOIN fixtures f ON f.id = cpf.fixture_id AND f.game_week_id = tgwp.game_week_id
        JOIN clubs c ON c.id = CASE WHEN cpf.is_home THEN f.home_team_id ELSE f.away_team_id END
        LEFT JOIN bonus_with_calculated bwc ON f.id = bwc.fixture_id AND p.id = bwc.player_id
        WHERE du.discord_id = $1 AND tgwp.game_week_id = $2;
        "#,
        user_id,
        game_week
    )
    .fetch_all(&*ctx.data().pool)
    .await?;

    let mut player_games: HashMap<i16, Vec<PlayerGameInfo>> = HashMap::new();

    for result in &results {
        let player_game_info = if result.started {
            if result.minutes == 0 && result.position != 16 {
                PlayerGameInfo::Status(GameStatus::NotPlayed)
            } else {
                let heuristic_multiplier = if result.position <= 15 && result.position >= 12 {
                    1
                } else {
                    result.multiplier
                };
                PlayerGameInfo::Status(GameStatus::Played(
                    result.points as i16 * heuristic_multiplier,
                ))
            }
        } else {
            let home_or_away = if result.is_home { "H" } else { "A" };
            PlayerGameInfo::Fixture(format!("{} ({})", result.short_name, home_or_away))
        };

        player_games
            .entry(result.player_id)
            .or_default()
            .push(player_game_info);
    }

    // POST PROCESS: If all of their games have been played, combine into 1 entry
    for games in player_games.values_mut() {
        if games.len() > 1 {
            // Check if all entries are GameStatus::Played
            let all_played = games
                .iter()
                .all(|g| matches!(g, PlayerGameInfo::Status(GameStatus::Played(_))));

            if all_played {
                let points = match games.first() {
                    Some(PlayerGameInfo::Status(GameStatus::Played(p))) => *p,
                    _ => continue,
                };

                games.clear();
                games.push(PlayerGameInfo::Status(GameStatus::Played(points)));
            }
        }
    }
    results.sort_by_key(|r| r.player_id);
    results.dedup_by_key(|r| r.player_id);

    let mut bench_players = Vec::new();

    for result in results {
        let game_info = player_games
            .get(&result.player_id)
            .expect("Player should exist in games map");

        let player_info = PlayerInfo::new(
            result.name,
            result.code as u32,
            game_info.clone(),
            result.captain,
            result.vice_captain,
        );

        match result.position {
            // Playing team
            1..=11 => match PlayerPosition::from_str(result.player_position.as_str()) {
                Ok(position) => match position {
                    PlayerPosition::Goalkeeper => {
                        team_data = team_data.goalkeeper(player_info);
                    }
                    PlayerPosition::Defender => {
                        team_data = team_data.add_defender(player_info);
                    }
                    PlayerPosition::Midfielder => {
                        team_data = team_data.add_midfielder(player_info);
                    }
                    PlayerPosition::Attacker => {
                        team_data = team_data.add_forward(player_info);
                    }
                    PlayerPosition::Manager => {
                        team_data = team_data.add_manager(player_info);
                    }
                },
                Err(e) => return Err(e.into()),
            },
            // Bench - collect for later
            12..=15 => {
                bench_players.push((result.position, player_info));
            }
            16 => {
                team_data = team_data.add_manager(player_info);
            }
            _ => {
                return Err("Position > 16 on team game week pick!".into());
            }
        }
    }

    // Sort bench players by position and add them in order
    bench_players.sort_by_key(|(pos, _)| *pos);
    for (_, player_info) in bench_players {
        team_data = team_data.add_bench_player(player_info);
    }

    Ok(team_data)
}

async fn get_transfers_data(
    ctx: Context<'_>,
    user_id: i64,
    game_week: i16,
    mut team_data: TeamDataBuilder,
) -> Result<TeamDataBuilder, Error> {
    let transfers: Vec<TransferInfo> = sqlx::query_as!(
        TransferInfo,
        r#"
    SELECT 
        player_in.web_name as "player_in_name!",
        player_in.code as "player_in_code!",
        (t.player_in_cost::float8 / 10) as "player_in_cost!",
        player_out.web_name as "player_out_name!",
        player_out.code as "player_out_code!",
        (t.player_out_cost::float8 / 10) as "player_out_cost!"
    FROM 
        transfers t
        LEFT JOIN players player_in ON t.player_in_id = player_in.id
        LEFT JOIN players player_out ON t.player_out_id = player_out.id
        left join discord_users du on du.team_id = t.team_id 
        WHERE t.game_week_id = $1 and du.discord_id = $2;
    "#,
        game_week,
        user_id
    )
    .fetch_all(&*ctx.data().pool)
    .await?;

    for transfer in transfers {
        team_data = team_data.add_transfer(transfer);
    }

    Ok(team_data)
}
