use async_trait::async_trait;
use poise::serenity_prelude::{self as serenity};
use poise::{SlashArgError, SlashArgument};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "i16")]
#[derive(sqlx::Type)]
#[sqlx(transparent)]
pub struct FixtureId(i16);

#[derive(Debug, thiserror::Error)]
#[error("FixtureId must be between 1 and 380, got {0}")]
pub struct FixtureIdError(i16);

impl FixtureId {
    pub const MIN: i16 = 1;
    pub const MAX: i16 = 380;
    pub const ALL: std::ops::RangeInclusive<i16> = Self::MIN..=Self::MAX;

    pub fn new(game_week: i16) -> Result<Self, FixtureIdError> {
        if Self::ALL.contains(&game_week) {
            Ok(Self(game_week))
        } else {
            Err(FixtureIdError(game_week))
        }
    }
}

impl Deref for FixtureId {
    type Target = i16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for FixtureId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<i16> for FixtureId {
    type Error = FixtureIdError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<FixtureId> for i16 {
    fn from(id: FixtureId) -> Self {
        id.0
    }
}

impl PartialEq<i16> for FixtureId {
    fn eq(&self, other: &i16) -> bool {
        self.0 == *other
    }
}

impl PartialEq<FixtureId> for i16 {
    fn eq(&self, other: &FixtureId) -> bool {
        *self == other.0
    }
}

#[async_trait]
impl SlashArgument for FixtureId {
    fn create(builder: serenity::CreateCommandOption) -> serenity::CreateCommandOption {
        builder
            .kind(serenity::CommandOptionType::Integer)
            .max_int_value(FixtureId::MAX as u64)
            .min_int_value(FixtureId::MIN as u64)
            .description(format!(
                "FPL Fixture ID ({}-{})",
                FixtureId::MIN,
                FixtureId::MAX
            ))
    }

    async fn extract(
        ctx: &serenity::Context,
        interaction: &serenity::CommandInteraction,
        value: &serenity::ResolvedValue<'_>,
    ) -> Result<FixtureId, SlashArgError> {
        tracing::info!("Extracting fixture_id from {:?}", value);
        let err: &'static str = "Couldn't parse provided Fixture ID into a FixtureID type.";
        let val = poise::extract_slash_argument!(i16, ctx, interaction, value).await?;
        FixtureId::try_from(val).map_err(|_| SlashArgError::new_command_structure_mismatch(err))
    }
}
