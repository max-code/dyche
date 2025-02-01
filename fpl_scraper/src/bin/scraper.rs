use std::sync::Arc;
use std::time::Duration;

use fpl_api::FplClient;
use fpl_scraper::{
    fixtures::FixturesScraper, game_state::GameStateScraper,
    game_week_players::GameWeekPlayersScraper, mini_leagues::MiniLeaguesScraper,
    players::PlayersScraper, team_game_weeks::TeamGameWeekScraper, teams::TeamsScraper,
    ScraperManager,
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

    // First
    let game_state_scraper =
        GameStateScraper::new(Arc::clone(&pool), Arc::clone(&client), five_minutes);
    manager.register_scraper(game_state_scraper);

    // Second
    let fixtures_scraper =
        FixturesScraper::new(Arc::clone(&pool), Arc::clone(&client), five_minutes);
    manager.register_scraper(fixtures_scraper);

    let teams_scraper = TeamsScraper::new(Arc::clone(&pool), Arc::clone(&client), five_minutes);
    manager.register_scraper(teams_scraper);

    let game_week_players_scraper =
        GameWeekPlayersScraper::new(Arc::clone(&pool), Arc::clone(&client), five_minutes);
    manager.register_scraper(game_week_players_scraper);

    // Third
    let player_scraper = PlayersScraper::new(Arc::clone(&pool), Arc::clone(&client), five_minutes);
    manager.register_scraper(player_scraper);

    let team_game_week_scraper =
        TeamGameWeekScraper::new(Arc::clone(&pool), Arc::clone(&client), five_minutes);
    manager.register_scraper(team_game_week_scraper);

    let mini_league_scraper =
        MiniLeaguesScraper::new(Arc::clone(&pool), Arc::clone(&client), five_minutes);
    manager.register_scraper(mini_league_scraper);

    manager.run().await;
    Ok(())
}
