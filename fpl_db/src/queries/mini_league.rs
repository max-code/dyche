use fpl_common::types::LeagueId;
use sqlx::PgPool;
use tracing::debug;

use crate::models::mini_league::{MiniLeague, MiniLeagueStanding};

pub async fn upsert_mini_leagues(pool: &PgPool, leagues: &[MiniLeague]) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    debug!("Upserting {} MiniLeague rows", leagues.len());

    for league in leagues {
        sqlx::query!(
            r#"
           INSERT INTO mini_leagues (
               id, last_updated_data, name, created, closed, max_entries,
               league_type, scoring, admin_entry, start_event, code_privacy,
               has_cup, cup_league, rank
           )
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
           ON CONFLICT (id) DO UPDATE SET
               last_updated_data = EXCLUDED.last_updated_data,
               closed = EXCLUDED.closed,
               max_entries = EXCLUDED.max_entries,
               rank = EXCLUDED.rank
           "#,
            i32::from(league.id),
            league.last_updated_data,
            league.name,
            league.created,
            league.closed,
            league.max_entries,
            league.league_type,
            league.scoring,
            i32::from(league.admin_entry),
            i16::from(league.start_event),
            league.code_privacy,
            league.has_cup,
            league.cup_league,
            league.rank
        )
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    debug!("Upsert Completed");
    Ok(())
}

pub async fn upsert_mini_league_standings(
    pool: &PgPool,
    standings: &[MiniLeagueStanding],
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    debug!("Upserting {} MiniLeagueStanding rows", standings.len());

    for standing in standings {
        sqlx::query!(
            r#"
           INSERT INTO mini_league_standings (
               id, event_total, player_name, rank, last_rank, rank_sort,
               total, team_id, entry_name, has_player, league_id
           )
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
           ON CONFLICT (league_id, team_id) DO UPDATE SET
               event_total = EXCLUDED.event_total,
               rank = EXCLUDED.rank,
               last_rank = EXCLUDED.last_rank,
               rank_sort = EXCLUDED.rank_sort,
               total = EXCLUDED.total,
               has_player = EXCLUDED.has_player
           "#,
            standing.id,
            standing.event_total,
            standing.player_name,
            standing.rank,
            standing.last_rank,
            standing.rank_sort,
            standing.total,
            i32::from(standing.team_id),
            standing.entry_name,
            standing.has_player,
            i32::from(standing.league_id)
        )
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    debug!("Upsert Completed");
    Ok(())
}

pub async fn get_all_mini_league_ids(pool: &PgPool) -> Result<Vec<LeagueId>, sqlx::Error> {
    let ids = sqlx::query!("SELECT id FROM mini_leagues")
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|row| LeagueId::from(row.id))
        .collect();

    Ok(ids)
}
