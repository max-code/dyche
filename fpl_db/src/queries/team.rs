use sqlx::PgPool;

use crate::models::team::Team;

pub async fn upsert_teams(pool: &PgPool, teams: &[Team]) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    for team in teams {
        sqlx::query!(
            r#"
            INSERT INTO teams (
                id, joined_time, started_event, favourite_team, player_first_name,
                player_last_name, player_region_id, player_region_name,
                player_region_iso_code_short, player_region_iso_code_long,
                summary_overall_points, summary_overall_rank, summary_event_points,
                summary_event_rank, current_event, name, name_change_blocked,
                last_deadline_bank, last_deadline_value, last_deadline_total_transfers
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13,
                    $14, $15, $16, $17, $18, $19, $20)
            ON CONFLICT (id) DO UPDATE SET
                summary_overall_points = EXCLUDED.summary_overall_points,
                summary_overall_rank = EXCLUDED.summary_overall_rank,
                summary_event_points = EXCLUDED.summary_event_points,
                summary_event_rank = EXCLUDED.summary_event_rank,
                current_event = EXCLUDED.current_event,
                name_change_blocked = EXCLUDED.name_change_blocked,
                last_deadline_bank = EXCLUDED.last_deadline_bank,
                last_deadline_value = EXCLUDED.last_deadline_value,
                last_deadline_total_transfers = EXCLUDED.last_deadline_total_transfers
            "#,
            i32::from(team.id),
            team.joined_time,
            i16::from(team.started_event),
            team.favourite_team.map(i16::from),
            team.player_first_name,
            team.player_last_name,
            team.player_region_id,
            team.player_region_name,
            team.player_region_iso_code_short,
            team.player_region_iso_code_long,
            team.summary_overall_points,
            team.summary_overall_rank,
            team.summary_event_points,
            team.summary_event_rank,
            team.current_event,
            team.name,
            team.name_change_blocked,
            team.last_deadline_bank,
            team.last_deadline_value,
            team.last_deadline_total_transfers
        )
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    Ok(())
}
