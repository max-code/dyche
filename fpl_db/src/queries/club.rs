use sqlx::PgPool;
use tracing::{debug, info};

use crate::models::club::Club;

pub async fn upsert_clubs(pool: &PgPool, clubs: &[Club]) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    info!("Upserting {} Club rows", clubs.len());

    for club in clubs {
        sqlx::query!(
            r#"
           INSERT INTO clubs (
               id, code, draw, form, loss, name, played, points, position,
               short_name, strength, team_division, unavailable, win,
               strength_overall_home, strength_overall_away,
               strength_attack_home, strength_attack_away,
               strength_defence_home, strength_defence_away, pulse_id
           )
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14,
                   $15, $16, $17, $18, $19, $20, $21)
           ON CONFLICT (id) DO UPDATE SET
               draw = EXCLUDED.draw,
               form = EXCLUDED.form,
               loss = EXCLUDED.loss,
               played = EXCLUDED.played,
               points = EXCLUDED.points,
               position = EXCLUDED.position,
               unavailable = EXCLUDED.unavailable,
               win = EXCLUDED.win
           "#,
            i16::from(club.id),
            club.code,
            club.draw,
            club.form,
            club.loss,
            club.name,
            club.played,
            club.points,
            club.position,
            club.short_name,
            club.strength,
            club.team_division,
            club.unavailable,
            club.win,
            club.strength_overall_home,
            club.strength_overall_away,
            club.strength_attack_home,
            club.strength_attack_away,
            club.strength_defence_home,
            club.strength_defence_away,
            club.pulse_id
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    debug!("Upsert Completed");
    Ok(())
}
