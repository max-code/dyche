use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::ops::Deref;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, sqlx::Type,
)]
#[sqlx(transparent)]
pub struct PlayerId(pub i32);

impl PlayerId {
    pub const fn new(id: i32) -> Self {
        Self(id)
    }

    pub fn as_i32(&self) -> i32 {
        self.0
    }
}

impl Display for PlayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i32> for PlayerId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}

impl From<PlayerId> for i32 {
    fn from(id: PlayerId) -> Self {
        id.0
    }
}

impl Deref for PlayerId {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
