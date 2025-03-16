use crate::error::ScraperError;
use crate::scraper::{Scraper, ScraperOrder, ShouldScrape};
use crate::NoScrapeReason;
use async_trait::async_trait;
use fpl_db::queries::player::get_all_player_codes;
use futures::StreamExt;
use sqlx::PgPool;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use fpl_api::requests::PlayerPhotoRequest;
use fpl_api::FplClient;

pub struct PlayerPhotosScraper {
    pool: Arc<PgPool>,
    client: Arc<FplClient>,
    min_scrape_interval: Duration,
    last_scrape: RwLock<Option<SystemTime>>,
}

impl PlayerPhotosScraper {
    pub fn new(pool: Arc<PgPool>, client: Arc<FplClient>, min_scrape_interval: Duration) -> Self {
        info!("Creating PlayerPhotosScraper");
        Self {
            pool,
            client,
            min_scrape_interval,
            last_scrape: RwLock::new(None),
        }
    }

    async fn process_photo_request(
        client: Arc<FplClient>,
        player_code: u32,
    ) -> Result<(), ScraperError> {
        client
            .get(PlayerPhotoRequest::new(
                player_code,
                fpl_common::paths::get_player_image_path(player_code),
            ))
            .await
            .map_err(ScraperError::FplApiError)
    }
}

#[async_trait]
impl Scraper for PlayerPhotosScraper {
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
        "PlayerPhotosScraper"
    }

    async fn scrape(&self) -> Result<(), ScraperError> {
        let all_player_codes = get_all_player_codes(&self.pool).await?;

        let mut stream = futures::stream::iter(all_player_codes.into_iter().map(|player_code| {
            PlayerPhotosScraper::process_photo_request(self.client.clone(), player_code)
        }))
        .buffer_unordered(10);

        let mut error_count = 0;
        let mut photos_processed = 0;

        while let Some(result) = stream.next().await {
            photos_processed += 1;
            if let Err(err) = result {
                warn!("Failed to process team {}", err);
                error_count += 1;
            }
        }

        debug!(
            "[{}] Successfully processed {} photos ({} errors)",
            self.name(),
            photos_processed,
            error_count
        );

        *self.last_scrape.write().await = Some(SystemTime::now());
        Ok(())
    }

    fn position(&self) -> ScraperOrder {
        ScraperOrder::Fourth
    }
}
