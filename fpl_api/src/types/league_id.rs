use std::fmt::Display;

use serde::{Deserialize, Deserializer};

/* LeagueId - The users ID essentially. */
#[derive(Debug, Deserialize)]
pub struct LeagueId(pub u32);

impl LeagueId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
}

impl Display for LeagueId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
