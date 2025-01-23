use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "u8")]
pub struct GameWeekId(u8);

#[derive(Debug, thiserror::Error)]
#[error("GameWeek must be between 1 and 38, got {0}")]
pub struct GameWeekError(u8);

impl GameWeekId {
    pub const MIN: u8 = 1;
    pub const MAX: u8 = 38;
    pub const ALL: std::ops::RangeInclusive<u8> = Self::MIN..=Self::MAX;

    pub const FIRST: GameWeekId = GameWeekId(Self::MIN);
    pub const LAST: GameWeekId = GameWeekId(Self::MAX);

    pub fn new(game_week: u8) -> Result<Self, GameWeekError> {
        if Self::ALL.contains(&game_week) {
            Ok(Self(game_week))
        } else {
            Err(GameWeekError(game_week))
        }
    }

    pub fn is_first(&self) -> bool {
        self.0 == Self::FIRST
    }

    pub fn is_last(&self) -> bool {
        self.0 == Self::LAST
    }

    pub fn next(&self) -> Option<GameWeekId> {
        if self.is_last() {
            None
        } else {
            Some(Self(self.0 + 1))
        }
    }

    pub fn previous(&self) -> Option<GameWeekId> {
        if self.is_first() {
            None
        } else {
            Some(Self(self.0 - 1))
        }
    }
}

impl Deref for GameWeekId {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for GameWeekId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u8> for GameWeekId {
    type Error = GameWeekError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl PartialEq<u8> for GameWeekId {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl PartialEq<GameWeekId> for u8 {
    fn eq(&self, other: &GameWeekId) -> bool {
        *self == other.0
    }
}
