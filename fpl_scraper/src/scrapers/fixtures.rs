use std::time::{Duration, SystemTime};

use crate::error::ScraperError;
use crate::scraper::{Scraper, ScraperOrder, ShouldScrape};
use crate::NoScrapeReason;
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

use fpl_api::requests::FixtureRequest;
use fpl_api::FplClient;

use fpl_db::models::{Bonus, Fixture};
use fpl_db::queries::fixture::{upsert_bonuses, upsert_fixtures};

pub struct FixturesScraper {
    pool: Arc<PgPool>,
    client: Arc<FplClient>,
    min_scrape_interval: Duration,
    last_scrape: RwLock<Option<SystemTime>>,
}

impl FixturesScraper {
    pub fn new(pool: Arc<PgPool>, client: Arc<FplClient>, min_scrape_interval: Duration) -> Self {
        info!("Creating FixtureScraper");
        Self {
            pool,
            client,
            min_scrape_interval,
            last_scrape: RwLock::new(None),
        }
    }
}

#[async_trait]
impl Scraper for FixturesScraper {
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
        "FixturesScraper"
    }

    async fn scrape(&self) -> Result<(), ScraperError> {
        let request = FixtureRequest::new();
        let fixtures = self.client.get(request).await?;

        let bonuses = fixtures
            .iter()
            .flat_map(|fixture| fixture.bonuses.iter())
            .map(Bonus::from)
            .collect::<Vec<Bonus>>();

        debug!(
            "[{}] Got {} Fixtures from the API. Converted to {} Bonus rows for upsertion.",
            self.name(),
            fixtures.len(),
            bonuses.len()
        );
        upsert_bonuses(&self.pool, &bonuses)
            .await
            .map_err(ScraperError::DatabaseError)?;

        let fixtures_rows: Vec<Fixture> = fixtures.iter().map(|f| f.into()).collect();

        debug!(
            "[{}] Got {} fixtures from the API. Converted to {} Fixture rows for upsertion.",
            self.name(),
            fixtures.len(),
            fixtures_rows.len()
        );

        upsert_fixtures(&self.pool, &fixtures_rows)
            .await
            .map_err(ScraperError::DatabaseError)?;

        *self.last_scrape.write().await = Some(SystemTime::now());
        Ok(())
    }

    fn position(&self) -> ScraperOrder {
        ScraperOrder::Second
    }
}
