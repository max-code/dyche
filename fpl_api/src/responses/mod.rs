use serde::{Deserialize, Deserializer};

fn string_to_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    s.parse::<f32>().map_err(serde::de::Error::custom)
}

pub mod fixtures;
pub mod game_week_players_stats;
pub mod mini_league;
pub mod player;
pub mod team;
pub mod team_game_week;
pub mod transfers;
