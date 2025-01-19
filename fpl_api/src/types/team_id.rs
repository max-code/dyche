use std::fmt::Display;

use serde::{Deserialize, Deserializer};

/* TeamId - The users ID essentially. */
#[derive(Debug, Deserialize)]
pub struct TeamId(pub u32);

impl TeamId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
}

impl Display for TeamId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
