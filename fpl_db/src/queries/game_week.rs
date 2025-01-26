use sqlx::PgPool;
use tracing::{debug, info};

use crate::models::game_week::{GameWeek, GameWeekChipPlay, GameWeekTopElement};

pub async fn upsert_game_weeks(pool: &PgPool, game_weeks: &[GameWeek]) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    info!("Upserting {} GameWeek rows", game_weeks.len());

    for game_week in game_weeks {
        sqlx::query!(
            r#"
           INSERT INTO game_weeks (
               id, name, deadline_time, release_time, average_entry_score,
               finished, data_checked, highest_scoring_entry, deadline_time_epoch,
               deadline_time_game_offset, highest_score, is_previous, is_current,
               is_next, cup_leagues_created, h2h_ko_matches_created, can_enter,
               can_manage, released, ranked_count, transfers_made, most_selected,
               most_transferred_in, top_element, most_captained, most_vice_captained
           )
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13,
                   $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26)
           ON CONFLICT (id) DO UPDATE SET
               finished = EXCLUDED.finished,
               data_checked = EXCLUDED.data_checked,
               highest_scoring_entry = EXCLUDED.highest_scoring_entry,
               highest_score = EXCLUDED.highest_score,
               is_previous = EXCLUDED.is_previous,
               is_current = EXCLUDED.is_current,
               is_next = EXCLUDED.is_next,
               released = EXCLUDED.released,
               ranked_count = EXCLUDED.ranked_count,
               transfers_made = EXCLUDED.transfers_made
           "#,
            i16::from(game_week.id),
            game_week.name,
            game_week.deadline_time,
            game_week.release_time,
            game_week.average_entry_score,
            game_week.finished,
            game_week.data_checked,
            game_week.highest_scoring_entry,
            game_week.deadline_time_epoch,
            game_week.deadline_time_game_offset,
            game_week.highest_score,
            game_week.is_previous,
            game_week.is_current,
            game_week.is_next,
            game_week.cup_leagues_created,
            game_week.h2h_ko_matches_created,
            game_week.can_enter,
            game_week.can_manage,
            game_week.released,
            game_week.ranked_count,
            game_week.transfers_made,
            game_week.most_selected.map(i16::from),
            game_week.most_transferred_in.map(i16::from),
            game_week.top_element.map(i16::from),
            game_week.most_captained.map(i16::from),
            game_week.most_vice_captained.map(i16::from)
        )
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    debug!("Upsert Completed");
    Ok(())
}

pub async fn upsert_game_week_chip_plays(
    pool: &PgPool,
    chip_plays: &[GameWeekChipPlay],
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    info!("Upserting {} GameWeekChipPlay rows", chip_plays.len());

    for chip_play in chip_plays {
        sqlx::query!(
            r#"
            INSERT INTO game_week_chip_plays (
                game_week_id, chip_name, num_played
            )
            VALUES (
                $1, $2, $3
            )
            ON CONFLICT (game_week_id, chip_name) DO UPDATE SET
                num_played = EXCLUDED.num_played
            "#,
            i16::from(chip_play.game_week_id),
            chip_play.chip_name,
            chip_play.num_played
        )
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    debug!("Upsert Completed");
    Ok(())
}

pub async fn upsert_game_week_top_elements(
    pool: &PgPool,
    top_elements: &Vec<GameWeekTopElement>,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    info!("Upserting {} GameWeekTopElement rows", top_elements.len());

    for top_element in top_elements {
        sqlx::query!(
            r#"
            INSERT INTO game_week_top_elements (
                game_week_id, player_id, points
            )
            VALUES (
                $1, $2, $3
            )
            ON CONFLICT (game_week_id) DO UPDATE SET
                player_id = EXCLUDED.player_id,
                points = EXCLUDED.points
            "#,
            i16::from(top_element.game_week_id),
            i16::from(top_element.player_id),
            top_element.points
        )
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    debug!("Upsert Completed");
    Ok(())
}
