use serde_json::Value;

#[derive(Debug)]
pub enum FplResponseType {
    Json(Value),
    Binary(Vec<u8>),
}

pub trait FplRequest {
    type Response;
    fn to_url(&self, base_url: &str) -> String;
    fn process_response(
        &self,
        response: FplResponseType,
    ) -> Result<Self::Response, Box<dyn std::error::Error>>;

    fn is_binary(&self) -> bool {
        false
    }
}

pub mod fixtures;
pub mod game_state;
pub mod game_week_players;
pub mod mini_league;
pub mod player;
pub mod player_image;
pub mod team;
pub mod team_game_week;
pub mod transfers;

pub use fixtures::*;
pub use game_state::*;
pub use game_week_players::*;
pub use mini_league::*;
pub use player::*;
pub use player_image::*;
pub use team::*;
pub use team_game_week::*;
pub use transfers::*;
