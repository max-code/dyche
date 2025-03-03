use std::time::{Duration, SystemTime};

use crate::error::ScraperError;
use crate::scraper::{Scraper, ScraperOrder, ShouldScrape};
use crate::{with_retry, NoScrapeReason, DEFAULT_MAX_RETRIES};
use async_trait::async_trait;
use fpl_db::models::Transfer;
use fpl_db::queries::transfers::upsert_transfers;
use futures::StreamExt;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use fpl_api::requests::TransfersRequest;
use fpl_api::FplClient;
use fpl_common::types::TeamId;
use fpl_db::queries::team::get_all_team_ids;

pub struct TransfersScraper {
    pool: Arc<PgPool>,
    client: Arc<FplClient>,
    min_scrape_interval: Duration,
    last_scrape: RwLock<Option<SystemTime>>,
}

impl TransfersScraper {
    pub fn new(pool: Arc<PgPool>, client: Arc<FplClient>, min_scrape_interval: Duration) -> Self {
        info!("Creating TransfersScraper");
        Self {
            pool,
            client,
            min_scrape_interval,
            last_scrape: RwLock::new(None),
        }
    }

    async fn process_transfer_request(
        client: Arc<FplClient>,
        team_id: TeamId,
    ) -> Result<Vec<Transfer>, ScraperError> {
        let transfers_response = with_retry(
            || {
                let client_clone = client.clone();
                let team_id_clone = team_id;
                async move { client_clone.get(TransfersRequest::new(team_id_clone)).await }
            },
            DEFAULT_MAX_RETRIES,
        )
        .await?;
        Ok(transfers_response
            .into_iter()
            .map(|t| (&t).into())
            .collect())
    }
}

#[async_trait]
impl Scraper for TransfersScraper {
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
        "TransfersScraper"
    }

    async fn scrape(&self) -> Result<(), ScraperError> {
        let team_ids = get_all_team_ids(&self.pool).await?;

        let mut stream = futures::stream::iter(team_ids.into_iter().map(|team_id| {
            TransfersScraper::process_transfer_request(self.client.clone(), team_id)
        }))
        .buffer_unordered(5);

        let batch_size = 5000;
        let mut transfers_batch = Vec::with_capacity(batch_size);
        let mut total_transfers_processed = 0;
        let mut error_count = 0;

        while let Some(result) = stream.next().await {
            match result {
                Ok(transfers) => {
                    transfers_batch.extend(transfers);
                    if transfers_batch.len() >= batch_size {
                        upsert_transfers(&self.pool, &transfers_batch).await?;
                        total_transfers_processed += transfers_batch.len();
                        transfers_batch.clear();
                    }
                }
                Err(e) => {
                    warn!("Failed to process team {}", e);
                    error_count += 1;
                }
            }
        }

        if !transfers_batch.is_empty() {
            upsert_transfers(&self.pool, &transfers_batch).await?;
            total_transfers_processed += transfers_batch.len();
        }

        debug!(
            "[{}] Successfully processed {} transfers ({} errors)",
            self.name(),
            total_transfers_processed,
            error_count
        );

        *self.last_scrape.write().await = Some(SystemTime::now());
        Ok(())
    }

    fn position(&self) -> ScraperOrder {
        ScraperOrder::Fourth
    }
}
