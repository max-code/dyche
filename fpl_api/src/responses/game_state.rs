use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use super::{string_to_f32, string_to_option_f32};
use fpl_common::types::{Chip, ClubId, GameWeekId, PlayerId};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameStateResponse {
    pub total_players: u32,
    pub teams: Vec<ClubOverview>,
    pub elements: Vec<PlayerOverview>,
    pub events: Vec<GameWeekOverview>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameWeekOverview {
    pub id: GameWeekId,
    pub name: String,
    pub deadline_time: DateTime<Utc>,
    pub release_time: Option<DateTime<Utc>>,
    pub average_entry_score: u8,
    pub finished: bool,
    pub data_checked: bool,
    pub highest_scoring_entry: Option<u32>,
    pub deadline_time_epoch: i64,
    pub deadline_time_game_offset: u32,
    pub highest_score: Option<u8>,
    pub is_previous: bool,
    pub is_current: bool,
    pub is_next: bool,
    pub cup_leagues_created: bool,
    pub h2h_ko_matches_created: bool,
    pub can_enter: bool,
    pub can_manage: bool,
    pub released: bool,
    pub ranked_count: u32,
    pub chip_plays: Vec<ChipPlay>,
    pub most_selected: Option<PlayerId>,
    pub most_transferred_in: Option<PlayerId>,
    pub top_element: Option<PlayerId>,
    pub top_element_info: Option<TopElementInfo>,
    pub transfers_made: u32,
    pub most_captained: Option<PlayerId>,
    pub most_vice_captained: Option<PlayerId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChipPlay {
    pub chip_name: Chip,
    pub num_played: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopElementInfo {
    pub id: PlayerId,
    pub points: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClubOverview {
    pub code: u8,
    pub draw: u8,
    pub form: Option<String>,
    pub id: ClubId,
    pub loss: u8,
    pub name: String,
    pub played: u8,
    pub points: u8,
    pub position: u8,
    pub short_name: String,
    pub strength: u8,
    pub team_division: Option<String>,
    pub unavailable: bool,
    pub win: u8,
    pub strength_overall_home: u16,
    pub strength_overall_away: u16,
    pub strength_attack_home: u16,
    pub strength_attack_away: u16,
    pub strength_defence_home: u16,
    pub strength_defence_away: u16,
    pub pulse_id: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerOverview {
    pub can_transact: bool,
    pub can_select: bool,
    pub chance_of_playing_next_round: Option<u8>,
    pub chance_of_playing_this_round: Option<u8>,
    pub code: u32,
    pub cost_change_event: i8,
    pub cost_change_event_fall: i8,
    pub cost_change_start: i8,
    pub cost_change_start_fall: i8,
    pub dreamteam_count: u8,
    pub element_type: u8,
    #[serde(deserialize_with = "string_to_f32")]
    pub ep_next: f32,
    #[serde(deserialize_with = "string_to_f32")]
    pub ep_this: f32,
    pub event_points: i8,
    pub first_name: String,
    #[serde(deserialize_with = "string_to_option_f32")]
    pub form: Option<f32>,
    pub id: PlayerId,
    pub in_dreamteam: bool,
    pub news: String,
    pub news_added: Option<DateTime<Utc>>,
    pub now_cost: u8,
    pub photo: String,
    #[serde(deserialize_with = "string_to_f32")]
    pub points_per_game: f32,
    pub removed: bool,
    pub second_name: String,
    #[serde(deserialize_with = "string_to_f32")]
    pub selected_by_percent: f32,
    pub special: bool,
    pub squad_number: Option<u8>,
    pub status: String,
    pub team: ClubId,
    pub team_code: u8,
    pub total_points: u8,
    pub transfers_in: u32,
    pub transfers_in_event: u32,
    pub transfers_out: u32,
    pub transfers_out_event: u32,
    #[serde(deserialize_with = "string_to_f32")]
    pub value_form: f32,
    #[serde(deserialize_with = "string_to_f32")]
    pub value_season: f32,
    pub web_name: String,
    pub region: Option<u16>,
    pub team_join_date: Option<NaiveDate>,
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
    pub influence_rank: u16,
    pub influence_rank_type: u16,
    pub creativity_rank: u16,
    pub creativity_rank_type: u16,
    pub threat_rank: u16,
    pub threat_rank_type: u16,
    pub ict_index_rank: u16,
    pub ict_index_rank_type: u16,
    pub corners_and_indirect_freekicks_order: Option<u8>,
    pub corners_and_indirect_freekicks_text: String,
    pub direct_freekicks_order: Option<u8>,
    pub direct_freekicks_text: String,
    pub penalties_order: Option<u8>,
    pub penalties_text: String,
    pub expected_goals_per_90: f32,
    pub saves_per_90: f32,
    pub expected_assists_per_90: f32,
    pub expected_goal_involvements_per_90: f32,
    pub expected_goals_conceded_per_90: f32,
    pub goals_conceded_per_90: f32,
    pub now_cost_rank: u16,
    pub now_cost_rank_type: u16,
    pub form_rank: u16,
    pub form_rank_type: u16,
    pub points_per_game_rank: u16,
    pub points_per_game_rank_type: u16,
    pub selected_rank: u16,
    pub selected_rank_type: u16,
    pub starts_per_90: f32,
    pub clean_sheets_per_90: f32,
}
