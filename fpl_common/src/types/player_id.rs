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
pub struct PlayerId(pub i16);

impl PlayerId {
    pub const fn new(id: i16) -> Self {
        Self(id)
    }

    pub fn as_i32(&self) -> i16 {
        self.0
    }
}

// impl Default for PlayerId {
//     fn default() -> Self {
//         Self(0)
//     }
// }

impl Display for PlayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i16> for PlayerId {
    fn from(id: i16) -> Self {
        Self(id)
    }
}

impl From<PlayerId> for i16 {
    fn from(id: PlayerId) -> Self {
        id.0
    }
}

impl Deref for PlayerId {
    type Target = i16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl SlashArgument for PlayerId {
    fn create(builder: serenity::CreateCommandOption) -> serenity::CreateCommandOption {
        builder
            .kind(serenity::CommandOptionType::Integer)
            .description("FPL League ID")
    }

    async fn extract(
        ctx: &serenity::Context,
        interaction: &serenity::CommandInteraction,
        value: &serenity::ResolvedValue<'_>,
    ) -> Result<PlayerId, SlashArgError> {
        tracing::info!("Extracting player_id from {:?}", value);
        let val = poise::extract_slash_argument!(i16, ctx, interaction, value).await?;
        Ok(PlayerId::from(val))
    }
}
