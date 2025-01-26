use fpl_api::FplClient;
use fpl_scraper::{FixturesScraper, ScraperManager, ScraperOrder};
use sqlx::PgPool;
use tracing::info;
use tracing::{debug, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    dotenv::from_filename("../.env").ok();
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    let pool = PgPool::connect(&database_url).await?;
    let client = FplClient::new();

    info!("Scraper Start: DB Pool, Client and .env file loaded.");

    let mut manager = ScraperManager::new();

    let fixtures_scraper = FixturesScraper::new(pool, client);
    manager.register_scraper(ScraperOrder::First, fixtures_scraper);

    manager.run().await;
    Ok(())
}
