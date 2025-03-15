// lib.rs
pub mod autocompletes;
pub mod commands;
pub mod constants;
pub mod images;
pub mod notifications;
pub mod utils;

// Re-export important types for easy access
pub use crate::utils::embed::Embed;

// Define core types that should be accessible throughout the project
use fpl_api::FplClient;
use sqlx::PgPool;
use std::sync::Arc;
use tracing_subscriber::reload::Handle;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Registry;

pub struct Data {
    pub pool: Arc<PgPool>,
    pub client: Arc<FplClient>,
    pub log_levels: Arc<Handle<EnvFilter, Registry>>,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
