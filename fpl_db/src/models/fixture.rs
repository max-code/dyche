use chrono::{DateTime, Utc};
use fpl_api::responses::fixtures::{GameWeekBonus, GameweekFixture};
use fpl_common::types::{ClubId, FixtureId, GameWeekId, PlayerId};

#[derive(Debug, sqlx::FromRow)]
pub struct Fixture {
    pub id: FixtureId,
    pub code: i32,
    pub game_week_id: Option<GameWeekId>,
    pub home_team_id: ClubId,
    pub away_team_id: ClubId,
    pub home_team_score: Option<i16>,
    pub away_team_score: Option<i16>,
    pub kickoff_time: Option<DateTime<Utc>>,
    pub finished: bool,
    pub started: Option<bool>,
    pub minutes: i16,
    pub provisional_start_time: bool,
    pub team_h_difficulty: i16,
    pub team_a_difficulty: i16,
    pub pulse_id: i32,
}

impl From<&GameweekFixture> for Fixture {
    fn from(fixture: &GameweekFixture) -> Self {
        Self {
            id: fixture.common.id,
            code: fixture.common.code,
            game_week_id: fixture.common.event,
            home_team_id: fixture.common.team_h,
            away_team_id: fixture.common.team_a,
            home_team_score: fixture.common.team_h_score,
            away_team_score: fixture.common.team_a_score,
            kickoff_time: fixture.common.kickoff_time,
            finished: fixture.common.finished,
            started: fixture.started,
            minutes: fixture.common.minutes,
            provisional_start_time: fixture.common.provisional_start_time,
            team_h_difficulty: fixture.team_h_difficulty,
            team_a_difficulty: fixture.team_a_difficulty,
            pulse_id: fixture.pulse_id,
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct Bonus {
    pub fixture_id: FixtureId,
    pub player_id: PlayerId,
    pub bps: i16,
    pub bonus: i16,
}

impl From<&GameWeekBonus> for Bonus {
    fn from(value: &GameWeekBonus) -> Self {
        Self {
            fixture_id: value.fixture_id,
            player_id: value.player_id,
            bps: value.bps,
            bonus: value.bonus,
        }
    }
}
