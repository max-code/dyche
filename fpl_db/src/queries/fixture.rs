use fpl_common::types::{ClubId, FixtureId, GameWeekId};
use sqlx::PgPool;
use tracing::debug;

use crate::models::{fixture::Fixture, Bonus};

pub async fn upsert_fixtures(pool: &PgPool, fixtures: &[Fixture]) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    debug!("Upserting {} Fixture rows", fixtures.len());
    for fixture in fixtures {
        sqlx::query!(
            r#"
           INSERT INTO fixtures (
               id, code, game_week_id, home_team_id, away_team_id,
               home_team_score, away_team_score, kickoff_time, finished,
               started, minutes, provisional_start_time,
               team_h_difficulty, team_a_difficulty, pulse_id
           )
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
           ON CONFLICT (id) DO UPDATE SET
               home_team_score = EXCLUDED.home_team_score,
               away_team_score = EXCLUDED.away_team_score,
               finished = EXCLUDED.finished,
               started = EXCLUDED.started,
               minutes = EXCLUDED.minutes
           "#,
            i16::from(fixture.id),
            fixture.code,
            fixture
                .game_week_id
                .map(|gw: GameWeekId| -> i16 { gw.into() }),
            i16::from(fixture.home_team_id),
            i16::from(fixture.away_team_id),
            fixture.home_team_score,
            fixture.away_team_score,
            fixture.kickoff_time,
            fixture.finished,
            fixture.started,
            fixture.minutes,
            fixture.provisional_start_time,
            fixture.team_h_difficulty,
            fixture.team_a_difficulty,
            fixture.pulse_id
        )
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    debug!("Upsert Completed");
    Ok(())
}

pub async fn upsert_bonuses(pool: &PgPool, bonuses: &[Bonus]) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    debug!("Upserting {} Bonus rows", bonuses.len());
    for bonus in bonuses {
        sqlx::query!(
            r#"
           INSERT INTO bonus (
               fixture_id, player_id, bps, bonus
           )
           VALUES ($1, $2, $3, $4)
           ON CONFLICT (fixture_id, player_id) DO UPDATE SET
               bps = EXCLUDED.bps,
               bonus = EXCLUDED.bonus
           "#,
            i16::from(bonus.fixture_id),
            i16::from(bonus.player_id),
            bonus.bps,
            bonus.bonus
        )
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    debug!("Upsert Completed");
    Ok(())
}
