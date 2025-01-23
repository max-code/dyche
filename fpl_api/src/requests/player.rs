use super::FplRequest;
use crate::responses::player::PlayerResponse;
use fpl_common::types::PlayerId;

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
}
