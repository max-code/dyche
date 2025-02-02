use std::{collections::HashMap, time::Duration};

use crate::error::ScraperError;
use async_trait::async_trait;
use strum::{EnumIter, IntoEnumIterator};
use tokio::time;
use tracing::{error, info, instrument, trace, warn};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, PartialOrd, Ord, EnumIter)]
pub enum ScraperOrder {
    First,
    Second,
    Third,
    Fourth,
}

#[derive(Debug)]
pub enum ShouldScrape {
    Yes,
    No(NoScrapeReason),
}

#[derive(Debug)]
pub enum NoScrapeReason {
    TimeIntervalNotLapsed(Duration, u64),
}

impl NoScrapeReason {
    fn message(&self) -> String {
        match self {
            Self::TimeIntervalNotLapsed(interval, seconds) => format!(
                "Minimum time interval ({:?}) between scrapes not exceeded. {} seconds remaining.",
                interval, seconds
            ),
        }
    }
}

#[async_trait]
pub trait Scraper: Send + Sync {
    fn name(&self) -> &'static str;
    async fn should_scrape(&self) -> ShouldScrape;
    async fn scrape(&self) -> Result<(), ScraperError>;
    fn position(&self) -> ScraperOrder;
}

pub struct ScraperManager {
    scrapers: HashMap<ScraperOrder, Vec<Box<dyn Scraper>>>,
}
type ScraperResult = Result<(), Vec<(usize, ScraperError)>>;

impl ScraperManager {
    #[instrument]
    pub fn new() -> Self {
        info!("Initializing ScraperManager");
        Self {
            scrapers: HashMap::default(),
        }
    }

    #[instrument(skip(self, scraper))]
    pub fn register_scraper<S>(&mut self, scraper: S)
    where
        S: Scraper + 'static,
    {
        info!(
            "Registering scraper {} with order {:?}",
            scraper.name(),
            scraper.position()
        );
        self.scrapers
            .entry(scraper.position())
            .or_insert_with(Vec::new)
            .push(Box::new(scraper));
    }

    #[instrument(skip(self))]
    pub async fn run(&self) {
        info!("Starting scraper manager");
        let mut interval = time::interval(Duration::from_secs(10));
        loop {
            interval.tick().await;
            info!("ℹ️ Running all scrapers");
            trace!("Scrapers Tick");
            match self.process_all_scrapers().await {
                Ok(_) => info!("✅ All scrapers completed successfully"),
                Err(errors) => {
                    error!(
                        "❌ Failed to process scrapers. {} errors occurred",
                        errors.len()
                    );
                }
            }
        }
    }

    async fn process_all_scrapers(&self) -> ScraperResult {
        let mut all_errors = Vec::new();

        info!("Processing first run with sequential scrapers and delays");
        for order in ScraperOrder::iter() {
            if let Some(scrapers) = self.scrapers.get(&order) {
                let names = scrapers
                    .iter()
                    .map(|scraper| scraper.name())
                    .collect::<Vec<_>>();

                info!(
                    "[📋 {:?}] Processing {} scrapers ({})",
                    order,
                    scrapers.len(),
                    names.join(", ")
                );

                let scraper_futures: Vec<_> = scrapers
                    .iter()
                    .map(|scraper| self.handle_scraper(scraper, order))
                    .collect();

                let results = futures::future::join_all(scraper_futures).await;

                // Collect errors from this bucket
                let errors: Vec<_> = results
                    .into_iter()
                    .enumerate()
                    .filter_map(|(idx, res)| res.err().map(|e| (idx, e)))
                    .collect();

                if !errors.is_empty() {
                    for (idx, error) in &errors {
                        let scraper_name =
                            scrapers.get(*idx).map(|s| s.name()).unwrap_or_else(|| {
                                warn!("Scraper index {} not found", idx);
                                "Unknown scraper"
                            });
                        error!("Scraper {scraper_name} failed:\n\n\tError: {error}\n");
                    }
                    all_errors.extend(errors);
                }
            }
        }

        if all_errors.is_empty() {
            Ok(())
        } else {
            Err(all_errors)
        }
    }

    async fn handle_scraper(
        &self,
        scraper: &Box<dyn Scraper>,
        order: ScraperOrder,
    ) -> Result<(), ScraperError> {
        match scraper.should_scrape().await {
            ShouldScrape::Yes => self.run_scraper(scraper, order).await,
            ShouldScrape::No(reason) => {
                self.log_skip(scraper, reason);
                Ok(())
            }
        }
    }

    async fn run_scraper(
        &self,
        scraper: &Box<dyn Scraper>,
        order: ScraperOrder,
    ) -> Result<(), ScraperError> {
        info!("Running scraper {} of order {:?}", scraper.name(), order);
        scraper.scrape().await
    }

    fn log_skip(&self, scraper: &Box<dyn Scraper>, reason: NoScrapeReason) {
        info!(
            "Not running scraper {} with reason: {}",
            scraper.name(),
            reason.message()
        );
    }
}
