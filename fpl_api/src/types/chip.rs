use serde::{Deserialize, Deserializer};
use std::fmt::Display;

/* GameWeek - positive integer in the range  [1,38] */
#[derive(Debug)]
pub enum Chip {
    WildCard,
    FreeHit,
    TripleCaptain,
}

impl Chip {
    pub fn new(chip: &str) -> Option<Self> {
        match chip {
            "freehit" => Some(Self::FreeHit),
            "wildcard" => Some(Self::WildCard),
            "3xc" => Some(Self::TripleCaptain),
            _ => None,
        }
    }

    pub fn value(&self) -> &str {
        match self {
            Self::WildCard => "wildcard",
            Self::FreeHit => "freehit",
            Self::TripleCaptain => "3xc",
        }
    }
}

impl Display for Chip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl TryFrom<&str> for Chip {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Chip::new(value).ok_or_else(|| format!("Chip must be between 1 and 4, got {}", value))
    }
}

impl<'de> Deserialize<'de> for Chip {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value.as_str().try_into().map_err(serde::de::Error::custom)
    }
}
