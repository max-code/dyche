use std::time::{Duration, SystemTime};

use crate::error::ScraperError;
use crate::scraper::{Scraper, ScraperOrder, ShouldScrape};
use crate::NoScrapeReason;
use async_trait::async_trait;
use fpl_api::responses::team_game_week::TeamGameWeekResponse;
use fpl_db::queries::game_week::get_current_game_week;
use fpl_db::queries::team::get_all_team_ids;
use fpl_db::queries::team_game_week::{
    upsert_team_game_week_automatic_subs, upsert_team_game_week_picks, upsert_team_game_weeks,
};
use futures::StreamExt;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use fpl_api::requests::TeamGameWeekRequest;
use fpl_api::{FplClient, FplClientError};
use fpl_common::types::{GameWeekId, TeamId};

pub struct TeamGameWeekScraper {
    pool: Arc<PgPool>,
    client: Arc<FplClient>,
    min_scrape_interval: Duration,
    last_scrape: RwLock<Option<SystemTime>>,
}

impl TeamGameWeekScraper {
    pub fn new(pool: Arc<PgPool>, client: Arc<FplClient>, min_scrape_interval: Duration) -> Self {
        info!("Creating TeamGameWeekScraper");
        Self {
            pool,
            client,
            min_scrape_interval,
            last_scrape: RwLock::new(None),
        }
    }

    async fn process_team_game_week(
        client: Arc<FplClient>,
        team_id: TeamId,
        game_week_id: GameWeekId,
    ) -> Result<TeamGameWeekResponse, ScraperError> {
        let team_game_week_response = client
            .get(TeamGameWeekRequest::new(team_id, game_week_id))
            .await?;
        Ok(team_game_week_response)
    }
}

#[async_trait]
impl Scraper for TeamGameWeekScraper {
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
        "TeamGameWeekScraper"
    }

    async fn scrape(&self) -> Result<(), ScraperError> {
        let current_game_week = get_current_game_week(&self.pool).await?;
        let team_ids = get_all_team_ids(&self.pool).await?;
        let chunk_size = 100;

        for game_week_id in GameWeekId::weeks_range_iter(1, i16::from(current_game_week.id)) {
            for chunk in team_ids.chunks(chunk_size) {
                let chunk = chunk.to_vec();

                let mut stream = futures::stream::iter(chunk.into_iter().map(|team_id| {
                    TeamGameWeekScraper::process_team_game_week(
                        self.client.clone(),
                        team_id,
                        game_week_id,
                    )
                }))
                .buffer_unordered(20);

                let mut game_week_picks = Vec::with_capacity(chunk_size * 15);
                let mut game_week_automatic_subs = Vec::with_capacity(chunk_size * 4);
                let mut team_game_weeks = Vec::with_capacity(chunk_size);

                while let Some(result) = stream.next().await {
                    let response = match result {
                        Ok(response) => response,
                        Err(e) => {
                            warn!("{}", e);
                            continue;
                        }
                    };
                    let team_id = response.team_id.ok_or(ScraperError::FplApiError(
                        FplClientError::MissingExtraDetailError,
                    ))?;
                    let game_week_id = response.game_week_id.ok_or(ScraperError::FplApiError(
                        FplClientError::MissingExtraDetailError,
                    ))?;

                    game_week_picks.extend(
                        response
                            .picks
                            .iter()
                            .map(|pick| (team_id, game_week_id, pick).into()),
                    );

                    game_week_automatic_subs
                        .extend(response.automatic_subs.iter().map(|sub| sub.into()));

                    team_game_weeks.push((team_id, game_week_id, &response).into());
                }
                println!()
                upsert_team_game_weeks(&self.pool, &team_game_weeks).await?;
                upsert_team_game_week_picks(&self.pool, &game_week_picks).await?;
                upsert_team_game_week_automatic_subs(&self.pool, &game_week_automatic_subs).await?;

                info!(
                    "[{}] Processed {} teams for week {}",
                    self.name(),
                    team_game_weeks.len(),
                    game_week_id
                );
            }
        }
        *self.last_scrape.write().await = Some(SystemTime::now());
        Ok(())
    }

    fn position(&self) -> ScraperOrder {
        ScraperOrder::Third
    }
}
