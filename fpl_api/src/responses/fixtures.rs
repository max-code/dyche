use chrono::{DateTime, Utc};
use fpl_common::types::{ClubId, FixtureId, GameWeekId};
use serde::Deserialize;

pub type FixturesResponse = Vec<GameweekFixture>;

#[derive(Debug, Deserialize)]
pub struct GameweekFixture {
    #[serde(flatten)]
    pub common: FixtureCommon,
    pub started: Option<bool>,
    pub team_h_difficulty: i16,
    pub team_a_difficulty: i16,
    pub pulse_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct FixtureCommon {
    pub id: FixtureId,
    pub code: i32,
    pub team_h: ClubId,
    pub team_h_score: Option<i16>,
    pub team_a: ClubId,
    pub team_a_score: Option<i16>,
    pub event: Option<GameWeekId>,
    pub finished: bool,
    pub minutes: i16,
    pub provisional_start_time: bool,
    pub kickoff_time: Option<DateTime<Utc>>,
}
