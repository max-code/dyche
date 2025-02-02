use async_trait::async_trait;
use poise::serenity_prelude::{self as serenity};
use poise::{SlashArgError, SlashArgument};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, sqlx::Type,
)]
#[sqlx(transparent)]
pub struct TeamId(pub i32);

#[derive(Debug, thiserror::Error)]
#[error("TeamId must be i32, got {0}")]
pub struct TeamIdError(i64);

/* This is the USERS id, e.g. mine is 1871038. NOT related to clubs e.g. Forest, They're called clubs */
impl TeamId {
    pub const fn new(id: i32) -> Self {
        Self(id)
    }

    pub fn as_i32(&self) -> i32 {
        self.0
    }
}

impl Display for TeamId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i32> for TeamId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}

impl TryFrom<i64> for TeamId {
    type Error = TeamIdError;
    fn try_from(id: i64) -> Result<Self, Self::Error> {
        if id > i32::MAX as i64 || id < i32::MIN as i64 {
            return Err(TeamIdError(id));
        }

        Ok(Self(id as i32))
    }
}

impl From<TeamId> for i32 {
    fn from(id: TeamId) -> Self {
        id.0
    }
}

impl From<u32> for TeamId {
    fn from(id: u32) -> Self {
        Self::new(id as i32)
    }
}

impl Deref for TeamId {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for TeamId {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<i32>().map(TeamId::new)
    }
}

#[async_trait]
impl SlashArgument for TeamId {
    fn create(builder: serenity::CreateCommandOption) -> serenity::CreateCommandOption {
        builder
            .kind(serenity::CommandOptionType::Integer)
            .min_int_value(0)
            .description("FPL Team ID (from the FPL website)")
    }

    async fn extract(
        ctx: &serenity::Context,
        interaction: &serenity::CommandInteraction,
        value: &serenity::ResolvedValue<'_>,
    ) -> Result<TeamId, SlashArgError> {
        tracing::info!("Extracting team_id from {:?}", value);
        let val = poise::extract_slash_argument!(u32, ctx, interaction, value).await?;
        Ok(TeamId::from(val))
    }
}
