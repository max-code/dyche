use sqlx::PgPool;
use tracing::debug;

use crate::models::transfers::Transfer;

pub async fn upsert_transfers(pool: &PgPool, transfers: &[Transfer]) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    debug!("Upserting {} Transfer rows", transfers.len());

    for transfer in transfers {
        sqlx::query!(
            r#"
           INSERT INTO transfers (
               player_in_id, player_out_id, player_in_cost, player_out_cost,
               team_id, game_week_id, transfer_time
           )
           VALUES ($1, $2, $3, $4, $5, $6, $7)
           ON CONFLICT (team_id, game_week_id, player_in_id, player_out_id, transfer_time) DO UPDATE SET
               player_in_cost = EXCLUDED.player_in_cost,
               player_out_cost = EXCLUDED.player_out_cost
           "#,
            i16::from(transfer.player_in_id),
            i16::from(transfer.player_out_id),
            transfer.player_in_cost,
            transfer.player_out_cost,
            i32::from(transfer.team_id),
            i16::from(transfer.game_week_id),
            transfer.transfer_time,
        )
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    debug!("Upsert Completed");
    Ok(())
}
