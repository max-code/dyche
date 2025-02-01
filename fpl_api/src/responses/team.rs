use chrono::{DateTime, Utc};
use fpl_common::types::{ClubId, GameWeekId, TeamId};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TeamResponse {
    pub id: TeamId,
    pub joined_time: DateTime<Utc>,
    pub started_event: GameWeekId,
    pub favourite_team: Option<ClubId>,
    pub player_first_name: String,
    pub player_last_name: String,
    pub player_region_id: i16,
    pub player_region_name: String,
    pub player_region_iso_code_short: String,
    pub player_region_iso_code_long: String,
    pub summary_overall_points: i16,
    pub summary_overall_rank: i32,
    pub summary_event_points: i16,
    pub summary_event_rank: Option<i32>,
    pub current_event: i16,
    pub name: String,
    pub name_change_blocked: bool,
    pub last_deadline_bank: i16,
    pub last_deadline_value: i16,
    pub last_deadline_total_transfers: i16,
    pub leagues: ClassicLeagues,
}

#[derive(Debug, Deserialize)]
pub struct ClassicLeagues {
    pub classic: Vec<ClassicLeague>,
}

#[derive(Debug, Deserialize)]
pub struct ClassicLeague {
    pub id: i32,
    pub admin_entry: Option<i32>,
    pub rank_count: i32,
}
