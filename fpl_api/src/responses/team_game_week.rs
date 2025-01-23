use crate::types::{Chip, GameWeekId, PlayerPosition, TeamId};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TeamGameWeekResponse {
    pub active_chip: Option<Chip>,
    pub automatic_subs: Vec<AutomaticSub>,
    pub entry_history: EntryHistory,
    pub picks: Vec<Pick>,
}

#[derive(Debug, Deserialize)]
pub struct EntryHistory {
    pub event: GameWeekId,
    pub points: u8,
    pub total_points: u16,
    pub rank: u32,
    pub rank_sort: u32,
    pub overall_rank: u32,
    pub percentile_rank: u8,
    pub bank: u16,
    pub value: u16,
    pub event_transfers: u8,
    pub event_transfers_cost: u8,
    pub points_on_bench: u8,
}

#[derive(Debug, Deserialize)]
pub struct Pick {
    pub element: u16,
    pub position: u8,
    pub multiplier: u8,
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
    pub element_in: u16,
    pub element_out: u16,
    pub event: GameWeekId,
}
