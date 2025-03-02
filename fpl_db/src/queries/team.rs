use sqlx::PgPool;
use tracing::debug;

use fpl_common::types::TeamId;

use crate::models::team::Team;

pub async fn upsert_teams(pool: &PgPool, teams: &[Team]) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    debug!("Upserting {} Team rows", teams.len());

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
    debug!("Upsert Completed");
    Ok(())
}

pub async fn get_all_team_ids(pool: &PgPool) -> Result<Vec<TeamId>, sqlx::Error> {
    let ids = sqlx::query!("SELECT id FROM teams")
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|row| TeamId::from(row.id))
        .collect();

    Ok(ids)
}

pub async fn get_team_name_from_discord_id(
    pool: &PgPool,
    discord_id: i64,
) -> Result<String, sqlx::Error> {
    let record = sqlx::query!(
        "SELECT t.name, t.player_first_name, t.player_last_name 
         FROM discord_users du 
         JOIN teams t ON du.team_id = t.id 
         WHERE du.discord_id = $1",
        discord_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(record
        .map(|row| {
            format!(
                "{} ({} {})",
                row.name, row.player_first_name, row.player_last_name
            )
        })
        .unwrap_or_else(|| "N/A".to_string()))
}
