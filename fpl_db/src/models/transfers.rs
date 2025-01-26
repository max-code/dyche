use chrono::{DateTime, Utc};
use fpl_api::responses::transfers::TransferResponse;
use fpl_common::types::{GameWeekId, PlayerId, TeamId};

#[derive(Debug, sqlx::FromRow)]
pub struct Transfer {
    pub player_in_id: PlayerId,
    pub player_out_id: PlayerId,
    pub player_in_cost: i16,
    pub player_out_cost: i16,
    pub team_id: TeamId,
    pub game_week_id: GameWeekId,
    pub transfer_time: DateTime<Utc>,
}

impl From<&TransferResponse> for Transfer {
    fn from(transfer_response: &TransferResponse) -> Self {
        Self {
            player_in_id: transfer_response.element_in,
            player_out_id: transfer_response.element_out,
            player_in_cost: transfer_response.element_in_cost,
            player_out_cost: transfer_response.element_out_cost,
            team_id: transfer_response.entry,
            game_week_id: transfer_response.event,
            transfer_time: transfer_response.time,
        }
    }
}
