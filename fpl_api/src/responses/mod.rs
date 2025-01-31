use serde::{Deserialize, Deserializer};

fn string_to_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    s.parse::<f32>().map_err(serde::de::Error::custom)
}

fn string_to_option_f32<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        s.parse::<f32>().map(Some).map_err(serde::de::Error::custom)
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
