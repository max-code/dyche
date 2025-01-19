use std::fmt::Display;

use serde::{Deserialize, Deserializer};

/* GameWeek - positive integer in the range  [1,38] */
#[derive(Debug)]
pub struct GameWeek(pub u8);

impl GameWeek {
    pub fn new(game_week: u8) -> Option<Self> {
        match game_week {
            1..=38 => Some(Self(game_week)),
            _ => None,
        }
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

impl Display for GameWeek {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl TryFrom<u8> for GameWeek {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        GameWeek::new(value)
            .ok_or_else(|| format!("GameWeek must be between 1 and 38, got {}", value))
    }
}

impl<'de> Deserialize<'de> for GameWeek {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        value.try_into().map_err(serde::de::Error::custom)
    }
}
