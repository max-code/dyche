use super::{FplRequest, FplResponseType};
use crate::responses::player::PlayerResponse;
use fpl_common::types::PlayerId;

#[derive(Debug)]
pub struct PlayerRequest {
    pub player_id: PlayerId,
}

impl PlayerRequest {
    pub fn new(player_id: PlayerId) -> Self {
        Self { player_id }
    }
}

impl FplRequest for PlayerRequest {
    type Response = PlayerResponse;

    fn to_url(&self, base_url: &str) -> String {
        format!("{}/element-summary/{}/", base_url, self.player_id)
    }

    fn process_response(
        &self,
        response: FplResponseType,
    ) -> Result<Self::Response, Box<dyn std::error::Error>> {
        match response {
            FplResponseType::Json(value) => {
                let mut resp: PlayerResponse = serde_json::from_value(value)?;
                resp.player_id = Some(self.player_id);
                Ok(resp)
            }
            FplResponseType::Binary(_) => Err("Expected JSON response, got binary".into()),
        }
    }
}
