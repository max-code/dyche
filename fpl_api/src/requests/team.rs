use super::{FplRequest, FplResponseType};
use crate::responses::team::TeamResponse;
use fpl_common::types::TeamId;
use serde::de::Error;

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
        response: FplResponseType,
    ) -> Result<Self::Response, Box<dyn std::error::Error>> {
        match response {
            FplResponseType::Json(value) => {
                if let Some(message) = value.as_str() {
                    return Err(Box::new(serde_json::Error::custom(message)));
                }

                Ok(serde_json::from_value(value)?)
            }
            FplResponseType::Binary(_) => Err("Expected JSON response, got binary".into()),
        }
    }
}
