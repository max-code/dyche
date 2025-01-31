use serde::de::DeserializeOwned;
use serde_json::Value;

pub trait FplRequest {
    type Response: DeserializeOwned;
    fn to_url(&self, base_url: &str) -> String;
    fn process_response(&self, response: Value) -> Result<Self::Response, serde_json::Error> {
        serde_json::from_value(response)
    }
}

pub mod fixtures;
pub mod game_state;
pub mod game_week_players;
pub mod mini_league;
pub mod player;
pub mod team;
pub mod team_game_week;
pub mod transfers;

pub use fixtures::*;
pub use game_state::*;
pub use game_week_players::*;
pub use mini_league::*;
pub use player::*;
pub use team::*;
pub use team_game_week::*;
pub use transfers::*;
