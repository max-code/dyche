use poise::serenity_prelude::CreateEmbed;
use serenity::all::CreateEmbedAuthor;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct Processing;
#[derive(Clone)]
pub struct Error;
#[derive(Clone)]
pub struct Success;

pub trait EmbedState {
    fn emoji() -> &'static str;
    fn color() -> (u8, u8, u8);
    fn state_name() -> &'static str;
}

impl EmbedState for Processing {
    fn emoji() -> &'static str {
        "<a:loading:1286983242398040125>"
    }
    fn color() -> (u8, u8, u8) {
        (55, 2, 60)
    }
    fn state_name() -> &'static str {
        "Processing"
    }
}

impl EmbedState for Error {
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

impl EmbedState for Success {
    fn emoji() -> &'static str {
        "<:success:1286991961257148437>"
    }
    fn color() -> (u8, u8, u8) {
        (0, 255, 135)
    } // Green
    fn state_name() -> &'static str {
        "Success"
    }
}

#[derive(Clone)]
pub struct EmbedBuilder<S: EmbedState> {
    command: String,
    embed: CreateEmbed,
    _state: PhantomData<S>,
}

impl<S: EmbedState> EmbedBuilder<S> {
    pub fn build(self) -> CreateEmbed {
        self.embed
    }
}

const ICON_URL: &str = "https://cdn.discordapp.com/app-icons/1275927396129964054/e1ed26e9fed1b459ec6f19b2712898e6.png?size=512";
const BOT_NAME: &str = "FPL Bot ⚽️";

impl EmbedBuilder<Processing> {
    pub fn new(command: impl Into<String>, description: &str) -> Self {
        let command_str = command.into();
        Self {
            command: command_str.clone(),
            embed: CreateEmbed::new()
                .title(format!(
                    "{} {} - {}",
                    Processing::emoji(),
                    command_str,
                    Processing::state_name()
                ))
                .color(Processing::color())
                .description(description)
                .author(CreateEmbedAuthor::new(BOT_NAME).icon_url(ICON_URL)),
            _state: PhantomData,
        }
    }

    pub fn update(self, description: &str) -> Self {
        Self {
            embed: self.embed.description(description),
            ..self
        }
    }

    pub fn error(self, description: &str) -> EmbedBuilder<Error> {
        EmbedBuilder {
            command: self.command.clone(),
            embed: CreateEmbed::new()
                .title(format!(
                    "{} {} - {}",
                    Error::emoji(),
                    self.command,
                    Error::state_name()
                ))
                .color(Error::color())
                .description(description)
                .author(CreateEmbedAuthor::new(BOT_NAME).icon_url(ICON_URL)),
            _state: PhantomData,
        }
    }

    pub fn success(self, description: &str) -> EmbedBuilder<Success> {
        EmbedBuilder {
            command: self.command.clone(),
            embed: CreateEmbed::new()
                .title(format!(
                    "{} {} - {}",
                    Success::emoji(),
                    self.command,
                    Success::state_name()
                ))
                .color(Success::color())
                .description(description)
                .author(CreateEmbedAuthor::new(BOT_NAME).icon_url(ICON_URL)),
            _state: PhantomData,
        }
    }
}

impl EmbedBuilder<Error> {}
impl EmbedBuilder<Success> {}
