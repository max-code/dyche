use poise::serenity_prelude as serenity;
use poise::serenity_prelude::CreateEmbed;
use serenity::all::{CommandInteraction, CreateEmbedAuthor, Http};
use std::sync::Arc;
use std::{
    marker::{PhantomData, Send, Sync},
    path::{Path, PathBuf},
};
use tracing::{debug, warn};

use crate::constants::text_response::MAX_ROWS_PER_PAGE;
use crate::{Context, Error};
pub trait State {}
pub trait Sendable: State {}
pub trait ReadyForSend: Sendable {}

// Concrete state types
pub struct Uninitialised;
pub struct ProcessingState;
pub struct SuccessState;
pub struct ErrorState;
pub struct SentState;

impl State for Uninitialised {}
impl State for ProcessingState {}
impl State for SuccessState {}
impl State for ErrorState {}
impl State for SentState {}

impl Sendable for ProcessingState {}
impl Sendable for SuccessState {}
impl Sendable for ErrorState {}

impl ReadyForSend for ProcessingState {}
impl ReadyForSend for SuccessState {}
impl ReadyForSend for ErrorState {}

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

#[derive(Clone, Debug)]
pub struct EmbedPage {
    pub rows: Vec<String>,
    pub image: Option<PathBuf>,
}

impl EmbedPage {
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            image: None,
        }
    }

    pub fn add_row(mut self, row: impl Into<String>) -> Self {
        self.rows.push(row.into());
        self
    }

    pub fn add_rows(mut self, rows: Vec<impl Into<String>>) -> Self {
        for row in rows {
            self.rows.push(row.into());
        }
        self
    }

    pub fn with_image(mut self, image_path: impl AsRef<Path>) -> Self {
        self.image = Some(image_path.as_ref().to_path_buf());
        self
    }

    fn content(&self) -> String {
        self.rows.join("\n")
    }
}

const ICON_URL: &str = "https://cdn.discordapp.com/app-icons/1275927396129964054/e1ed26e9fed1b459ec6f19b2712898e6.png?size=512";
const BOT_NAME: &str = "FPL Bot ⚽️";

pub struct Embed<'a, S: State> {
    title: Option<String>,
    pages: Vec<EmbedPage>,
    state: PhantomData<S>,
    sent: bool,
    interaction: &'a CommandInteraction,
    http: Arc<Http>,
    token: String,
    ctx_id: u64,
    ctx: Context<'a>,
}

impl<'a, S: State> Clone for Embed<'a, S>
where
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            title: self.title.clone(),
            pages: self.pages.clone(),
            state: PhantomData,
            sent: self.sent,
            interaction: self.interaction,
            http: Arc::clone(&self.http),
            token: self.token.clone(),
            ctx_id: self.ctx_id,
            ctx: self.ctx,
        }
    }
}

impl<'a> Embed<'a, Uninitialised> {
    pub fn from_ctx(ctx: Context<'a>) -> Result<Self, Error> {
        let (interaction, http, token) = match ctx {
            Context::Application(ctx) => {
                let interaction = ctx.interaction;
                let http = ctx.serenity_context().http.clone();
                let token = interaction.token.clone();
                (interaction, http, token)
            }
            _ => return Err("Couldn't fetch interaction, http or token from ctx".into()),
        };

        Ok(Self {
            title: None,
            pages: Vec::new(),
            state: PhantomData,
            interaction,
            http,
            token,
            sent: false,
            ctx_id: ctx.id(),
            ctx,
        })
    }

    fn transition<T: State>(self) -> Embed<'a, T> {
        Embed {
            title: None,
            pages: vec![],
            state: PhantomData,
            http: self.http,
            interaction: self.interaction,
            sent: self.sent,
            token: self.token,
            ctx_id: self.ctx_id,
            ctx: self.ctx,
        }
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

impl<'a> Embed<'a, SentState> {
    pub fn processing(self) -> Embed<'a, ProcessingState> {
        self.transition()
    }

    pub fn error(self) -> Embed<'a, ErrorState> {
        self.transition()
    }

    pub fn success(self) -> Embed<'a, SuccessState> {
        self.transition()
    }

    fn transition<T: State>(self) -> Embed<'a, T> {
        Embed {
            title: None,
            pages: vec![],
            state: PhantomData,
            http: self.http,
            interaction: self.interaction,
            sent: self.sent,
            token: self.token,
            ctx_id: self.ctx_id,
            ctx: self.ctx,
        }
    }
}

impl<'a, S: ReadyForSend + EmbedState + Sync + Send> Embed<'a, S> {
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    fn transition(self) -> Embed<'a, SentState> {
        Embed {
            title: None,
            pages: vec![],
            state: PhantomData,
            http: self.http,
            interaction: self.interaction,
            sent: true,
            token: self.token,
            ctx_id: self.ctx_id,
            ctx: self.ctx,
        }
    }

    fn create_base_embed(&self, page_content: &str, current_page: usize) -> CreateEmbed {
        let page_count = self.pages.len();
        let title = match &self.title {
            Some(title) => format!("{} {}", S::emoji(), title),
            None => format!("{} {}", S::emoji(), S::state_name()),
        };

        let mut embed = CreateEmbed::new()
            .title(title)
            .color(S::color())
            .description(page_content);

        embed = embed.author(CreateEmbedAuthor::new(BOT_NAME).icon_url(ICON_URL));

        if page_count > 1 {
            embed = embed.footer(serenity::all::CreateEmbedFooter::new(format!(
                "📖 Page {}/{}",
                current_page + 1,
                page_count
            )));
        }

        embed
    }

    pub async fn send(mut self) -> Result<Embed<'a, SentState>, Error> {
        if self.pages.is_empty() {
            debug!(
                "Trying to send an embed with no pages ({}). Adding empty page.",
                self.ctx_id
            );
            self.pages.push(EmbedPage::new());
        }
        if self.pages.len() < 2 {
            self.send_or_edit_single_page().await?
        } else {
            self.send_or_edit_paginated().await?
        }
        Ok(self.transition())
    }

    async fn send_or_edit_single_page(&mut self) -> Result<(), Error> {
        let (embed, attachments) = self.prepare_embed_with_attachments(0).await?;

        if self.sent {
            self.http
                .edit_original_interaction_response(
                    &self.token,
                    &serenity::json::json!({
                        "embeds": [embed],
                        "components": []
                    }),
                    attachments,
                )
                .await
                .map(|_| ()) // Convert Message to () to match the expected return type
                .map_err(|e| format!("Failed to edit embed: {}", e).into())
        } else {
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
    }

    async fn send_or_edit_paginated(&mut self) -> Result<(), Error> {
        let current_page = 0;
        let (embed, attachments) = self.prepare_embed_with_attachments(current_page).await?;
        let components = self.create_pagination_buttons();

        if self.sent {
            self.http
                .edit_original_interaction_response(
                    &self.token,
                    &serenity::json::json!({
                        "embeds": [embed],
                        "components": [components]
                    }),
                    attachments,
                )
                .await?;
        } else {
            let mut response_message = serenity::builder::CreateInteractionResponseMessage::new()
                .embed(embed)
                .components(vec![components]);

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
        }
        self.handle_pagination(current_page).await
    }

    async fn handle_pagination(&self, current_page: usize) -> Result<(), Error> {
        let ctx_id = self.ctx_id;
        let mut current_page = current_page;
        let page_count = self.pages.len();
        let prev_button_id = format!("{}prev", ctx_id);
        let next_button_id = format!("{}next", ctx_id);

        while let Some(press) = serenity::collector::ComponentInteractionCollector::new(self.ctx)
            .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
            .timeout(std::time::Duration::from_secs(3600)) // Standardize to 1 hour
            .await
        {
            if press.data.custom_id == next_button_id {
                current_page = (current_page + 1) % page_count;
            } else if press.data.custom_id == prev_button_id {
                current_page = current_page.checked_sub(1).unwrap_or(page_count - 1);
            } else {
                continue;
            }

            let (embed, attachments) = self.prepare_embed_with_attachments(current_page).await?;

            let mut response_message =
                serenity::builder::CreateInteractionResponseMessage::new().embed(embed);

            if !attachments.is_empty() {
                response_message = response_message.files(attachments);
            }

            press
                .create_response(
                    &self.ctx.serenity_context(),
                    serenity::builder::CreateInteractionResponse::UpdateMessage(response_message),
                )
                .await?;
        }

        Ok(())
    }

    fn create_pagination_buttons(&self) -> serenity::builder::CreateActionRow {
        let prev_button_id = format!("{}prev", self.ctx_id);
        let next_button_id = format!("{}next", self.ctx_id);

        serenity::builder::CreateActionRow::Buttons(vec![
            serenity::builder::CreateButton::new(&prev_button_id)
                .emoji('◀')
                .style(serenity::ButtonStyle::Secondary),
            serenity::builder::CreateButton::new(&next_button_id)
                .emoji('▶')
                .style(serenity::ButtonStyle::Secondary),
        ])
    }

    async fn prepare_embed_with_attachments(
        &self,
        page_index: usize,
    ) -> Result<(CreateEmbed, Vec<serenity::builder::CreateAttachment>), Error> {
        let page = &self.pages[page_index];
        let mut embed = self.create_base_embed(&page.content(), page_index);
        let mut attachments = Vec::new();

        if let Some(image_path) = &page.image {
            match serenity::builder::CreateAttachment::path(image_path).await {
                Ok(attachment) => {
                    embed = embed.image(format!("attachment://{}", attachment.filename));
                    attachments.push(attachment);
                }
                Err(e) => {
                    warn!(
                        "Failed to create attachment: {} - {}",
                        image_path.display(),
                        e
                    );
                }
            }
        }

        Ok((embed, attachments))
    }
}

impl<'a> Embed<'a, ProcessingState> {
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.pages.push(EmbedPage::new().add_row(body));
        self
    }
}

impl<'a> Embed<'a, ErrorState> {
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.pages.push(EmbedPage::new().add_row(body));
        self
    }
}

impl<'a> Embed<'a, SuccessState> {
    pub fn add_page(mut self, page: EmbedPage) -> Self {
        self.pages.push(page);
        self
    }

    pub fn add_pages(mut self, pages: Vec<EmbedPage>) -> Self {
        self.pages.extend(pages);
        self
    }

    pub fn add_pages_from_strings(
        mut self,
        strings: Vec<impl Into<String>>,
        rows_per_page: Option<usize>,
    ) -> Self {
        let rows_per_page = rows_per_page.unwrap_or(MAX_ROWS_PER_PAGE);

        let strings: Vec<String> = strings.into_iter().map(|s| s.into()).collect();
        for chunk in strings.chunks(rows_per_page) {
            let page = EmbedPage::new().add_rows(chunk.to_vec());
            self.pages.push(page);
        }
        self
    }

    pub fn add_pages_from_images(mut self, images: Vec<impl AsRef<Path>>) -> Self {
        for image in images {
            let page = EmbedPage::new().with_image(image);
            self.pages.push(page);
        }
        self
    }
}
