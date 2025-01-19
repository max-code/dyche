use super::string_to_f32;
use crate::types::GameWeek;
use crate::{responses::fixtures::PlayerFixture, types::PlayerId};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PlayerResponse {
    pub fixtures: Vec<PlayerFixture>,
    pub history: Vec<PlayerHistory>,
    pub history_past: Vec<PlayerHistoryPast>,
}

#[derive(Debug, Deserialize)]
pub struct PlayerHistory {
    pub element: PlayerId,
    pub fixture: u16,
    pub opponent_team: u8,
    pub total_points: u8,
    pub was_home: bool,
    pub kickoff_time: DateTime<Utc>,
    pub team_h_score: Option<u8>,
    pub team_a_score: Option<u8>,
    pub round: GameWeek,
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
    pub bps: i8,
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
    pub value: u16,
    pub transfers_balance: i32,
    pub selected: u32,
    pub transfers_in: u32,
    pub transfers_out: u32,
}

#[derive(Debug, Deserialize)]
pub struct PlayerHistoryPast {
    pub season_name: String,
    pub element_code: u32,
    pub start_cost: u8,
    pub end_cost: u8,
    pub total_points: u16,
    pub minutes: u16,
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
}
