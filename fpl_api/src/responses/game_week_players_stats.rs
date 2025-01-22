use crate::types::{FixtureId, PlayerId};

use super::string_to_f32;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GameWeekPlayersStatsResponse {
    pub elements: Vec<GameWeekPlayer>,
}

#[derive(Debug, Deserialize)]
pub struct GameWeekPlayer {
    pub id: PlayerId,
    pub stats: GameWeekPlayerStats,
    pub explain: Vec<GameWeekPlayerExplain>,
    pub modified: bool,
}

#[derive(Debug, Deserialize)]
pub struct GameWeekPlayerStats {
    pub minutes: u8,
    pub goals_scored: u8,
    pub assists: u8,
    pub clean_sheets: u8,
    pub goals_conceded: u8,
    pub own_goals: u8,
    pub penalties_saved: u8,
    pub penalties_missed: u8,
    pub yellow_cards: u8,
    pub red_cards: u8,
    pub saves: u8,
    pub bonus: u8,
    pub bps: i16,
    #[serde(deserialize_with = "string_to_f32")]
    pub influence: f32,
    #[serde(deserialize_with = "string_to_f32")]
    pub creativity: f32,
    #[serde(deserialize_with = "string_to_f32")]
    pub threat: f32,
    #[serde(deserialize_with = "string_to_f32")]
    pub ict_index: f32,
    pub starts: u8,
    #[serde(deserialize_with = "string_to_f32")]
    pub expected_goals: f32,
    #[serde(deserialize_with = "string_to_f32")]
    pub expected_assists: f32,
    #[serde(deserialize_with = "string_to_f32")]
    pub expected_goal_involvements: f32,
    #[serde(deserialize_with = "string_to_f32")]
    pub expected_goals_conceded: f32,
    pub total_points: i8,
    pub in_dreamteam: bool,
}

#[derive(Debug, Deserialize)]
pub struct GameWeekPlayerExplain {
    pub fixture: FixtureId,
    pub stats: Vec<GameWeekPlayerExplainStat>,
}

#[derive(Debug, Deserialize)]
pub struct GameWeekPlayerExplainStat {
    pub identifier: String,
    pub points: i8,
    pub value: f32,
    pub points_modification: i32,
}
