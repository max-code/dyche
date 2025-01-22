use serde::de::DeserializeOwned;

pub trait FplRequest {
    type Response: DeserializeOwned;
    fn to_url(&self, base_url: &str) -> String;
}

pub mod fixtures;
pub mod game_week_players_stats;
pub mod mini_league;
pub mod player;
pub mod team;
pub mod team_game_week;

pub use fixtures::*;
pub use game_week_players_stats::*;
pub use mini_league::*;
pub use player::*;
pub use team::*;
pub use team_game_week::*;
