use super::FplRequest;
use crate::responses::game_state::GameStateResponse;

#[derive(Debug)]
pub struct GameStateRequest {}

impl GameStateRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl FplRequest for GameStateRequest {
    type Response = GameStateResponse;

    fn to_url(&self, base_url: &str) -> String {
        format!("{}/bootstrap-static/", base_url)
    }
}
