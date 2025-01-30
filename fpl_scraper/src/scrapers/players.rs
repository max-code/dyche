use std::time::{Duration, SystemTime};

use crate::error::ScraperError;
use crate::scraper::{Scraper, ScraperOrder, ShouldScrape};
use crate::NoScrapeReason;
use async_trait::async_trait;
use fpl_db::models::{PlayerFixtureDb, PlayerHistoryDb, PlayerHistoryPastDb};
use fpl_db::queries::player::{
    get_all_player_ids, upsert_player_fixtures, upsert_player_histories, upsert_player_history_past,
};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

use fpl_api::requests::PlayerRequest;
use fpl_api::FplClient;
use fpl_common::types::PlayerId;

pub struct PlayersScraper {
    pool: Arc<PgPool>,
    client: Arc<FplClient>,
    min_scrape_interval: Duration,
    last_scrape: RwLock<Option<SystemTime>>,
}

impl PlayersScraper {
    pub fn new(pool: Arc<PgPool>, client: Arc<FplClient>, min_scrape_interval: Duration) -> Self {
        info!("Creating PlayersScraper");
        Self {
            pool,
            client,
            min_scrape_interval,
            last_scrape: RwLock::new(None),
        }
    }

    async fn process_player(
        pool: Arc<PgPool>,
        client: Arc<FplClient>,
        player_id: PlayerId,
    ) -> Result<(), ScraperError> {
        let player = client.get(PlayerRequest::new(player_id)).await?;

        // Process fixtures
        let fixtures: Vec<PlayerFixtureDb> = player
            .fixtures
            .iter()
            .map(|f| (player_id, f).into())
            .collect();
        upsert_player_fixtures(&pool, &fixtures).await?;

        // Process history
        let history: Vec<PlayerHistoryDb> = player.history.iter().map(|h| h.into()).collect();
        upsert_player_histories(&pool, &history).await?;

        // Process history past
        let history_past: Vec<PlayerHistoryPastDb> = player
            .history_past
            .iter()
            .map(|f| (player_id, f).into())
            .collect();
        upsert_player_history_past(&pool, &history_past).await?;

        Ok(())
    }
}

#[async_trait]
impl Scraper for PlayersScraper {
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
        "PlayersScraper"
    }

    async fn scrape(&self) -> Result<(), ScraperError> {
        let all_player_ids = get_all_player_ids(&self.pool).await?;

        let player_chunks: Vec<_> = all_player_ids
            .iter()
            .collect::<Vec<_>>()
            .chunks(100)
            .map(|c| c.to_vec())
            .collect();

        for chunk in player_chunks {
            let futures: Vec<_> = chunk
                .into_iter()
                .map(|&player_id| {
                    PlayersScraper::process_player(
                        Arc::clone(&self.pool),
                        Arc::clone(&self.client),
                        player_id,
                    )
                })
                .collect();
            futures::future::join_all(futures).await;
        }

        debug!(
            "[{}] Got {} players from the API. Will try to upsert {} rows.",
            self.name(),
            all_player_ids.len(),
            all_player_ids.len()
        );

        *self.last_scrape.write().await = Some(SystemTime::now());
        Ok(())
    }

    fn position(&self) -> ScraperOrder {
        ScraperOrder::Second
    }
}
