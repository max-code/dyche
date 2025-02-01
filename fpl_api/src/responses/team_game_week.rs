use fpl_common::types::{Chip, GameWeekId, PlayerId, PlayerPosition, TeamId};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum TeamGameWeekResponseWrapper {
    Success(TeamGameWeekResponse),
    Error(ErrorResponse),
    PlainText(String),
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub detail: String,
}

#[derive(Debug, Deserialize)]
pub struct TeamGameWeekResponse {
    pub active_chip: Option<Chip>,
    pub automatic_subs: Vec<AutomaticSub>,
    pub entry_history: EntryHistory,
    pub picks: Vec<Pick>,
    pub team_id: Option<TeamId>,
    pub game_week_id: Option<GameWeekId>,
}

#[derive(Debug, Deserialize)]
pub struct EntryHistory {
    pub event: GameWeekId,
    pub points: i16,
    pub total_points: i16,
    pub rank: Option<i32>,
    pub rank_sort: Option<i32>,
    pub overall_rank: i32,
    pub percentile_rank: Option<i16>,
    pub bank: i16,
    pub value: i16,
    pub event_transfers: i16,
    pub event_transfers_cost: i16,
    pub points_on_bench: i16,
}

#[derive(Debug, Deserialize)]
pub struct Pick {
    pub element: PlayerId,
    pub position: i16,
    pub multiplier: i16,
    pub is_captain: bool,
    pub is_vice_captain: bool,
    pub element_type: PlayerPosition,
}

impl Pick {
    pub fn is_benched(&self) -> bool {
        self.multiplier == 0 && self.position >= 12
    }
}

#[derive(Debug, Deserialize)]
pub struct AutomaticSub {
    pub entry: TeamId,
    pub element_in: PlayerId,
    pub element_out: PlayerId,
    pub event: GameWeekId,
}
