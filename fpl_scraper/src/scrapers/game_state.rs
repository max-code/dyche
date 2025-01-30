use crate::error::ScraperError;
use crate::scraper::{Scraper, ScraperOrder, ShouldScrape};
use crate::NoScrapeReason;
use async_trait::async_trait;
use fpl_api::responses::game_state::{ClubOverview, GameWeekOverview, PlayerOverview};
use fpl_db::queries::club::upsert_clubs;
use fpl_db::queries::game_week::{
    upsert_game_week_chip_plays, upsert_game_week_top_elements, upsert_game_weeks,
};
use fpl_db::queries::player::upsert_players;
use sqlx::PgPool;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use tracing::{debug, info};

use fpl_api::requests::GameStateRequest;
use fpl_api::FplClient;

use fpl_db::models::{Club, GameWeek, GameWeekChipPlay, GameWeekTopElement, Player};

pub struct GameStateScraper {
    pool: Arc<PgPool>,
    client: Arc<FplClient>,
    min_scrape_interval: Duration,
    last_scrape: RwLock<Option<SystemTime>>,
}

impl GameStateScraper {
    pub fn new(pool: Arc<PgPool>, client: Arc<FplClient>, min_scrape_interval: Duration) -> Self {
        info!("Creating GameStateScraper");
        Self {
            pool,
            client,
            min_scrape_interval,
            last_scrape: RwLock::new(None),
        }
    }

    async fn handle_game_weeks(
        pool: &PgPool,
        scraper_name: &str,
        game_weeks: &Vec<GameWeekOverview>,
    ) -> Result<(), ScraperError> {
        /*
        GAME WEEKS
         */
        let game_week_rows: Vec<GameWeek> = game_weeks.iter().map(|f| f.into()).collect();

        upsert_game_weeks(pool, &game_week_rows)
            .await
            .map_err(ScraperError::DatabaseError)?;

        /*
        CHIPS
         */
        let game_weeks_chips_rows: Vec<GameWeekChipPlay> = game_weeks
            .iter()
            .map(|gameweek| GameWeekChipPlay::from_overview(gameweek))
            .flatten()
            .collect();
        upsert_game_week_chip_plays(pool, &game_weeks_chips_rows).await?;

        /*
        TOP ELEMENTS
        */
        let game_weeks_top_elements: Vec<GameWeekTopElement> = game_weeks
            .iter()
            .filter_map(|gameweek| GameWeekTopElement::from_overview(gameweek))
            .collect();
        upsert_game_week_top_elements(pool, &game_weeks_top_elements).await?;

        debug!(
            "[{}] Got {} game weeks from the API. Converted to {} GameWeek rows, {} GameWeekChip rows and {} GameWeekTopElement for upsertion.",
            scraper_name,
            game_weeks.len(),
            game_week_rows.len(),
            game_weeks_chips_rows.len(),
            game_weeks_top_elements.len()
        );

        Ok(())
    }

    async fn handle_clubs(
        pool: &PgPool,
        scraper_name: &str,
        clubs: &Vec<ClubOverview>,
    ) -> Result<(), ScraperError> {
        let clubs_rows: Vec<Club> = clubs.iter().map(|f| f.into()).collect();

        upsert_clubs(pool, &clubs_rows)
            .await
            .map_err(ScraperError::DatabaseError)?;

        debug!(
            "[{}] Got {} clubs from the API. Converted to {} Club rows.",
            scraper_name,
            clubs.len(),
            clubs_rows.len(),
        );

        Ok(())
    }

    async fn handle_players(
        pool: &PgPool,
        scraper_name: &str,
        players: &Vec<PlayerOverview>,
    ) -> Result<(), ScraperError> {
        let players_rows: Vec<Player> = players.iter().map(|f| f.into()).collect();

        upsert_players(pool, &players_rows)
            .await
            .map_err(ScraperError::DatabaseError)?;

        debug!(
            "[{}] Got {} clubs from the API. Converted to {} Club rows.",
            scraper_name,
            players.len(),
            players_rows.len(),
        );

        Ok(())
    }
}

#[async_trait]
impl Scraper for GameStateScraper {
    async fn should_scrape(&self) -> ShouldScrape {
        let last_scrape = self.last_scrape.read().await;
        let result;

        match *last_scrape {
            None => result = ShouldScrape::Yes,
            Some(time) => {
                let elapsed_time = SystemTime::now()
                    .duration_since(time)
                    .unwrap_or(Duration::ZERO);

                if elapsed_time >= self.min_scrape_interval {
                    result = ShouldScrape::Yes;
                } else {
                    let remaining_seconds = (self.min_scrape_interval - elapsed_time).as_secs();
                    result = ShouldScrape::No(NoScrapeReason::TimeIntervalNotLapsed(
                        self.min_scrape_interval,
                        remaining_seconds,
                    ));
                }
            }
        }

        debug!("[{}] Should Scrape Result: {:?}", self.name(), result);
        result
    }

    fn name(&self) -> &'static str {
        "GameStateScraper"
    }

    async fn scrape(&self) -> Result<(), ScraperError> {
        let request = GameStateRequest::new();
        let game_state = self.client.get(request).await?;

        GameStateScraper::handle_clubs(&self.pool, self.name(), &game_state.teams).await?;
        GameStateScraper::handle_players(&self.pool, self.name(), &game_state.elements).await?;
        GameStateScraper::handle_game_weeks(&self.pool, self.name(), &game_state.events).await?;

        *self.last_scrape.write().await = Some(SystemTime::now());
        Ok(())
    }

    fn position(&self) -> ScraperOrder {
        ScraperOrder::First
    }
}
