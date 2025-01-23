use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

/* Chip - wildcard, freehit, triple cap, assmanm bboost */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Chip {
    #[serde(rename = "wildcard")]
    WildCard,
    #[serde(rename = "freehit")]
    FreeHit,
    #[serde(rename = "3xc")]
    TripleCaptain,
    #[serde(rename = "bboost")]
    BenchBoost,
}

#[derive(Debug, thiserror::Error)]
#[error("Invalid chip name: {0}")]
pub struct ParseChipError(String);

impl Chip {
    pub const ALL: [Chip; 3] = [Chip::WildCard, Chip::FreeHit, Chip::TripleCaptain];

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::WildCard => "wildcard",
            Self::FreeHit => "freehit",
            Self::TripleCaptain => "3xc",
            Self::BenchBoost => "bboost",
        }
    }
}

impl Display for Chip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Chip {
    type Err = ParseChipError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "freehit" => Ok(Self::FreeHit),
            "wildcard" => Ok(Self::WildCard),
            "3xc" => Ok(Self::TripleCaptain),
            "bboost" => Ok(Self::BenchBoost),
            _ => Err(ParseChipError(s.to_owned())),
        }
    }
}
