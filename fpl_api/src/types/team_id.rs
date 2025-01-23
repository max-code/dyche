use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct TeamId(pub u32);

/* This is the USERS id, e.g. mine is 1871038. NOT related to clubs e.g. Forest, They're called clubs */
impl TeamId {
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

impl Display for TeamId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for TeamId {
    fn from(id: u32) -> Self {
        Self(id)
    }
}

impl From<TeamId> for u32 {
    fn from(id: TeamId) -> Self {
        id.0
    }
}

impl Deref for TeamId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
