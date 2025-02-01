use super::FplRequest;
use crate::responses::team::TeamResponse;
use fpl_common::types::TeamId;

#[derive(Debug)]
pub struct TeamRequest {
    pub team_id: TeamId,
}

impl TeamRequest {
    pub fn new(team_id: TeamId) -> Self {
        Self { team_id }
    }
}

impl FplRequest for TeamRequest {
    type Response = TeamResponse;

    fn to_url(&self, base_url: &str) -> String {
        format!("{}/entry/{}/", base_url, self.team_id)
    }

    fn process_response(
        &self,
        response: serde_json::Value,
    ) -> Result<Self::Response, serde_json::Error> {
        if let Some(message) = response.as_str() {
            return Err(serde::de::Error::custom(message));
        }

        serde_json::from_value(response)
    }
}
