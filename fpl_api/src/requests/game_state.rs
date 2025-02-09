use super::FplRequest;
use crate::responses::game_state::GameStateResponse;

#[derive(Debug, Default)]
pub struct GameStateRequest {}

impl FplRequest for GameStateRequest {
    type Response = GameStateResponse;

    fn to_url(&self, base_url: &str) -> String {
        format!("{}/bootstrap-static/", base_url)
    }
}
