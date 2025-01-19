use crate::types::TeamId;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TeamResponse {
    pub id: TeamId,
    pub joined_time: String,
    pub started_event: u8,
    pub favourite_team: u8,
    pub player_first_name: String,
    pub player_last_name: String,
    pub player_region_id: u16,
    pub player_region_name: String,
    pub player_region_iso_code_short: String,
    pub player_region_iso_code_long: String,
    pub summary_overall_points: u16,
    pub summary_overall_rank: u32,
    pub summary_event_points: u8,
    pub summary_event_rank: u32,
    pub current_event: u8,
    pub name: String,
    pub name_change_blocked: bool,
    pub last_deadline_bank: u16,
    pub last_deadline_value: u16,
    pub last_deadline_total_transfers: u16,
}
