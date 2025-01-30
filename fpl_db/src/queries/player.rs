use fpl_common::types::PlayerId;
use sqlx::PgPool;
use tracing::{debug, info};

use crate::models::{player::Player, PlayerFixtureDb, PlayerHistoryDb, PlayerHistoryPastDb};

pub async fn upsert_players(pool: &PgPool, players: &[Player]) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    info!("Upserting {} Player rows", players.len());

    for player in players {
        sqlx::query!(
           r#"
           INSERT INTO players (
               id, can_transact, can_select, chance_of_playing_next_round, chance_of_playing_this_round,
               code, cost_change_event, cost_change_event_fall, cost_change_start, cost_change_start_fall,
               dreamteam_count, element_type, ep_next, ep_this, event_points,
               first_name, form, in_dreamteam, news, news_added,
               now_cost, photo, points_per_game, removed, second_name,
               selected_by_percent, special, squad_number, status, team,
               team_code, total_points, transfers_in, transfers_in_event, transfers_out,
               transfers_out_event, value_form, value_season, web_name, region,
               team_join_date, minutes, goals_scored, assists, clean_sheets,
               goals_conceded, own_goals, penalties_saved, penalties_missed, yellow_cards,
               red_cards, saves, bonus, bps, influence,
               creativity, threat, ict_index, starts, expected_goals,
               expected_assists, expected_goal_involvements, expected_goals_conceded, influence_rank, influence_rank_type,
               creativity_rank, creativity_rank_type, threat_rank, threat_rank_type, ict_index_rank,
               ict_index_rank_type, corners_and_indirect_freekicks_order, corners_and_indirect_freekicks_text, direct_freekicks_order, direct_freekicks_text,
               penalties_order, penalties_text, expected_goals_per_90, saves_per_90, expected_assists_per_90,
               expected_goal_involvements_per_90, expected_goals_conceded_per_90, goals_conceded_per_90, now_cost_rank, now_cost_rank_type,
               form_rank, form_rank_type, points_per_game_rank, points_per_game_rank_type, selected_rank,
               selected_rank_type, starts_per_90, clean_sheets_per_90
           )
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15,
                   $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30,
                   $31, $32, $33, $34, $35, $36, $37, $38, $39, $40, $41, $42, $43, $44, $45,
                   $46, $47, $48, $49, $50, $51, $52, $53, $54, $55, $56, $57, $58, $59, $60,
                   $61, $62, $63, $64, $65, $66, $67, $68, $69, $70, $71, $72, $73, $74, $75,
                   $76, $77, $78, $79, $80, $81, $82, $83, $84, $85, $86, $87, $88, $89, $90,
                   $91, $92, $93)
           ON CONFLICT (id) DO UPDATE SET
               can_transact = EXCLUDED.can_transact,
               can_select = EXCLUDED.can_select,
               chance_of_playing_next_round = EXCLUDED.chance_of_playing_next_round,
               chance_of_playing_this_round = EXCLUDED.chance_of_playing_this_round,
               code = EXCLUDED.code,
               cost_change_event = EXCLUDED.cost_change_event,
               cost_change_event_fall = EXCLUDED.cost_change_event_fall,
               cost_change_start = EXCLUDED.cost_change_start,
               cost_change_start_fall = EXCLUDED.cost_change_start_fall,
               dreamteam_count = EXCLUDED.dreamteam_count,
               element_type = EXCLUDED.element_type,
               ep_next = EXCLUDED.ep_next,
               ep_this = EXCLUDED.ep_this,
               event_points = EXCLUDED.event_points,
               first_name = EXCLUDED.first_name,
               form = EXCLUDED.form,
               in_dreamteam = EXCLUDED.in_dreamteam,
               news = EXCLUDED.news,
               news_added = EXCLUDED.news_added,
               now_cost = EXCLUDED.now_cost,
               photo = EXCLUDED.photo,
               points_per_game = EXCLUDED.points_per_game,
               removed = EXCLUDED.removed,
               second_name = EXCLUDED.second_name,
               selected_by_percent = EXCLUDED.selected_by_percent,
               special = EXCLUDED.special,
               squad_number = EXCLUDED.squad_number,
               status = EXCLUDED.status,
               team = EXCLUDED.team,
               team_code = EXCLUDED.team_code,
               total_points = EXCLUDED.total_points,
               transfers_in = EXCLUDED.transfers_in,
               transfers_in_event = EXCLUDED.transfers_in_event,
               transfers_out = EXCLUDED.transfers_out,
               transfers_out_event = EXCLUDED.transfers_out_event,
               value_form = EXCLUDED.value_form,
               value_season = EXCLUDED.value_season,
               web_name = EXCLUDED.web_name,
               region = EXCLUDED.region,
               team_join_date = EXCLUDED.team_join_date,
               minutes = EXCLUDED.minutes,
               goals_scored = EXCLUDED.goals_scored,
               assists = EXCLUDED.assists,
               clean_sheets = EXCLUDED.clean_sheets,
               goals_conceded = EXCLUDED.goals_conceded,
               own_goals = EXCLUDED.own_goals,
               penalties_saved = EXCLUDED.penalties_saved,
               penalties_missed = EXCLUDED.penalties_missed,
               yellow_cards = EXCLUDED.yellow_cards,
               red_cards = EXCLUDED.red_cards,
               saves = EXCLUDED.saves,
               bonus = EXCLUDED.bonus,
               bps = EXCLUDED.bps,
               influence = EXCLUDED.influence,
               creativity = EXCLUDED.creativity,
               threat = EXCLUDED.threat,
               ict_index = EXCLUDED.ict_index,
               starts = EXCLUDED.starts,
               expected_goals = EXCLUDED.expected_goals,
               expected_assists = EXCLUDED.expected_assists,
               expected_goal_involvements = EXCLUDED.expected_goal_involvements,
               expected_goals_conceded = EXCLUDED.expected_goals_conceded,
               influence_rank = EXCLUDED.influence_rank,
               influence_rank_type = EXCLUDED.influence_rank_type,
               creativity_rank = EXCLUDED.creativity_rank,
               creativity_rank_type = EXCLUDED.creativity_rank_type,
               threat_rank = EXCLUDED.threat_rank,
               threat_rank_type = EXCLUDED.threat_rank_type,
               ict_index_rank = EXCLUDED.ict_index_rank,
               ict_index_rank_type = EXCLUDED.ict_index_rank_type,
               corners_and_indirect_freekicks_order = EXCLUDED.corners_and_indirect_freekicks_order,
               corners_and_indirect_freekicks_text = EXCLUDED.corners_and_indirect_freekicks_text,
               direct_freekicks_order = EXCLUDED.direct_freekicks_order,
               direct_freekicks_text = EXCLUDED.direct_freekicks_text,
               penalties_order = EXCLUDED.penalties_order,
               penalties_text = EXCLUDED.penalties_text,
               expected_goals_per_90 = EXCLUDED.expected_goals_per_90,
               saves_per_90 = EXCLUDED.saves_per_90,
               expected_assists_per_90 = EXCLUDED.expected_assists_per_90,
               expected_goal_involvements_per_90 = EXCLUDED.expected_goal_involvements_per_90,
               expected_goals_conceded_per_90 = EXCLUDED.expected_goals_conceded_per_90,
               goals_conceded_per_90 = EXCLUDED.goals_conceded_per_90,
               now_cost_rank = EXCLUDED.now_cost_rank,
               now_cost_rank_type = EXCLUDED.now_cost_rank_type,
               form_rank = EXCLUDED.form_rank,
               form_rank_type = EXCLUDED.form_rank_type,
               points_per_game_rank = EXCLUDED.points_per_game_rank,
               points_per_game_rank_type = EXCLUDED.points_per_game_rank_type,
               selected_rank = EXCLUDED.selected_rank,
               selected_rank_type = EXCLUDED.selected_rank_type,
               starts_per_90 = EXCLUDED.starts_per_90,
               clean_sheets_per_90 = EXCLUDED.clean_sheets_per_90
           "#,
           i16::from(player.id),
           player.can_transact,
           player.can_select,
           player.chance_of_playing_next_round,
           player.chance_of_playing_this_round,
           player.code,
           player.cost_change_event,
           player.cost_change_event_fall,
           player.cost_change_start,
           player.cost_change_start_fall,
           player.dreamteam_count,
           player.element_type,
           player.ep_next,
           player.ep_this,
           player.event_points,
           player.first_name,
           player.form,
           player.in_dreamteam,
           player.news,
           player.news_added,
           player.now_cost,
           player.photo,
           player.points_per_game,
           player.removed,
           player.second_name,
           player.selected_by_percent,
           player.special,
           player.squad_number,
           player.status,
           i16::from(player.team),
           player.team_code,
           player.total_points,
           player.transfers_in,
           player.transfers_in_event,
           player.transfers_out,
           player.transfers_out_event,
           player.value_form,
           player.value_season,
           player.web_name,
           player.region,
           player.team_join_date,
           player.minutes,
           player.goals_scored,
           player.assists,
           player.clean_sheets,
           player.goals_conceded,
           player.own_goals,
           player.penalties_saved,
           player.penalties_missed,
           player.yellow_cards,
           player.red_cards,
           player.saves,
           player.bonus,
           player.bps,
           player.influence,
           player.creativity,
           player.threat,
           player.ict_index,
           player.starts,
           player.expected_goals,
           player.expected_assists,
           player.expected_goal_involvements,
           player.expected_goals_conceded,
           player.influence_rank,
           player.influence_rank_type,
           player.creativity_rank,
           player.creativity_rank_type,
           player.threat_rank,
           player.threat_rank_type,
           player.ict_index_rank,
           player.ict_index_rank_type,
           player.corners_and_indirect_freekicks_order,
           player.corners_and_indirect_freekicks_text,
           player.direct_freekicks_order,
           player.direct_freekicks_text,
           player.penalties_order,
           player.penalties_text,
           player.expected_goals_per_90,
           player.saves_per_90,
           player.expected_assists_per_90,
           player.expected_goal_involvements_per_90,
           player.expected_goals_conceded_per_90,
           player.goals_conceded_per_90,
           player.now_cost_rank,
           player.now_cost_rank_type,
           player.form_rank,
           player.form_rank_type,
           player.points_per_game_rank,
           player.points_per_game_rank_type,
           player.selected_rank,
           player.selected_rank_type,
           player.starts_per_90,
           player.clean_sheets_per_90
       )
       .execute(&mut *tx)
       .await?;
    }

    tx.commit().await?;
    debug!("Upsert Completed");
    Ok(())
}

pub async fn upsert_player_fixtures(
    pool: &PgPool,
    player_fixtures: &[PlayerFixtureDb],
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    info!("Upserting {} PlayerFixtureDb rows", player_fixtures.len());

    for player_fixture in player_fixtures {
        sqlx::query!(
            r#"
           INSERT INTO player_fixtures (
           fixture_id, player_id, event_name, is_home, difficulty
           )
           VALUES ($1, $2, $3, $4, $5)
           ON CONFLICT (player_id, fixture_id) DO UPDATE SET
           event_name = EXCLUDED.event_name,
           is_home = EXCLUDED.is_home,
            difficulty = EXCLUDED.difficulty
           "#,
            i16::from(player_fixture.fixture_id),
            i16::from(player_fixture.player_id),
            player_fixture.event_name,
            player_fixture.is_home,
            player_fixture.difficulty
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    debug!("Upsert Completed");
    Ok(())
}

pub async fn upsert_player_history_past(
    pool: &PgPool,
    histories: &[PlayerHistoryPastDb],
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    info!("Upserting {} PlayerHistoryPastDb rows", histories.len());

    for history in histories {
        sqlx::query!(
            r#"
            INSERT INTO player_history_past (
                player_id, season_name, element_code, start_cost, end_cost, total_points,
                minutes, goals_scored, assists, clean_sheets, goals_conceded, own_goals,
                penalties_saved, penalties_missed, yellow_cards, red_cards, saves, bonus,
                bps, influence, creativity, threat, ict_index, starts, expected_goals,
                expected_assists, expected_goal_involvements, expected_goals_conceded
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15,
                    $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28)
            ON CONFLICT (player_id, season_name) DO UPDATE SET
                element_code = EXCLUDED.element_code,
                start_cost = EXCLUDED.start_cost,
                end_cost = EXCLUDED.end_cost,
                total_points = EXCLUDED.total_points,
                minutes = EXCLUDED.minutes,
                goals_scored = EXCLUDED.goals_scored,
                assists = EXCLUDED.assists,
                clean_sheets = EXCLUDED.clean_sheets,
                goals_conceded = EXCLUDED.goals_conceded,
                own_goals = EXCLUDED.own_goals,
                penalties_saved = EXCLUDED.penalties_saved,
                penalties_missed = EXCLUDED.penalties_missed,
                yellow_cards = EXCLUDED.yellow_cards,
                red_cards = EXCLUDED.red_cards,
                saves = EXCLUDED.saves,
                bonus = EXCLUDED.bonus,
                bps = EXCLUDED.bps,
                influence = EXCLUDED.influence,
                creativity = EXCLUDED.creativity,
                threat = EXCLUDED.threat,
                ict_index = EXCLUDED.ict_index,
                starts = EXCLUDED.starts,
                expected_goals = EXCLUDED.expected_goals,
                expected_assists = EXCLUDED.expected_assists,
                expected_goal_involvements = EXCLUDED.expected_goal_involvements,
                expected_goals_conceded = EXCLUDED.expected_goals_conceded
            "#,
            i16::from(history.player_id),
            history.season_name,
            history.element_code,
            history.start_cost,
            history.end_cost,
            history.total_points,
            history.minutes,
            history.goals_scored,
            history.assists,
            history.clean_sheets,
            history.goals_conceded,
            history.own_goals,
            history.penalties_saved,
            history.penalties_missed,
            history.yellow_cards,
            history.red_cards,
            history.saves,
            history.bonus,
            history.bps,
            history.influence,
            history.creativity,
            history.threat,
            history.ict_index,
            history.starts,
            history.expected_goals,
            history.expected_assists,
            history.expected_goal_involvements,
            history.expected_goals_conceded,
        )
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    debug!("Upsert Completed");
    Ok(())
}

pub async fn upsert_player_histories(
    pool: &PgPool,
    histories: &[PlayerHistoryDb],
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    info!("Upserting {} PlayerHistoryDb rows", histories.len());

    for history in histories {
        sqlx::query!(
            r#"
            INSERT INTO player_history (
                player_id, fixture_id, opponent_team, total_points, was_home, kickoff_time,
                team_h_score, team_a_score, round, minutes, goals_scored, assists,
                clean_sheets, goals_conceded, own_goals, penalties_saved, penalties_missed,
                yellow_cards, red_cards, saves, bonus, bps, influence, creativity,
                threat, ict_index, starts, expected_goals, expected_assists,
                expected_goal_involvements, expected_goals_conceded, value,
                transfers_balance, selected, transfers_in, transfers_out
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15,
                    $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28,
                    $29, $30, $31, $32, $33, $34, $35, $36)
            ON CONFLICT (player_id, fixture_id) DO UPDATE SET
                opponent_team = EXCLUDED.opponent_team,
                total_points = EXCLUDED.total_points,
                was_home = EXCLUDED.was_home,
                kickoff_time = EXCLUDED.kickoff_time,
                team_h_score = EXCLUDED.team_h_score,
                team_a_score = EXCLUDED.team_a_score,
                round = EXCLUDED.round,
                minutes = EXCLUDED.minutes,
                goals_scored = EXCLUDED.goals_scored,
                assists = EXCLUDED.assists,
                clean_sheets = EXCLUDED.clean_sheets,
                goals_conceded = EXCLUDED.goals_conceded,
                own_goals = EXCLUDED.own_goals,
                penalties_saved = EXCLUDED.penalties_saved,
                penalties_missed = EXCLUDED.penalties_missed,
                yellow_cards = EXCLUDED.yellow_cards,
                red_cards = EXCLUDED.red_cards,
                saves = EXCLUDED.saves,
                bonus = EXCLUDED.bonus,
                bps = EXCLUDED.bps,
                influence = EXCLUDED.influence,
                creativity = EXCLUDED.creativity,
                threat = EXCLUDED.threat,
                ict_index = EXCLUDED.ict_index,
                starts = EXCLUDED.starts,
                expected_goals = EXCLUDED.expected_goals,
                expected_assists = EXCLUDED.expected_assists,
                expected_goal_involvements = EXCLUDED.expected_goal_involvements,
                expected_goals_conceded = EXCLUDED.expected_goals_conceded,
                value = EXCLUDED.value,
                transfers_balance = EXCLUDED.transfers_balance,
                selected = EXCLUDED.selected,
                transfers_in = EXCLUDED.transfers_in,
                transfers_out = EXCLUDED.transfers_out
            "#,
            i16::from(history.player_id),
            i16::from(history.fixture_id),
            history.opponent_team,
            history.total_points,
            history.was_home,
            history.kickoff_time,
            history.team_h_score,
            history.team_a_score,
            i16::from(history.round),
            history.minutes,
            history.goals_scored,
            history.assists,
            history.clean_sheets,
            history.goals_conceded,
            history.own_goals,
            history.penalties_saved,
            history.penalties_missed,
            history.yellow_cards,
            history.red_cards,
            history.saves,
            history.bonus,
            history.bps,
            history.influence,
            history.creativity,
            history.threat,
            history.ict_index,
            history.starts,
            history.expected_goals,
            history.expected_assists,
            history.expected_goal_involvements,
            history.expected_goals_conceded,
            history.value,
            history.transfers_balance,
            history.selected,
            history.transfers_in,
            history.transfers_out,
        )
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    debug!("Upsert Completed");
    Ok(())
}

pub async fn get_all_player_ids(pool: &PgPool) -> Result<Vec<PlayerId>, sqlx::Error> {
    let ids = sqlx::query!("SELECT id FROM players")
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|row| PlayerId::from(row.id))
        .collect();

    Ok(ids)
}
