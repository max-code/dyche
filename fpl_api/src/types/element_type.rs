use std::fmt::Display;

use serde::{Deserialize, Deserializer};

/* GameWeek - positive integer in the range  [1,38] */
#[derive(Debug)]
pub struct ElementType(u8);

impl ElementType {
    pub fn new(position: u8) -> Option<Self> {
        match position {
            1..=4 => Some(Self(position)),
            _ => None,
        }
    }

    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn to_position(&self) -> &str {
        match self.value() {
            1 => "Goalkeeper",
            2 => "Defender",
            3 => "Midfielder",
            4 => "Attacker",
            _ => "Unknown",
        }
    }
}

impl Display for ElementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl TryFrom<u8> for ElementType {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        ElementType::new(value)
            .ok_or_else(|| format!("Element type must be between 1 and 4, got {}", value))
    }
}

impl<'de> Deserialize<'de> for ElementType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        value.try_into().map_err(serde::de::Error::custom)
    }
}
