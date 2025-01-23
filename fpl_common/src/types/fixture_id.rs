use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "i32")]
#[derive(sqlx::Type)]
#[sqlx(transparent)]
pub struct FixtureId(i32);

#[derive(Debug, thiserror::Error)]
#[error("FixtureId must be between 1 and 380, got {0}")]
pub struct FixtureIdError(i32);

impl FixtureId {
    pub const MIN: i32 = 1;
    pub const MAX: i32 = 380;
    pub const ALL: std::ops::RangeInclusive<i32> = Self::MIN..=Self::MAX;

    pub fn new(game_week: i32) -> Result<Self, FixtureIdError> {
        if Self::ALL.contains(&game_week) {
            Ok(Self(game_week))
        } else {
            Err(FixtureIdError(game_week))
        }
    }
}

impl Deref for FixtureId {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for FixtureId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<i32> for FixtureId {
    type Error = FixtureIdError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<FixtureId> for i32 {
    fn from(id: FixtureId) -> Self {
        id.0
    }
}

impl PartialEq<i32> for FixtureId {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<FixtureId> for i32 {
    fn eq(&self, other: &FixtureId) -> bool {
        *self == other.0
    }
}
