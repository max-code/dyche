use chrono::{DateTime, Utc};
use fpl_api::responses::fixtures::GameweekFixture;
use fpl_common::types::{ClubId, FixtureId, GameWeekId};

#[derive(Debug, sqlx::FromRow)]
pub struct Fixture {
    pub id: FixtureId,
    pub code: i32,
    pub game_week_id: GameWeekId,
    pub home_team_id: ClubId,
    pub away_team_id: ClubId,
    pub home_team_score: Option<i16>,
    pub away_team_score: Option<i16>,
    pub kickoff_time: DateTime<Utc>,
    pub finished: bool,
    pub started: bool,
    pub minutes: i16,
    pub provisional_start_time: bool,
    pub team_h_difficulty: i16,
    pub team_a_difficulty: i16,
    pub pulse_id: i32,
}

impl TryFrom<GameweekFixture> for Fixture {
    type Error = anyhow::Error;

    fn try_from(fixture: GameweekFixture) -> Result<Self, Self::Error> {
        Ok(Self {
            id: fixture.common.id,
            code: fixture.common.code as i32,
            game_week_id: fixture.common.event,
            home_team_id: fixture.common.team_h,
            away_team_id: fixture.common.team_a,
            home_team_score: fixture.common.team_h_score.map(|s| s as i16),
            away_team_score: fixture.common.team_a_score.map(|s| s as i16),
            kickoff_time: fixture.common.kickoff_time,
            finished: fixture.common.finished,
            started: fixture.started,
            minutes: fixture.common.minutes as i16,
            provisional_start_time: fixture.common.provisional_start_time,
            team_h_difficulty: fixture.team_h_difficulty as i16,
            team_a_difficulty: fixture.team_a_difficulty as i16,
            pulse_id: fixture.pulse_id as i32,
        })
    }
}
