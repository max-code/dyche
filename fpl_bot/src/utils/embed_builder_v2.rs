use poise::serenity_prelude::CreateEmbed;
use serenity::all::{CommandInteraction, CreateEmbedAuthor, Http};
use std::sync::Arc;
use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
};

use crate::{Context, Error};
pub trait State {}
pub trait Sendable: State {}

pub struct Uninitialised;
pub struct ProcessingState;
pub struct SuccessState;
pub struct ErrorState;

impl State for Uninitialised {}
impl State for ProcessingState {}
impl State for SuccessState {}
impl State for ErrorState {}

impl Sendable for ProcessingState {}
impl Sendable for SuccessState {}
impl Sendable for ErrorState {}

pub trait EmbedState {
    fn emoji() -> &'static str;
    fn color() -> (u8, u8, u8);
    fn state_name() -> &'static str;
}

impl EmbedState for ProcessingState {
    fn emoji() -> &'static str {
        "<a:loading:1286983242398040125>"
    }
    fn color() -> (u8, u8, u8) {
        (45, 0, 77)
    }
    fn state_name() -> &'static str {
        "Processing"
    }
}

impl EmbedState for ErrorState {
    fn emoji() -> &'static str {
        "<:reject:1286991959914975284>"
    }
    fn color() -> (u8, u8, u8) {
        (171, 12, 44)
    }
    fn state_name() -> &'static str {
        "Error"
    }
}

impl EmbedState for SuccessState {
    fn emoji() -> &'static str {
        "<:success:1286991961257148437>"
    }
    fn color() -> (u8, u8, u8) {
        (0, 255, 136)
    } // Green
    fn state_name() -> &'static str {
        "Success"
    }
}

const ICON_URL: &str = "https://cdn.discordapp.com/app-icons/1275927396129964054/e1ed26e9fed1b459ec6f19b2712898e6.png?size=512";
const BOT_NAME: &str = "FPL Bot ⚽️";

pub struct Embed<'a, S: State> {
    title: Option<String>,
    body: Option<String>,
    images: Option<Vec<PathBuf>>,
    state: PhantomData<S>,
    sent: bool,
    interaction: &'a CommandInteraction,
    http: Arc<Http>,
    token: String,
}

impl<'a, S: State> Embed<'a, S> {
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn image(mut self, image_uri: impl AsRef<Path>) -> Self {
        let path_buf = image_uri.as_ref().to_path_buf();
        match &mut self.images {
            Some(images) => images.push(path_buf),
            None => self.images = Some(vec![path_buf]),
        }
        self
    }

    pub fn images(mut self, image_uris: &[impl AsRef<Path>]) -> Self {
        self.images = Some(
            image_uris
                .iter()
                .map(|p| p.as_ref().to_path_buf())
                .collect(),
        );
        self
    }

    pub fn transition<T: State>(self) -> Embed<'a, T> {
        Embed {
            title: self.title,
            body: self.body,
            images: self.images,
            state: std::marker::PhantomData,
            http: self.http,
            interaction: &self.interaction,
            sent: self.sent,
            token: self.token,
        }
    }
}

impl<'a> Embed<'a, Uninitialised> {
    pub fn new(ctx: Context<'a>) -> Result<Self, Error> {
        let raw_ctx = match ctx {
            Context::Application(ctx) => ctx,
            _ => return Err("This command only works as a slash command".into()),
        };
        let interaction = raw_ctx.interaction;
        let http = ctx.serenity_context().http.clone();
        let token = interaction.token.clone();

        Ok(Self {
            title: None,
            body: None,
            images: None,
            state: std::marker::PhantomData,
            http,
            token,
            interaction,
            sent: false,
        })
    }

    pub fn processing(self) -> Embed<'a, ProcessingState> {
        self.transition()
    }

    pub fn error(self) -> Embed<'a, ErrorState> {
        self.transition()
    }

    pub fn success(self) -> Embed<'a, SuccessState> {
        self.transition()
    }
}

impl<'a> Embed<'a, ProcessingState> {
    pub fn success(self) -> Embed<'a, SuccessState> {
        self.transition()
    }
}

impl<'a, S: Sendable + EmbedState> Embed<'a, S> {
    pub async fn send(&mut self) -> Result<(), Error> {
        if self.sent {
            return self.edit().await;
        }

        let mut attachments = vec![];
        if let Some(images) = &self.images {
            for image_path in images {
                let attachment = serenity::builder::CreateAttachment::path(image_path)
                    .await
                    .map_err(|e| format!("Failed to create attachment: {}", e))?;
                attachments.push(attachment);
            }
        }

        let mut embed = self.make_embed();

        if let Some(first_attachment) = attachments.first() {
            embed = embed.image(format!("attachment://{}", first_attachment.filename));
        }

        let mut response_message =
            serenity::builder::CreateInteractionResponseMessage::new().embed(embed);

        if !attachments.is_empty() {
            response_message = response_message.files(attachments);
        }

        self.interaction
            .create_response(
                &self.http,
                serenity::builder::CreateInteractionResponse::Message(response_message),
            )
            .await?;

        self.sent = true;

        Ok(())
    }

    async fn edit(&mut self) -> Result<(), Error> {
        let mut attachments = vec![];
        if let Some(images) = &self.images {
            for image_path in images {
                let attachment = serenity::builder::CreateAttachment::path(image_path)
                    .await
                    .map_err(|e| format!("Attachment error: {}", e))?;
                attachments.push(attachment);
            }
        }

        let mut embed = self.make_embed();

        if let Some(first_attachment) = attachments.first() {
            embed = embed.image(format!("attachment://{}", first_attachment.filename));
        }

        self.http
            .edit_original_interaction_response(
                &self.token,
                &serde_json::json!({ "embeds": [embed] }),
                attachments,
            )
            .await?;

        Ok(())
    }

    fn make_embed(&self) -> CreateEmbed {
        CreateEmbed::new()
            .title(format!(
                "{} {} - {}",
                S::emoji(),
                self.title.as_deref().unwrap_or_default(),
                S::state_name()
            ))
            .color(S::color())
            .description(self.body.as_deref().unwrap_or_default())
            .author(CreateEmbedAuthor::new(BOT_NAME).icon_url(ICON_URL))
    }
}
