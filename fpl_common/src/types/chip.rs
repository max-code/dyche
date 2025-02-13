use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

/* Chip - wildcard, freehit, triple cap, assmanm, bboost */
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
    #[serde(rename = "manager")]
    AssMan,
}

#[derive(Debug, thiserror::Error)]
#[error("Invalid chip name: {0}")]
pub struct ParseChipError(String);

impl Chip {
    pub const ALL: [Chip; 5] = [
        Chip::WildCard,
        Chip::FreeHit,
        Chip::TripleCaptain,
        Chip::BenchBoost,
        Chip::AssMan,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::WildCard => "wildcard",
            Self::FreeHit => "freehit",
            Self::TripleCaptain => "3xc",
            Self::BenchBoost => "bboost",
            Self::AssMan => "manager",
        }
    }

    pub fn pretty_name(&self) -> &'static str {
        match self {
            Self::WildCard => "Wildcard",
            Self::FreeHit => "Free Hit",
            Self::TripleCaptain => "Triple Captain",
            Self::BenchBoost => "Bench Boost",
            Self::AssMan => "Assistant Manager",
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
            "manager" => Ok(Self::AssMan),
            _ => Err(ParseChipError(s.to_owned())),
        }
    }
}
