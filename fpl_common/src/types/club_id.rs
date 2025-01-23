use serde::{Deserialize, Serialize};

use std::fmt::Display;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "i32")]
#[derive(sqlx::Type)]
#[sqlx(transparent)]
pub struct ClubId(i32);

#[derive(Debug, thiserror::Error)]
#[error("ClubId must be between 1 and 20, got {0}")]
pub struct ClubIdError(i32);

impl ClubId {
    pub const MIN: i32 = 1;
    pub const MAX: i32 = 20;
    pub const ALL: std::ops::RangeInclusive<i32> = Self::MIN..=Self::MAX;

    pub fn new(club_id: i32) -> Result<Self, ClubIdError> {
        if Self::ALL.contains(&club_id) {
            Ok(Self(club_id))
        } else {
            Err(ClubIdError(club_id))
        }
    }
}

impl Deref for ClubId {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for ClubId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<i32> for ClubId {
    type Error = ClubIdError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<ClubId> for i32 {
    fn from(id: ClubId) -> Self {
        id.0
    }
}

impl PartialEq<i32> for ClubId {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ClubId> for i32 {
    fn eq(&self, other: &ClubId) -> bool {
        *self == other.0
    }
}
