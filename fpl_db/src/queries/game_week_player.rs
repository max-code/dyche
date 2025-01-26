use sqlx::PgPool;
use tracing::{debug, info};

use crate::models::game_week_player::GameWeekPlayerDb;
pub async fn upsert_game_week_players(
    pool: &PgPool,
    game_week_players: &[GameWeekPlayerDb],
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    info!(
        "Upserting {} GameWeekPlayerDb rows",
        game_week_players.len()
    );

    for game_week_player in game_week_players {
        sqlx::query!(
            r#"
           INSERT INTO game_week_players (
               player_id, game_week_id, minutes, goals_scored, assists, clean_sheets,
               goals_conceded, own_goals, penalties_saved, penalties_missed, yellow_cards,
               red_cards, saves, bonus, bps, influence, creativity, threat, ict_index,
               starts, expected_goals, expected_assists, expected_goal_involvements,
               expected_goals_conceded, total_points, in_dreamteam
           )
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15,
                  $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26)
           ON CONFLICT (player_id, game_week_id) DO UPDATE SET
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
               total_points = EXCLUDED.total_points,
               in_dreamteam = EXCLUDED.in_dreamteam,
               updated_at = NOW()
           "#,
            i16::from(game_week_player.player_id),
            i16::from(game_week_player.game_week_id),
            game_week_player.minutes,
            game_week_player.goals_scored,
            game_week_player.assists,
            game_week_player.clean_sheets,
            game_week_player.goals_conceded,
            game_week_player.own_goals,
            game_week_player.penalties_saved,
            game_week_player.penalties_missed,
            game_week_player.yellow_cards,
            game_week_player.red_cards,
            game_week_player.saves,
            game_week_player.bonus,
            game_week_player.bps,
            game_week_player.influence,
            game_week_player.creativity,
            game_week_player.threat,
            game_week_player.ict_index,
            game_week_player.starts,
            game_week_player.expected_goals,
            game_week_player.expected_assists,
            game_week_player.expected_goal_involvements,
            game_week_player.expected_goals_conceded,
            game_week_player.total_points,
            game_week_player.in_dreamteam,
        )
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    debug!("Upsert Completed");
    Ok(())
}
