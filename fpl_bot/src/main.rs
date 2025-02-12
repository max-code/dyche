mod commands;
mod constants;
mod utils;

use commands::{captains, chips, deadline, register, whohas};

use ::serenity::all::CommandOptionChoice;
use fpl_api::FplClient;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::{error, info};

use poise::serenity_prelude as serenity;

struct Data {
    pool: Arc<PgPool>,
    client: Arc<FplClient>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn handle_bot_error(err: poise::FrameworkError<'_, Data, Error>) {
    error!("{}", err);
}

#[tokio::main]
async fn main() -> Result<(), Box<(dyn std::error::Error + std::marker::Send + Sync + 'static)>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    dotenv::from_filename(".env").ok();
    dotenv::from_filename("../.env").ok();
    let database_url = std::env::var("DATABASE_URL")?;
    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set in .env file");
    let intents = serenity::GatewayIntents::non_privileged();

    let pool = Arc::new(PgPool::connect(&database_url).await?);
    let client = Arc::new(FplClient::new());

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![register(), captains(), deadline(), whohas(), chips()],
            on_error: |error| Box::pin(handle_bot_error(error)),
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                // Test Guild command registration
                let test_guild_id = serenity::GuildId::new(
                    std::env::var("TEST_GUILD_ID")
                        .expect("Missing TEST_GUILD_ID")
                        .parse()
                        .expect("Invalid TEST_GUILD_ID"),
                );

                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    test_guild_id,
                )
                .await?;

                // Local
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { pool, client })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
    Ok(())
}
