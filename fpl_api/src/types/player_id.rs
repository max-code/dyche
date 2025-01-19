use std::fmt::Display;

use serde::{Deserialize, Deserializer};

/* PlayerId - The users ID essentially. */
#[derive(Debug, Deserialize)]
pub struct PlayerId(pub u16);

impl PlayerId {
    pub fn new(id: u16) -> Self {
        Self(id)
    }
}

impl Display for PlayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
