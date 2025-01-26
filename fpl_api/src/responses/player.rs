use super::string_to_f32;
use crate::responses::fixtures::FixtureCommon;
use chrono::{DateTime, Utc};
use fpl_common::types::{FixtureId, GameWeekId, PlayerId};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PlayerResponse {
    pub fixtures: Vec<PlayerFixture>,
    pub history: Vec<PlayerHistory>,
    pub history_past: Vec<PlayerHistoryPast>,
    pub player_id: Option<PlayerId>,
}

#[derive(Debug, Deserialize)]
pub struct PlayerHistory {
    pub element: PlayerId,
    pub fixture: FixtureId,
    pub opponent_team: i16,
    pub total_points: i16,
    pub was_home: bool,
    pub kickoff_time: DateTime<Utc>,
    pub team_h_score: Option<i16>,
    pub team_a_score: Option<i16>,
    pub round: GameWeekId,
    pub minutes: i16,
    pub goals_scored: i16,
    pub assists: i16,
    pub clean_sheets: i16,
    pub goals_conceded: i16,
    pub own_goals: i16,
    pub penalties_saved: i16,
    pub penalties_missed: i16,
    pub yellow_cards: i16,
    pub red_cards: i16,
    pub saves: i16,
    pub bonus: i16,
    pub bps: i16,
    #[serde(deserialize_with = "string_to_f32")]
    pub influence: f32,
    #[serde(deserialize_with = "string_to_f32")]
    pub creativity: f32,
    #[serde(deserialize_with = "string_to_f32")]
    pub threat: f32,
    #[serde(deserialize_with = "string_to_f32")]
    pub ict_index: f32,
    pub starts: i16,
    #[serde(deserialize_with = "string_to_f32")]
    pub expected_goals: f32,
    #[serde(deserialize_with = "string_to_f32")]
    pub expected_assists: f32,
    #[serde(deserialize_with = "string_to_f32")]
    pub expected_goal_involvements: f32,
    #[serde(deserialize_with = "string_to_f32")]
    pub expected_goals_conceded: f32,
    pub value: i16,
    pub transfers_balance: i32,
    pub selected: i32,
    pub transfers_in: i32,
    pub transfers_out: i32,
}

#[derive(Debug, Deserialize)]
pub struct PlayerHistoryPast {
    pub season_name: String,
    pub element_code: i32,
    pub start_cost: i16,
    pub end_cost: i16,
    pub total_points: i16,
    pub minutes: i16,
    pub goals_scored: i16,
    pub assists: i16,
    pub clean_sheets: i16,
    pub goals_conceded: i16,
    pub own_goals: i16,
    pub penalties_saved: i16,
    pub penalties_missed: i16,
    pub yellow_cards: i16,
    pub red_cards: i16,
    pub saves: i16,
    pub bonus: i16,
    pub bps: i16,
    #[serde(deserialize_with = "string_to_f32")]
    pub influence: f32,
    #[serde(deserialize_with = "string_to_f32")]
    pub creativity: f32,
    #[serde(deserialize_with = "string_to_f32")]
    pub threat: f32,
    #[serde(deserialize_with = "string_to_f32")]
    pub ict_index: f32,
    pub starts: i16,
    #[serde(deserialize_with = "string_to_f32")]
    pub expected_goals: f32,
    #[serde(deserialize_with = "string_to_f32")]
    pub expected_assists: f32,
    #[serde(deserialize_with = "string_to_f32")]
    pub expected_goal_involvements: f32,
    #[serde(deserialize_with = "string_to_f32")]
    pub expected_goals_conceded: f32,
}

#[derive(Debug, Deserialize)]
pub struct PlayerFixture {
    #[serde(flatten)]
    pub common: FixtureCommon,
    pub event_name: String,
    pub is_home: bool,
    pub difficulty: i16,
}
