use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/* PlayerPosition - Number representing the players FPL position */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(try_from = "u8")]
pub enum PlayerPosition {
    Goalkeeper,
    Defender,
    Midfielder,
    Attacker,
}

#[derive(Debug, thiserror::Error)]
#[error("Invalid player position: {0}")]
pub struct ParsePositionError(String);

impl PlayerPosition {
    pub fn to_u8(&self) -> u8 {
        match self {
            PlayerPosition::Goalkeeper => 1,
            PlayerPosition::Defender => 2,
            PlayerPosition::Midfielder => 3,
            PlayerPosition::Attacker => 4,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Goalkeeper => "Goalkeeper",
            Self::Defender => "Defender",
            Self::Midfielder => "Midfielder",
            Self::Attacker => "Attacker",
        }
    }

    pub const fn short_name(self) -> &'static str {
        match self {
            Self::Goalkeeper => "GK",
            Self::Defender => "DEF",
            Self::Midfielder => "MID",
            Self::Attacker => "FWD",
        }
    }
}

impl Display for PlayerPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for PlayerPosition {
    type Err = ParsePositionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "goalkeeper" | "gk" => Ok(Self::Goalkeeper),
            "defender" | "def" => Ok(Self::Defender),
            "midfielder" | "mid" => Ok(Self::Midfielder),
            "attacker" | "fwd" | "forward" => Ok(Self::Attacker),
            _ => Err(ParsePositionError(s.to_owned())),
        }
    }
}

impl TryFrom<u8> for PlayerPosition {
    type Error = ParsePositionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Goalkeeper),
            2 => Ok(Self::Defender),
            3 => Ok(Self::Midfielder),
            4 => Ok(Self::Attacker),
            _ => Err(ParsePositionError(format!(
                "Invalid position number: {}",
                value
            ))),
        }
    }
}
