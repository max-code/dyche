mod autocompletes;
mod commands;
mod constants;
pub mod images;
pub mod notifications;
mod utils;

use commands::{
    captains, chips, deadline, differentials, hits, loglevel, register, table, team, transfers,
    unique, whohas,
};

use ::serenity::all::ChannelId;
use fpl_api::FplClient;
use fpl_bot::notifications::Points;
use poise::serenity_prelude as serenity;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::{str::FromStr, sync::Arc, time::Duration};
use tracing::{error, info};
use tracing_subscriber::{prelude::*, reload, EnvFilter};

use fpl_bot::Context;
use fpl_bot::Data;
use fpl_bot::Error;

async fn handle_bot_error(error: poise::FrameworkError<'_, Data, Error>) {
    match &error {
        poise::FrameworkError::Command { error, ctx, .. } => {
            error!(
                "Error in command '{}' by {}: {}\n{:?}",
                ctx.command().name,
                ctx.author().name,
                error,
                error
            );
        }
        other => {
            error!("Framework error: {:?}", other.to_string());
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<(dyn std::error::Error + std::marker::Send + Sync + 'static)>> {
    // Some defaults incase not set in .env
    let env_filter = EnvFilter::try_from_env("RUST_LOG").unwrap_or_else(|_| {
        EnvFilter::default()
            .add_directive("fpl_bot=debug".parse().unwrap())
            .add_directive("fpl_api=debug".parse().unwrap())
            .add_directive("fpl_common=debug".parse().unwrap())
            .add_directive("fpl_db=debug".parse().unwrap())
            .add_directive("serenity=debug".parse().unwrap())
            .add_directive("poise=debug".parse().unwrap())
            .add_directive("sqlx=info".parse().unwrap())
            .add_directive("debug=off".parse().unwrap())
    });

    let (filter, handle) = reload::Layer::new(env_filter);

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    let log_levels = std::sync::Arc::new(handle);

    dotenv::from_filename(".env").ok();
    dotenv::from_filename("../.env").ok();
    let database_url = std::env::var("DATABASE_URL")?;
    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set in .env file");
    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::GUILD_MEMBERS
        | serenity::prelude::GatewayIntents::MESSAGE_CONTENT;

    let options = PgConnectOptions::from_str(&database_url)?
        .application_name("fpl_db")
        .statement_cache_capacity(500);

    let pool = Arc::new(
        PgPoolOptions::new()
            .max_connections(15)
            .min_connections(5)
            .acquire_timeout(Duration::from_secs(5))
            .connect_with(options)
            .await?,
    );
    let client = Arc::new(FplClient::new());

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                register(),
                captains(),
                deadline(),
                whohas(),
                chips(),
                loglevel(),
                hits(),
                table(),
                team(),
                unique(),
                differentials(),
                transfers(),
            ],
            on_error: |error| Box::pin(handle_bot_error(error)),
            allowed_mentions: Some(
                serenity::CreateAllowedMentions::new()
                    .empty_roles()
                    .empty_users(),
            ),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                // Test Guild command registration
                let test_guild_ids = std::env::var("TEST_GUILD_IDS")
                    .expect("Missing TEST_GUILD_IDS")
                    .split(',')
                    .map(|id| {
                        serenity::GuildId::new(
                            id.trim()
                                .parse()
                                .expect("Invalid guild ID in TEST_GUILD_IDS"),
                        )
                    })
                    .collect::<Vec<_>>();

                for guild_id in test_guild_ids {
                    match poise::builtins::register_in_guild(
                        ctx,
                        &framework.options().commands,
                        guild_id,
                    )
                    .await
                    {
                        Ok(_) => info!("Successfully registered commands in guild {}", guild_id),
                        Err(e) => {
                            error!("Failed to register commands in guild {}: {}", guild_id, e)
                        }
                    }
                }

                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                // Live notifs
                let notification_channel = ChannelId::new(
                    // TODO: Actually set up notifications correctly
                    std::env::var("NOTIFICATION_CHANNEL")
                        .unwrap()
                        .parse::<u64>()
                        .unwrap(),
                );

                let live_points_notifications = Arc::new(Points::new(
                    Arc::clone(&pool),
                    Arc::clone(&ctx.http),
                    notification_channel,
                ));

                live_points_notifications.start().await?;

                Ok(Data {
                    pool,
                    client,
                    log_levels,
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
    Ok(())
}
