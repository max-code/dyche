use std::time::{Duration, SystemTime};

use crate::error::ScraperError;
use crate::scraper::{Scraper, ScraperOrder, ShouldScrape};
use crate::{with_retry, NoScrapeReason, DEFAULT_MAX_RETRIES};
use async_trait::async_trait;
use fpl_api::responses::mini_league::{MiniLeagueResponse, Standing};
use fpl_common::types::LeagueId;
use fpl_db::models::{MiniLeague, MiniLeagueStanding};
use fpl_db::queries::mini_league::{
    get_all_mini_league_ids, upsert_mini_league_standings, upsert_mini_leagues,
};
use futures::StreamExt;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use fpl_api::requests::MiniLeagueRequest;
use fpl_api::FplClient;

pub struct MiniLeaguesScraper {
    pool: Arc<PgPool>,
    client: Arc<FplClient>,
    min_scrape_interval: Duration,
    last_scrape: RwLock<Option<SystemTime>>,
}

impl MiniLeaguesScraper {
    pub fn new(pool: Arc<PgPool>, client: Arc<FplClient>, min_scrape_interval: Duration) -> Self {
        info!("Creating MiniLeaguesScraper");
        Self {
            pool,
            client,
            min_scrape_interval,
            last_scrape: RwLock::new(None),
        }
    }

    async fn handle_mini_league(
        client: Arc<FplClient>,
        league_id: LeagueId,
    ) -> Result<(MiniLeagueResponse, Vec<Standing>), ScraperError> {
        let mut mini_league_standings: Vec<Standing> = Vec::new();
        let mut page = 1;
        let mut current_page = with_retry(
            || {
                let client_clone = client.clone();
                let league_id_clone = league_id;
                async move {
                    client_clone
                        .get(MiniLeagueRequest::new(league_id_clone, page))
                        .await
                }
            },
            DEFAULT_MAX_RETRIES,
        )
        .await?;

        mini_league_standings.extend(current_page.standings.results.clone());
        while current_page.standings.has_next {
            page += 1;
            current_page = with_retry(
                || {
                    let client_clone = client.clone();
                    let league_id_clone = league_id;
                    let page_clone = page;
                    async move {
                        client_clone
                            .get(MiniLeagueRequest::new(league_id_clone, page_clone))
                            .await
                    }
                },
                DEFAULT_MAX_RETRIES,
            )
            .await?;

            mini_league_standings.extend(current_page.standings.results.clone());
        }

        Ok((current_page, mini_league_standings))
    }
}

#[async_trait]
impl Scraper for MiniLeaguesScraper {
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
        let all_league_ids = get_all_mini_league_ids(&self.pool).await?;
        let chunk_size = 100;

        for chunk in all_league_ids.chunks(chunk_size) {
            let chunk = chunk.to_vec();

            let mut stream = futures::stream::iter(chunk.into_iter().map(|league_id| {
                MiniLeaguesScraper::handle_mini_league(self.client.clone(), league_id)
            }))
            .buffer_unordered(5);

            let mut leagues_info: Vec<MiniLeague> = Vec::with_capacity(chunk_size);
            let mut leagues_standing_info: Vec<MiniLeagueStanding> = Vec::new();

            while let Some(result) = stream.next().await {
                let response = match result {
                    Ok(response) => response,
                    Err(e) => {
                        warn!("{}", e);
                        continue;
                    }
                };

                let (league, standings) = response;

                leagues_info.push((&league).into());
                leagues_standing_info.extend(
                    standings
                        .into_iter()
                        .map(|standing| (&league.league.id, &standing).into())
                        .collect::<Vec<MiniLeagueStanding>>(),
                )
            }

            upsert_mini_leagues(&self.pool, &leagues_info).await?;
            upsert_mini_league_standings(&self.pool, &leagues_standing_info).await?;
        }

        *self.last_scrape.write().await = Some(SystemTime::now());
        Ok(())
    }

    fn position(&self) -> ScraperOrder {
        ScraperOrder::Third
    }
}
