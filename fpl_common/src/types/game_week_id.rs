use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "i32")]
#[derive(sqlx::Type)]
#[sqlx(transparent)]
pub struct GameWeekId(i32);

#[derive(Debug, thiserror::Error)]
#[error("GameWeek must be between 1 and 38, got {0}")]
pub struct GameWeekError(i32);

impl GameWeekId {
    pub const MIN: i32 = 1;
    pub const MAX: i32 = 38;
    pub const ALL: std::ops::RangeInclusive<i32> = Self::MIN..=Self::MAX;

    pub const FIRST: GameWeekId = GameWeekId(Self::MIN);
    pub const LAST: GameWeekId = GameWeekId(Self::MAX);

    pub fn new(game_week: i32) -> Result<Self, GameWeekError> {
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
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for GameWeekId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<i32> for GameWeekId {
    type Error = GameWeekError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<GameWeekId> for i32 {
    fn from(id: GameWeekId) -> i32 {
        id.0 as i32
    }
}

impl PartialEq<i32> for GameWeekId {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<GameWeekId> for i32 {
    fn eq(&self, other: &GameWeekId) -> bool {
        *self == other.0
    }
}
