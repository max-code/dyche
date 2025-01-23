use chrono::{DateTime, Utc};
use fpl_common::types::{ClubId, FixtureId, GameWeekId};
use serde::Deserialize;

pub type FixturesResponse = Vec<GameweekFixture>;

#[derive(Debug, Deserialize)]
pub struct GameweekFixture {
    #[serde(flatten)]
    pub common: FixtureCommon,
    pub started: bool,
    pub team_h_difficulty: u8,
    pub team_a_difficulty: u8,
    pub pulse_id: u32,
}

#[derive(Debug, Deserialize)]
pub struct FixtureCommon {
    pub id: FixtureId,
    pub code: u32,
    pub team_h: ClubId,
    pub team_h_score: Option<u8>,
    pub team_a: ClubId,
    pub team_a_score: Option<u8>,
    pub event: GameWeekId,
    pub finished: bool,
    pub minutes: u8,
    pub provisional_start_time: bool,
    pub kickoff_time: DateTime<Utc>,
}
