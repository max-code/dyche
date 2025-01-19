use crate::types::GameWeek;
use chrono::{DateTime, Utc};
use serde::Deserialize;

// #[derive(Debug, Deserialize)]
// pub struct FixturesResponse {
//     pub fixtures: Vec<Fixture>,
// }

pub type FixturesResponse = Vec<GameweekFixture>;

#[derive(Debug, Deserialize)]
pub struct FixtureCommon {
    pub id: i32,
    pub code: i32,
    pub team_h: i32,
    pub team_h_score: Option<i32>,
    pub team_a: i32,
    pub team_a_score: Option<i32>,
    pub event: GameWeek,
    pub finished: bool,
    pub minutes: i32,
    pub provisional_start_time: bool,
    pub kickoff_time: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct PlayerFixture {
    #[serde(flatten)]
    pub common: FixtureCommon,
    pub event_name: String,
    pub is_home: bool,
    pub difficulty: i32,
}

#[derive(Debug, Deserialize)]
pub struct GameweekFixture {
    #[serde(flatten)]
    pub common: FixtureCommon,
    pub started: bool,
    pub team_h_difficulty: i32,
    pub team_a_difficulty: i32,
    pub pulse_id: i32,
    // We can add stats later if needed
    // pub stats: Vec<FixtureStat>,
}
