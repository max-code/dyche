use std::{collections::HashMap, time::Duration};

use crate::error::ScraperError;
use async_trait::async_trait;
use strum::{EnumIter, IntoEnumIterator};
use tokio::time;
use tracing::{debug, error, info, instrument, trace};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, PartialOrd, Ord, EnumIter)]
pub enum ScraperOrder {
    First,
    Second,
    Third,
}

#[derive(Debug)]
pub enum ShouldScrape {
    Yes,
    No(NoScrapeReason),
}

#[derive(Debug)]
enum NoScrapeReason {
    TimeIntervalNotLapsed(u16),
}

impl NoScrapeReason {
    fn message(&self) -> String {
        match self {
            Self::TimeIntervalNotLapsed(mins) => format!(
                "Minimum time interval ({} minutes) between scrapes not exceeded.",
                mins
            ),
        }
    }
}

#[async_trait]
pub trait Scraper: Send + Sync {
    fn name(&self) -> &'static str;
    fn should_scrape(&self) -> ShouldScrape;
    async fn scrape(&self) -> Result<(), ScraperError>;
    fn position(&self) -> ScraperOrder;
}

pub struct ScraperManager {
    scrapers: HashMap<ScraperOrder, Vec<Box<dyn Scraper>>>,
}

impl ScraperManager {
    #[instrument]
    pub fn new() -> Self {
        info!("Initializing ScraperManager");
        Self {
            scrapers: HashMap::default(),
        }
    }

    #[instrument(skip(self, scraper))]
    pub fn register_scraper<S>(&mut self, order: ScraperOrder, scraper: S)
    where
        S: Scraper + 'static,
    {
        info!(
            "Registering scraper {} with order {:?}",
            scraper.name(),
            order
        );
        self.scrapers
            .entry(order)
            .or_insert_with(Vec::new)
            .push(Box::new(scraper));
        debug!("Current scraper count: {}", self.scrapers.len());
    }

    #[instrument(skip(self))]
    pub async fn run(&self) {
        info!("Starting scraper manager");
        let mut interval = time::interval(Duration::from_secs(10));
        loop {
            interval.tick().await;
            trace!("Tick");
            self.process_all_orders().await;
        }
    }

    async fn process_all_orders(&self) {
        for order in ScraperOrder::iter() {
            debug!("Processing scrapers for order {:?}", order);
            if let Some(scrapers) = self.scrapers.get(&order) {
                self.process_scrapers(scrapers, order).await;
            }
        }
    }

    async fn process_scrapers(&self, scrapers: &[Box<dyn Scraper>], order: ScraperOrder) {
        for (idx, scraper) in scrapers.iter().enumerate() {
            debug!("Checking scraper {} of order {:?}", idx, order);
            self.handle_scraper(scraper, idx, order).await;
        }
    }

    async fn handle_scraper(&self, scraper: &Box<dyn Scraper>, idx: usize, order: ScraperOrder) {
        match scraper.should_scrape() {
            ShouldScrape::Yes => self.run_scraper(scraper, idx, order).await,
            ShouldScrape::No(reason) => self.log_skip(scraper, idx, reason),
        }
    }

    async fn run_scraper(&self, scraper: &Box<dyn Scraper>, idx: usize, order: ScraperOrder) {
        info!(
            "Running scraper {} (idx {}) of order {:?}",
            scraper.name(),
            idx,
            order
        );
        match scraper.scrape().await {
            Ok(_) => info!(
                "Scraper {} (idx {}) completed successfully",
                scraper.name(),
                idx
            ),
            Err(e) => error!("Scraper {} (idx {}) failed: {}", scraper.name(), idx, e),
        }
    }

    fn log_skip(&self, scraper: &Box<dyn Scraper>, idx: usize, reason: NoScrapeReason) {
        debug!(
            "Not running scraper {} (idx {}) with reason: {}",
            scraper.name(),
            idx,
            reason.message()
        );
    }
}
