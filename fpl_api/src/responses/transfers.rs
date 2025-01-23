use chrono::{DateTime, Utc};
use fpl_common::types::{GameWeekId, PlayerId, TeamId};
use serde::Deserialize;

pub type TransfersResponse = Vec<TransferResponse>;

#[derive(Debug, Deserialize)]
pub struct TransferResponse {
    pub element_in: PlayerId,
    pub element_out: PlayerId,
    pub element_in_cost: u8,
    pub element_out_cost: u8,
    pub entry: TeamId,
    pub event: GameWeekId,
    pub time: DateTime<Utc>,
}
