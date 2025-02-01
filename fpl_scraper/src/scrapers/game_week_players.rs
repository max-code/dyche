use std::time::{Duration, SystemTime};

use crate::error::ScraperError;
use crate::scraper::{Scraper, ScraperOrder, ShouldScrape};
use crate::NoScrapeReason;
use async_trait::async_trait;
use fpl_db::models::GameWeekPlayerDb;
use fpl_db::queries::game_week_player::upsert_game_week_players;
use futures::StreamExt;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use fpl_api::requests::GameWeekPlayersRequest;
use fpl_api::FplClient;
use fpl_common::types::GameWeekId;

pub struct GameWeekPlayersScraper {
    pool: Arc<PgPool>,
    client: Arc<FplClient>,
    min_scrape_interval: Duration,
    last_scrape: RwLock<Option<SystemTime>>,
}

impl GameWeekPlayersScraper {
    pub fn new(pool: Arc<PgPool>, client: Arc<FplClient>, min_scrape_interval: Duration) -> Self {
        info!("Creating GameWeekPlayersScraper");
        Self {
            pool,
            client,
            min_scrape_interval,
            last_scrape: RwLock::new(None),
        }
    }

    async fn process_game_week_players(
        client: Arc<FplClient>,
        game_week_id: GameWeekId,
    ) -> Result<Vec<GameWeekPlayerDb>, ScraperError> {
        let game_week_response = client
            .get(GameWeekPlayersRequest::new(game_week_id))
            .await?;
        Ok(game_week_response
            .elements
            .into_iter()
            .map(|gwp| (game_week_id, gwp).into())
            .collect())
    }
}

#[async_trait]
impl Scraper for GameWeekPlayersScraper {
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
        "GameWeekPlayersScraper"
    }

    async fn scrape(&self) -> Result<(), ScraperError> {
        let mut stream = futures::stream::iter(GameWeekId::all_weeks_iter().map(|game_week_id| {
            GameWeekPlayersScraper::process_game_week_players(self.client.clone(), game_week_id)
        }))
        .buffer_unordered(20);

        while let Some(result) = stream.next().await {
            let response = match result {
                Ok(response) => response,
                Err(e) => {
                    warn!("{}", e);
                    continue;
                }
            };

            upsert_game_week_players(&self.pool, &response).await?;
        }

        *self.last_scrape.write().await = Some(SystemTime::now());
        Ok(())
    }

    fn position(&self) -> ScraperOrder {
        ScraperOrder::Second
    }
}
