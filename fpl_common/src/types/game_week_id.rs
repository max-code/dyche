use async_trait::async_trait;
use poise::serenity_prelude::{self as serenity};
use poise::{SlashArgError, SlashArgument};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "i16")]
#[derive(sqlx::Type)]
#[sqlx(transparent)]
pub struct GameWeekId(pub i16);

#[derive(Debug, thiserror::Error)]
#[error("GameWeek must be between 1 and 38, got {0}")]
pub struct GameWeekError(i16);

impl GameWeekId {
    pub const MIN: i16 = 1;
    pub const MAX: i16 = 38;
    pub const ALL: std::ops::RangeInclusive<i16> = Self::MIN..=Self::MAX;

    pub const FIRST: GameWeekId = GameWeekId(Self::MIN);
    pub const LAST: GameWeekId = GameWeekId(Self::MAX);

    pub fn new(game_week: i16) -> Result<Self, GameWeekError> {
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

    pub fn all_weeks_iter() -> impl Iterator<Item = GameWeekId> {
        (Self::MIN..=Self::MAX).map(|w| Self(w))
    }

    pub fn weeks_range_iter(start: i16, end: i16) -> impl Iterator<Item = GameWeekId> {
        (start..=end).map(|w| Self(w))
    }
}

impl Deref for GameWeekId {
    type Target = i16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for GameWeekId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<i16> for GameWeekId {
    type Error = GameWeekError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<GameWeekId> for i16 {
    fn from(id: GameWeekId) -> i16 {
        id.0 as i16
    }
}

impl PartialEq<i16> for GameWeekId {
    fn eq(&self, other: &i16) -> bool {
        self.0 == *other
    }
}

impl PartialEq<GameWeekId> for i16 {
    fn eq(&self, other: &GameWeekId) -> bool {
        *self == other.0
    }
}

impl FromStr for GameWeekId {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<i16>().and_then(|n| match GameWeekId::new(n) {
            Ok(id) => Ok(id),
            Err(_) => Err(s.parse::<i16>().unwrap_err()),
        })
    }
}

#[async_trait]
impl SlashArgument for GameWeekId {
    fn create(builder: serenity::CreateCommandOption) -> serenity::CreateCommandOption {
        builder
            .kind(serenity::CommandOptionType::Integer)
            .max_int_value(GameWeekId::MAX as u64)
            .min_int_value(GameWeekId::MIN as u64)
            .description(format!(
                "FPL Game Week ID ({}-{})",
                GameWeekId::MIN,
                GameWeekId::MAX
            ))
    }

    async fn extract(
        ctx: &serenity::Context,
        interaction: &serenity::CommandInteraction,
        value: &serenity::ResolvedValue<'_>,
    ) -> Result<GameWeekId, SlashArgError> {
        tracing::debug!("Extracting game_week_id from {:?}", value);
        let err: &'static str = "Couldn't parse provided Game Week ID into a GameWeekId type.";
        let val = poise::extract_slash_argument!(i16, ctx, interaction, value).await?;
        GameWeekId::try_from(val).map_err(|_| SlashArgError::new_command_structure_mismatch(err))
    }
}
