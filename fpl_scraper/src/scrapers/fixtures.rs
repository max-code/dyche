use crate::error::ScraperError;
use crate::scraper::{Scraper, ScraperOrder, ShouldScrape};
use async_trait::async_trait;
use sqlx::PgPool;
use tracing::{debug, info};

use fpl_api::requests::FixtureRequest;
use fpl_api::FplClient;

use fpl_db::models::Fixture;
use fpl_db::queries::fixture::upsert_fixtures;
use tracing::{debug, info};

pub struct FixturesScraper {
    pool: PgPool,
    client: FplClient,
}

impl FixturesScraper {
    pub fn new(pool: PgPool, client: FplClient) -> Self {
        info!("Creating FixtureScraper");
        Self { pool, client }
    }
}

#[async_trait]
impl Scraper for FixturesScraper {
    fn should_scrape(&self) -> ShouldScrape {
        let result = ShouldScrape::Yes;
        info!("Should Scrape Result: {:?}", result);
        result
    }

    fn name(&self) -> &'static str {
        "FixturesScraper"
    }

    async fn scrape(&self) -> Result<(), ScraperError> {
        let request = FixtureRequest::new();
        let fixtures = self
            .client
            .get(request)
            .await
            .map_err(|e| ScraperError::FplApiError(e.to_string()))?;

        let fixtures_rows: Vec<Fixture> = fixtures.iter().map(|f| f.into()).collect();

        debug!(
            "Got {} fixtures from the API. Converted to {} Fixture rows for upsertion.",
            fixtures.len(),
            fixtures_rows.len()
        );

        upsert_fixtures(&self.pool, &fixtures_rows)
            .await
            .map_err(ScraperError::DatabaseError)?;
        Ok(())
    }

    fn position(&self) -> ScraperOrder {
        ScraperOrder::First
    }
}
