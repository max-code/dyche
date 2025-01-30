use std::time::{Duration, SystemTime};

use crate::error::ScraperError;
use crate::scraper::{Scraper, ScraperOrder, ShouldScrape};
use crate::NoScrapeReason;
use async_trait::async_trait;
use fpl_db::models::Team;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

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
        let team_response = client.get(TeamRequest::new(team_id)).await?;
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
        let team_ids = get_all_team_ids(&self.pool)
            .await
            .map_err(|e| ScraperError::DatabaseError(e))?;

        let team_id_chunks: Vec<_> = team_ids.chunks(100).map(|c| c.to_vec()).collect();
        let mut all_teams = Vec::with_capacity(team_ids.len());

        for chunk in team_id_chunks {
            let futures: Vec<_> = chunk
                .into_iter()
                .map(|team_id| TeamsScraper::process_teams_request(self.client.clone(), team_id))
                .collect();
            let results = futures::future::join_all(futures).await;

            let teams = results.into_iter().collect::<Result<Vec<_>, _>>()?;

            all_teams.extend(teams);
        }

        upsert_teams(&self.pool, &all_teams).await?;

        debug!(
            "[{}] Got {} teams from the API. Converted to {} Team rows for upsertion.",
            self.name(),
            all_teams.len(),
            all_teams.len()
        );

        *self.last_scrape.write().await = Some(SystemTime::now());
        Ok(())
    }

    fn position(&self) -> ScraperOrder {
        ScraperOrder::Second
    }
}
