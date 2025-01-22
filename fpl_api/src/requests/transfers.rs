use super::FplRequest;
use crate::responses::transfers::TransfersResponse;
use crate::types::TeamId;

pub struct TransfersRequest {
    team_id: TeamId,
}

impl TransfersRequest {
    pub fn new(team_id: TeamId) -> Self {
        Self { team_id }
    }
}

impl FplRequest for TransfersRequest {
    type Response = TransfersResponse;

    fn to_url(&self, base_url: &str) -> String {
        format!("{}/entry/{}/transfers", base_url, self.team_id)
    }
}
