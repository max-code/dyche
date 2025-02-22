use super::{FplRequest, FplResponseType};
use crate::responses::game_state::GameStateResponse;

#[derive(Debug, Default)]
pub struct GameStateRequest {}

impl FplRequest for GameStateRequest {
    type Response = GameStateResponse;

    fn to_url(&self, base_url: &str) -> String {
        format!("{}/bootstrap-static/", base_url)
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
