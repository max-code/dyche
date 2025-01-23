use crate::types::{GameWeekId, LeagueId, TeamId};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MiniLeagueResponse {
    pub last_updated_data: DateTime<Utc>,
    pub league: League,
    pub standings: Standings,
}

#[derive(Debug, Deserialize)]
pub struct League {
    pub id: LeagueId,
    pub name: String,
    pub created: DateTime<Utc>,
    pub closed: bool,
    pub max_entries: Option<u32>,
    pub league_type: String,
    pub scoring: String,
    pub admin_entry: TeamId,
    pub start_event: GameWeekId,
    pub code_privacy: String,
    pub has_cup: bool,
    pub cup_league: Option<u32>,
    pub rank: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct Standings {
    pub has_next: bool,
    pub page: u32,
    pub results: Vec<Standing>,
}

#[derive(Debug, Deserialize)]
pub struct Standing {
    pub id: u32,
    pub event_total: u8,
    pub player_name: String,
    pub rank: u32,
    pub last_rank: u32,
    pub rank_sort: u32,
    pub total: u16,
    pub entry: TeamId,
    pub entry_name: String,
    pub has_played: bool,
}
