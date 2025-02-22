use super::{FplRequest, FplResponseType};
use crate::responses::transfers::TransfersResponse;
use fpl_common::types::TeamId;

#[derive(Debug)]
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

    fn process_response(
        &self,
        response: FplResponseType,
    ) -> Result<Self::Response, Box<dyn std::error::Error>> {
        match response {
            FplResponseType::Json(value) => Ok(serde_json::from_value(value)?),
            FplResponseType::Binary(_) => Err("Expected JSON response, got binary".into()),
        }
    }
}
