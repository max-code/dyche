use std::time::{Duration, SystemTime};

use crate::error::ScraperError;
use crate::scraper::{Scraper, ScraperOrder, ShouldScrape};
use crate::{with_retry, NoScrapeReason, DEFAULT_MAX_RETRIES};
use async_trait::async_trait;
use fpl_db::models::Team;
use futures::StreamExt;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use fpl_api::requests::TeamRequest;
use fpl_api::FplClient;
use fpl_common::types::TeamId;
use fpl_db::queries::team::{get_all_team_ids, upsert_teams};

pub struct TeamsScraper {
    pool: Arc<PgPool>,
    client: Arc<FplClient>,
    min_scrape_interval: Duration,
    last_scrape: RwLock<Option<SystemTime>>,
}

impl TeamsScraper {
    pub fn new(pool: Arc<PgPool>, client: Arc<FplClient>, min_scrape_interval: Duration) -> Self {
        info!("Creating TeamsScraper");
        Self {
            pool,
            client,
            min_scrape_interval,
            last_scrape: RwLock::new(None),
        }
    }

    async fn process_teams_request(
        client: Arc<FplClient>,
        team_id: TeamId,
    ) -> Result<Team, ScraperError> {
        let team_response = with_retry(
            || {
                let client_clone = client.clone();
                let team_id_clone = team_id;
                async move { client_clone.get(TeamRequest::new(team_id_clone)).await }
            },
            DEFAULT_MAX_RETRIES,
        )
        .await?;

        Ok((&team_response).into())
    }
}

#[async_trait]
impl Scraper for TeamsScraper {
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
        "TeamsScraper"
    }

    async fn scrape(&self) -> Result<(), ScraperError> {
        let team_ids = get_all_team_ids(&self.pool).await?;

        let mut stream = futures::stream::iter(
            team_ids
                .into_iter()
                .map(|team_id| TeamsScraper::process_teams_request(self.client.clone(), team_id)),
        )
        .buffer_unordered(5);

        let batch_size = 1000;
        let mut teams_batch = Vec::with_capacity(batch_size);
        let mut total_teams_processed = 0;
        let mut error_count = 0;

        while let Some(result) = stream.next().await {
            match result {
                Ok(team) => {
                    teams_batch.push(team);
                    if teams_batch.len() >= batch_size {
                        upsert_teams(&self.pool, &teams_batch).await?;
                        total_teams_processed += teams_batch.len();
                        teams_batch.clear();
                    }
                }
                Err(e) => {
                    warn!("Failed to process team {}", e);
                    error_count += 1;
                }
            }
        }

        if !teams_batch.is_empty() {
            upsert_teams(&self.pool, &teams_batch).await?;
            total_teams_processed += teams_batch.len();
        }

        debug!(
            "[{}] Successfully processed {} teams ({} errors)",
            self.name(),
            total_teams_processed,
            error_count
        );

        *self.last_scrape.write().await = Some(SystemTime::now());
        Ok(())
    }

    fn position(&self) -> ScraperOrder {
        ScraperOrder::Second
    }
}
