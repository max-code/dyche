use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use super::{string_to_f32, string_to_option_f32};
use fpl_common::types::{Chip, ClubId, GameWeekId, PlayerId};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameStateResponse {
    pub total_players: i32,
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
    pub average_entry_score: i16,
    pub finished: bool,
    pub data_checked: bool,
    pub highest_scoring_entry: Option<i32>,
    pub deadline_time_epoch: i64,
    pub deadline_time_game_offset: i32,
    pub highest_score: Option<i16>,
    pub is_previous: bool,
    pub is_current: bool,
    pub is_next: bool,
    pub cup_leagues_created: bool,
    pub h2h_ko_matches_created: bool,
    pub can_enter: bool,
    pub can_manage: bool,
    pub released: bool,
    pub ranked_count: i32,
    pub chip_plays: Vec<ChipPlay>,
    pub most_selected: Option<PlayerId>,
    pub most_transferred_in: Option<PlayerId>,
    pub top_element: Option<PlayerId>,
    pub top_element_info: Option<TopElementInfo>,
    pub transfers_made: i32,
    pub most_captained: Option<PlayerId>,
    pub most_vice_captained: Option<PlayerId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChipPlay {
    pub chip_name: Chip,
    pub num_played: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopElementInfo {
    pub id: PlayerId,
    pub points: i16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClubOverview {
    pub code: i16,
    pub draw: i16,
    pub form: Option<String>,
    pub id: ClubId,
    pub loss: i16,
    pub name: String,
    pub played: i16,
    pub points: i16,
    pub position: i16,
    pub short_name: String,
    pub strength: i16,
    pub team_division: Option<String>,
    pub unavailable: bool,
    pub win: i16,
    pub strength_overall_home: i16,
    pub strength_overall_away: i16,
    pub strength_attack_home: i16,
    pub strength_attack_away: i16,
    pub strength_defence_home: i16,
    pub strength_defence_away: i16,
    pub pulse_id: i16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerOverview {
    pub can_transact: bool,
    pub can_select: bool,
    pub chance_of_playing_next_round: Option<i16>,
    pub chance_of_playing_this_round: Option<i16>,
    pub code: i32,
    pub cost_change_event: i16,
    pub cost_change_event_fall: i16,
    pub cost_change_start: i16,
    pub cost_change_start_fall: i16,
    pub dreamteam_count: i16,
    pub element_type: i16,
    #[serde(deserialize_with = "string_to_f32")]
    pub ep_next: f32,
    #[serde(deserialize_with = "string_to_f32")]
    pub ep_this: f32,
    pub event_points: i16,
    pub first_name: String,
    #[serde(deserialize_with = "string_to_option_f32")]
    pub form: Option<f32>,
    pub id: PlayerId,
    pub in_dreamteam: bool,
    pub news: String,
    pub news_added: Option<DateTime<Utc>>,
    pub now_cost: i16,
    pub photo: String,
    #[serde(deserialize_with = "string_to_f32")]
    pub points_per_game: f32,
    pub removed: bool,
    pub second_name: String,
    #[serde(deserialize_with = "string_to_f32")]
    pub selected_by_percent: f32,
    pub special: bool,
    pub squad_number: Option<i16>,
    pub status: String,
    pub team: ClubId,
    pub team_code: i16,
    pub total_points: i16,
    pub transfers_in: i32,
    pub transfers_in_event: i32,
    pub transfers_out: i32,
    pub transfers_out_event: i32,
    #[serde(deserialize_with = "string_to_f32")]
    pub value_form: f32,
    #[serde(deserialize_with = "string_to_f32")]
    pub value_season: f32,
    pub web_name: String,
    pub region: Option<i16>,
    pub team_join_date: Option<NaiveDate>,
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
    pub influence_rank: i16,
    pub influence_rank_type: i16,
    pub creativity_rank: i16,
    pub creativity_rank_type: i16,
    pub threat_rank: i16,
    pub threat_rank_type: i16,
    pub ict_index_rank: i16,
    pub ict_index_rank_type: i16,
    pub corners_and_indirect_freekicks_order: Option<i16>,
    pub corners_and_indirect_freekicks_text: String,
    pub direct_freekicks_order: Option<i16>,
    pub direct_freekicks_text: String,
    pub penalties_order: Option<i16>,
    pub penalties_text: String,
    pub expected_goals_per_90: f32,
    pub saves_per_90: f32,
    pub expected_assists_per_90: f32,
    pub expected_goal_involvements_per_90: f32,
    pub expected_goals_conceded_per_90: f32,
    pub goals_conceded_per_90: f32,
    pub now_cost_rank: i16,
    pub now_cost_rank_type: i16,
    pub form_rank: i16,
    pub form_rank_type: i16,
    pub points_per_game_rank: i16,
    pub points_per_game_rank_type: i16,
    pub selected_rank: i16,
    pub selected_rank_type: i16,
    pub starts_per_90: f32,
    pub clean_sheets_per_90: f32,
}
