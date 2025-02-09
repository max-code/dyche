use async_trait::async_trait;
use poise::serenity_prelude::{self as serenity};
use poise::{SlashArgError, SlashArgument};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::ops::Deref;
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, sqlx::Type,
)]
#[sqlx(transparent)]
pub struct LeagueId(pub i32);

#[derive(Debug, thiserror::Error)]
#[error("LeagueId {0} invalid")]
pub struct LeagueIdError(i16);

impl LeagueId {
    pub const fn new(id: i32) -> Self {
        Self(id)
    }

    pub fn as_i32(&self) -> i32 {
        self.0
    }
}

impl Display for LeagueId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i32> for LeagueId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}

impl From<LeagueId> for i32 {
    fn from(id: LeagueId) -> Self {
        id.0
    }
}

impl Deref for LeagueId {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl SlashArgument for LeagueId {
    fn create(builder: serenity::CreateCommandOption) -> serenity::CreateCommandOption {
        builder
            .kind(serenity::CommandOptionType::Integer)
            .description("FPL League ID")
    }

    async fn extract(
        ctx: &serenity::Context,
        interaction: &serenity::CommandInteraction,
        value: &serenity::ResolvedValue<'_>,
    ) -> Result<LeagueId, SlashArgError> {
        tracing::info!("Extracting league_id from {:?}", value);
        let val = poise::extract_slash_argument!(i32, ctx, interaction, value).await?;
        Ok(LeagueId::from(val))
    }
}
