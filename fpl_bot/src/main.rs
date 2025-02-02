mod commands;

use commands::{captains, register};

use poise::serenity_prelude as serenity;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    dotenv::from_filename(".env").ok();
    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set in .env file");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![register(), captains()],
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
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
