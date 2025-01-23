use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "u8")]
pub struct ClubId(u8);

#[derive(Debug, thiserror::Error)]
#[error("ClubId must be between 1 and 20, got {0}")]
pub struct ClubIdError(u8);

impl ClubId {
    pub const MIN: u8 = 1;
    pub const MAX: u8 = 20;
    pub const ALL: std::ops::RangeInclusive<u8> = Self::MIN..=Self::MAX;

    pub fn new(club_id: u8) -> Result<Self, ClubIdError> {
        if Self::ALL.contains(&club_id) {
            Ok(Self(club_id))
        } else {
            Err(ClubIdError(club_id))
        }
    }
}

impl Deref for ClubId {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for ClubId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u8> for ClubId {
    type Error = ClubIdError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl PartialEq<u8> for ClubId {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ClubId> for u8 {
    fn eq(&self, other: &ClubId) -> bool {
        *self == other.0
    }
}
