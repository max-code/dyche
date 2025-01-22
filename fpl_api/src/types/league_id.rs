use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct LeagueId(pub u32);

impl LeagueId {
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

impl Display for LeagueId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for LeagueId {
    fn from(id: u32) -> Self {
        Self(id)
    }
}

impl From<LeagueId> for u32 {
    fn from(id: LeagueId) -> Self {
        id.0
    }
}

impl Deref for LeagueId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
