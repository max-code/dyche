use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "u16")]
pub struct FixtureId(u16);

#[derive(Debug, thiserror::Error)]
#[error("FixtureId must be between 1 and 380, got {0}")]
pub struct FixtureIdError(u16);

impl FixtureId {
    pub const MIN: u16 = 1;
    pub const MAX: u16 = 380;
    pub const ALL: std::ops::RangeInclusive<u16> = Self::MIN..=Self::MAX;

    pub fn new(game_week: u16) -> Result<Self, FixtureIdError> {
        if Self::ALL.contains(&game_week) {
            Ok(Self(game_week))
        } else {
            Err(FixtureIdError(game_week))
        }
    }
}

impl Deref for FixtureId {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for FixtureId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GW{}", self.0)
    }
}

impl TryFrom<u16> for FixtureId {
    type Error = FixtureIdError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl PartialEq<u16> for FixtureId {
    fn eq(&self, other: &u16) -> bool {
        self.0 == *other
    }
}

impl PartialEq<FixtureId> for u16 {
    fn eq(&self, other: &FixtureId) -> bool {
        *self == other.0
    }
}
