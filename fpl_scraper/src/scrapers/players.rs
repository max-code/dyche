use std::time::{Duration, SystemTime};

use crate::error::ScraperError;
use crate::scraper::{Scraper, ScraperOrder, ShouldScrape};
use crate::NoScrapeReason;
use async_trait::async_trait;
use fpl_db::models::{PlayerFixtureDb, PlayerHistoryDb, PlayerHistoryPastDb};
use fpl_db::queries::player::{
    get_all_player_ids, upsert_player_fixtures, upsert_player_histories, upsert_player_history_past,
};
use futures::StreamExt;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

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
        client: Arc<FplClient>,
        player_id: PlayerId,
    ) -> Result<
        (
            Vec<PlayerFixtureDb>,
            Vec<PlayerHistoryDb>,
            Vec<PlayerHistoryPastDb>,
        ),
        ScraperError,
    > {
        let player = client.get(PlayerRequest::new(player_id)).await?;

        // Process fixtures
        let fixtures: Vec<PlayerFixtureDb> = player
            .fixtures
            .iter()
            .map(|f| (player_id, f).into())
            .collect();

        // Process history
        let history: Vec<PlayerHistoryDb> = player.history.iter().map(|h| h.into()).collect();

        // Process history past
        let history_past: Vec<PlayerHistoryPastDb> = player
            .history_past
            .iter()
            .map(|f| (player_id, f).into())
            .collect();

        Ok((fixtures, history, history_past))
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
        let chunk_size = 100;

        for chunk in all_player_ids.chunks(chunk_size) {
            let chunk = chunk.to_vec();

            let mut stream = futures::stream::iter(chunk.into_iter().map(|player_id| {
                PlayersScraper::process_player(Arc::clone(&self.client), player_id)
            }))
            .buffer_unordered(5);

            let mut player_fixtures = Vec::new();
            let mut player_history = Vec::new();
            let mut player_history_past = Vec::new();

            while let Some(result) = stream.next().await {
                let response = match result {
                    Ok(response) => response,
                    Err(e) => {
                        warn!("{}", e);
                        continue;
                    }
                };

                let (fixtures, history, history_past) = response;

                player_fixtures.extend(fixtures);
                player_history.extend(history);
                player_history_past.extend(history_past);
            }

            upsert_player_fixtures(&self.pool, &player_fixtures).await?;
            upsert_player_histories(&self.pool, &player_history).await?;
            upsert_player_history_past(&self.pool, &player_history_past).await?;
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
