use crate::models::{
    team_game_week::{TeamGameWeek, TeamGameWeekPick},
    TeamGameWeekAutomaticSub,
};
use sqlx::PgPool;
use tracing::debug;

pub async fn upsert_team_game_week(
    pool: &PgPool,
    team_game_week: &TeamGameWeek,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    debug!("Upserting TeamGameWeek row");

    sqlx::query!(
        r#"
       INSERT INTO team_game_weeks (
           team_id, game_week_id, active_chip, points, total_points, rank,
           rank_sort, overall_rank, percentile_rank, bank, value,
           event_transfers, event_transfers_cost, points_on_bench
       )
       VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
       ON CONFLICT (team_id, game_week_id) DO UPDATE SET
           active_chip = EXCLUDED.active_chip,
           points = EXCLUDED.points,
           total_points = EXCLUDED.total_points,
           rank = EXCLUDED.rank,
           rank_sort = EXCLUDED.rank_sort,
           overall_rank = EXCLUDED.overall_rank,
           percentile_rank = EXCLUDED.percentile_rank,
           bank = EXCLUDED.bank,
           value = EXCLUDED.value,
           event_transfers = EXCLUDED.event_transfers,
           event_transfers_cost = EXCLUDED.event_transfers_cost,
           points_on_bench = EXCLUDED.points_on_bench
       "#,
        i32::from(team_game_week.team_id),
        i16::from(team_game_week.game_week_id),
        team_game_week.active_chip,
        team_game_week.points,
        team_game_week.total_points,
        team_game_week.rank,
        team_game_week.rank_sort,
        team_game_week.overall_rank,
        team_game_week.percentile_rank,
        team_game_week.bank,
        team_game_week.value,
        team_game_week.event_transfers,
        team_game_week.event_transfers_cost,
        team_game_week.points_on_bench,
    )
    .execute(&mut *tx)
    .await?;

    debug!("Upsert Completed");
    tx.commit().await?;
    Ok(())
}

pub async fn upsert_team_game_weeks(
    pool: &PgPool,
    team_game_weeks: &[TeamGameWeek],
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    debug!("Upserting TeamGameWeek rowr");

    for row in team_game_weeks {
        sqlx::query!(
            r#"
        INSERT INTO team_game_weeks (
            team_id, game_week_id, active_chip, points, total_points, rank,
            rank_sort, overall_rank, percentile_rank, bank, value,
            event_transfers, event_transfers_cost, points_on_bench
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
        ON CONFLICT (team_id, game_week_id) DO UPDATE SET
            active_chip = EXCLUDED.active_chip,
            points = EXCLUDED.points,
            total_points = EXCLUDED.total_points,
            rank = EXCLUDED.rank,
            rank_sort = EXCLUDED.rank_sort,
            overall_rank = EXCLUDED.overall_rank,
            percentile_rank = EXCLUDED.percentile_rank,
            bank = EXCLUDED.bank,
            value = EXCLUDED.value,
            event_transfers = EXCLUDED.event_transfers,
            event_transfers_cost = EXCLUDED.event_transfers_cost,
            points_on_bench = EXCLUDED.points_on_bench
        "#,
            i32::from(row.team_id),
            i16::from(row.game_week_id),
            row.active_chip,
            row.points,
            row.total_points,
            row.rank,
            row.rank_sort,
            row.overall_rank,
            row.percentile_rank,
            row.bank,
            row.value,
            row.event_transfers,
            row.event_transfers_cost,
            row.points_on_bench,
        )
        .execute(&mut *tx)
        .await?;
    }
    debug!("Upsert Completed");
    tx.commit().await?;
    Ok(())
}

pub async fn upsert_team_game_week_picks(
    pool: &PgPool,
    picks: &[TeamGameWeekPick],
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    debug!("Upserting {} TeamGameWeekPick rows", picks.len());

    for pick in picks {
        sqlx::query!(
            r#"
           INSERT INTO team_game_week_picks (
               team_id, game_week_id, player_id, position, multiplier,
               is_captain, is_vice_captain, element_type
           )
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
           ON CONFLICT (team_id, game_week_id, player_id) DO UPDATE SET
               position = EXCLUDED.position,
               multiplier = EXCLUDED.multiplier,
               is_captain = EXCLUDED.is_captain,
               is_vice_captain = EXCLUDED.is_vice_captain,
               element_type = EXCLUDED.element_type
           "#,
            i32::from(pick.team_id),
            i16::from(pick.game_week_id),
            i16::from(pick.player_id),
            pick.position,
            pick.multiplier,
            pick.is_captain,
            pick.is_vice_captain,
            pick.element_type,
        )
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    debug!("Upsert Completed");
    Ok(())
}

pub async fn upsert_team_game_week_automatic_subs(
    pool: &PgPool,
    subs: &[TeamGameWeekAutomaticSub],
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    debug!("Upserting {} TeamGameWeekAutomaticSub rows", subs.len());

    for sub in subs {
        sqlx::query!(
            r#"
           INSERT INTO team_game_week_automatic_subs (
               team_id, game_week_id, player_in_id, player_out_id
           )
           VALUES ($1, $2, $3, $4)
           ON CONFLICT (team_id, game_week_id, player_in_id, player_out_id) DO NOTHING
           "#,
            i32::from(sub.team_id),
            i16::from(sub.game_week_id),
            i16::from(sub.player_in_id),
            i16::from(sub.player_out_id),
        )
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    debug!("Upsert Completed");
    Ok(())
}
