use crate::types::{GameWeek, PlayerId, TeamId};
use chrono::{DateTime, Utc};
use serde::Deserialize;

pub type TransfersResponse = Vec<TransferResponse>;

#[derive(Debug, Deserialize)]
pub struct TransferResponse {
    pub element_in: PlayerId,
    pub element_out: PlayerId,
    pub element_in_cost: u8,
    pub element_out_cost: u8,
    pub entry: TeamId,
    pub event: GameWeek,
    pub time: DateTime<Utc>,
}
