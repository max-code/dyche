use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::models::fixtures::FixturesRow;

pub async fn upsert_fixtures(
    pool: &PgPool,
    fixtures: &Vec<FixturesRow>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO fixtures (
            id, code, game_week_id, home_team_id, away_team_id,
            home_team_score, away_team_score, kickoff_time, finished,
            started, minutes, provisional_start_time,
            team_h_difficulty, team_a_difficulty, pulse_id
        )
        SELECT * FROM UNNEST(
            $1::int[], $2::int[], $3::int[], $4::int[], $5::int[],
            $6::smallint[], $7::smallint[], $8::timestamptz[], $9::bool[],
            $10::bool[], $11::smallint[], $12::bool[],
            $13::smallint[], $14::smallint[], $15::int[]
        )
        ON CONFLICT (id) DO UPDATE SET
            home_team_score = EXCLUDED.home_team_score,
            away_team_score = EXCLUDED.away_team_score,
            finished = EXCLUDED.finished,
            started = EXCLUDED.started,
            minutes = EXCLUDED.minutes
        "#,
        field_to_vec!(fixtures, id),
        field_to_vec!(fixtures, code),
        field_to_vec!(fixtures, game_week_id),
        field_to_vec!(fixtures, home_team_id),
        field_to_vec!(fixtures, away_team_id),
        option_field_to_vec!(fixtures, home_team_score),
        option_field_to_vec!(fixtures, away_team_score),
        field_to_vec_with_type!(fixtures, kickoff_time, DateTime<Utc>),
        field_to_vec!(fixtures, finished),
        field_to_vec!(fixtures, started),
        field_to_vec!(fixtures, minutes),
        field_to_vec!(fixtures, provisional_start_time),
        field_to_vec!(fixtures, team_h_difficulty),
        field_to_vec!(fixtures, team_a_difficulty),
        field_to_vec!(fixtures, pulse_id)
    )
    .execute(pool)
    .await?;

    Ok(())
}
