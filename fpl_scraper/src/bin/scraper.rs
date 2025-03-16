use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use fpl_api::FplClient;
use fpl_scraper::{
    fixtures::FixturesScraper, game_state::GameStateScraper,
    game_week_players::GameWeekPlayersScraper, mini_leagues::MiniLeaguesScraper,
    player_images::PlayerPhotosScraper, players::PlayersScraper,
    team_game_weeks::TeamGameWeekScraper, teams::TeamsScraper, transfers::TransfersScraper,
    ScraperManager,
};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    dotenv::from_filename("../.env").ok();
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

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

    info!("Scraper Start: DB Pool, Client and .env file loaded.");

    let mut manager = ScraperManager::new();

    let fifteen_seconds = Duration::from_secs(15);
    let one_minute = Duration::from_secs(60);
    let five_minutes = Duration::from_secs(60 * 5);
    let one_day = Duration::from_secs(60 * 60 * 24);

    // First
    let game_state_scraper =
        GameStateScraper::new(Arc::clone(&pool), Arc::clone(&client), one_minute);
    manager.register_scraper(game_state_scraper);

    // Second
    let fixtures_scraper = FixturesScraper::new(Arc::clone(&pool), Arc::clone(&client), one_minute);
    manager.register_scraper(fixtures_scraper);

    let teams_scraper = TeamsScraper::new(Arc::clone(&pool), Arc::clone(&client), five_minutes);
    manager.register_scraper(teams_scraper);

    let game_week_players_scraper =
        GameWeekPlayersScraper::new(Arc::clone(&pool), Arc::clone(&client), fifteen_seconds);
    manager.register_scraper(game_week_players_scraper);

    // Third
    let player_scraper = PlayersScraper::new(Arc::clone(&pool), Arc::clone(&client), one_minute);
    manager.register_scraper(player_scraper);

    let team_game_week_scraper =
        TeamGameWeekScraper::new(Arc::clone(&pool), Arc::clone(&client), five_minutes);
    manager.register_scraper(team_game_week_scraper);

    let mini_league_scraper =
        MiniLeaguesScraper::new(Arc::clone(&pool), Arc::clone(&client), five_minutes);
    manager.register_scraper(mini_league_scraper);

    let transfers_scraper =
        TransfersScraper::new(Arc::clone(&pool), Arc::clone(&client), five_minutes);
    manager.register_scraper(transfers_scraper);

    // Fourth
    let photos_scraper = PlayerPhotosScraper::new(Arc::clone(&pool), Arc::clone(&client), one_day);
    manager.register_scraper(photos_scraper);

    manager.run().await;
    Ok(())
}
