use std::sync::Arc;
use std::time::Duration;

use fpl_api::FplClient;
use fpl_scraper::{
    fixtures::FixturesScraper, game_state::GameStateScraper, ScraperManager, ScraperOrder,
};
use sqlx::PgPool;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    dotenv::from_filename("../.env").ok();
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    let pool = Arc::new(PgPool::connect(&database_url).await?);
    let client = Arc::new(FplClient::new());

    info!("Scraper Start: DB Pool, Client and .env file loaded.");

    let mut manager = ScraperManager::new();

    let five_minutes = Duration::from_secs(300);

    // No dependencies
    let game_state_scraper =
        GameStateScraper::new(Arc::clone(&pool), Arc::clone(&client), five_minutes);
    manager.register_scraper(ScraperOrder::First, game_state_scraper);

    // Depends on clubs and game_weeks
    let fixtures_scraper =
        FixturesScraper::new(Arc::clone(&pool), Arc::clone(&client), five_minutes);
    manager.register_scraper(ScraperOrder::Second, fixtures_scraper);

    manager.run().await;
    Ok(())
}
