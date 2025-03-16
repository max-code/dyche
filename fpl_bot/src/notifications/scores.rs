use std::{
    collections::{HashMap, HashSet},
    sync::{atomic::AtomicBool, Arc, Mutex},
    time::Duration,
};

use serenity::all::{ChannelId, Http};
use sqlx::PgPool;
use tracing::{debug, error, info};

use crate::Error;

#[derive(Debug)]
pub struct LiveFixtures {
    pub id: i16,
    pub home_team_score: i16,
    pub away_team_score: i16,
    pub home_team_name: String,
    pub away_team_name: String,
}

pub struct ScoreNotifications {
    // FixtureId: Score(home_team_score, away_team_score)
    scores: Mutex<HashMap<i16, (i16, i16)>>,
    pool: Arc<PgPool>,
    http: Arc<Http>,
    notification_channel: ChannelId,
    first_run: AtomicBool,
}

pub struct ScoreNotification {
    pub home_team: String,
    pub home_team_score: i16,
    pub home_team_score_changed: bool,
    pub away_team: String,
    pub away_team_score: i16,
    pub away_team_score_changed: bool,
    pub new_fixture: bool,
}

impl ScoreNotifications {
    /*

    Updates logic:

    - Get live fixtures and remove any keys for which fixture started = true and finished = false
    - Remove any keys from the map where the fixture_id isnt in this query
    - For anything in this query
    - - If its a new key, send a fixture started notif
    - - If its not a new key, and the score is the same, do nothing
    - - If its not a new key, and the score ISNT the same, send a score update notif underlining the side that changed

     */
    pub fn new(pool: Arc<PgPool>, http: Arc<Http>, notification_channel: ChannelId) -> Self {
        Self {
            pool,
            http,
            notification_channel,
            scores: Mutex::new(HashMap::new()),
            first_run: AtomicBool::new(true),
        }
    }

    pub async fn start(self: Arc<Self>) -> Result<(), Error> {
        info!("Starting live scores tracking & notifications");
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(10));

            loop {
                interval.tick().await;
                if let Err(e) = self.poll().await {
                    error!("Error when polling live scores notifications: {}", e);
                }
            }
        });
        Ok(())
    }

    pub async fn poll(&self) -> Result<(), Error> {
        info!(
            "Polling live scores. ChannelID=[{}], Tracked Fixture Count = [{}]",
            self.notification_channel,
            self.scores.lock().unwrap().len()
        );

        let live_fixtures = sqlx::query_as!(
            LiveFixtures,
            r#"    
            SELECT 
                f.id as "id!",
                f.home_team_score as "home_team_score!",
                f.away_team_score as "away_team_score!",
                home_club.name AS "home_team_name!",
                away_club.name AS "away_team_name!"
            FROM 
                fixtures f
            JOIN 
                clubs home_club ON f.home_team_id = home_club.id
            JOIN 
                clubs away_club ON f.away_team_id = away_club.id
            WHERE f.started  = true and f.finished  = false;
            "#
        )
        .fetch_all(&*self.pool)
        .await?;

        if live_fixtures.is_empty() {
            debug!("No live fixtures, returning");
            return Ok(());
        }

        let is_first_run = self
            .first_run
            .swap(false, std::sync::atomic::Ordering::SeqCst);

        let notifications = {
            let mut notifications_vec: Vec<ScoreNotification> = Vec::new();
            let mut stored_scores = self.scores.lock().unwrap();
            let mut live_fixture_ids = HashSet::new();

            for fixture in live_fixtures {
                live_fixture_ids.insert(fixture.id);
                match stored_scores.get(&fixture.id) {
                    None => {
                        if !is_first_run {
                            notifications_vec.push(ScoreNotification {
                                home_team: fixture.home_team_name,
                                away_team: fixture.away_team_name,
                                home_team_score: fixture.home_team_score,
                                home_team_score_changed: false,
                                away_team_score: fixture.away_team_score,
                                away_team_score_changed: false,
                                new_fixture: true,
                            });
                        }
                    }
                    Some((stored_home_team_score, stored_away_team_score)) => {
                        if stored_home_team_score != &fixture.home_team_score
                            || stored_away_team_score != &fixture.away_team_score
                        {
                            let home_score_changed =
                                stored_home_team_score != &fixture.home_team_score;
                            let away_score_changed =
                                stored_away_team_score != &fixture.away_team_score;

                            notifications_vec.push(ScoreNotification {
                                home_team: fixture.home_team_name,
                                away_team: fixture.away_team_name,
                                home_team_score: fixture.home_team_score,
                                home_team_score_changed: home_score_changed,
                                away_team_score: fixture.away_team_score,
                                away_team_score_changed: away_score_changed,
                                new_fixture: false,
                            });
                        }
                    }
                };
                stored_scores.insert(
                    fixture.id,
                    (fixture.home_team_score, fixture.away_team_score),
                );
            }

            stored_scores.retain(|k, _v| live_fixture_ids.contains(k));
            notifications_vec
        };

        if !notifications.is_empty() {
            self.send_updates(&notifications).await?;
        }

        Ok(())
    }

    async fn send_updates(&self, notifications: &[ScoreNotification]) -> Result<(), Error> {
        info!("Sending {} score update notifications", notifications.len());
        for notification in notifications {
            let home_score_bold = if notification.home_team_score_changed {
                "**"
            } else {
                ""
            };
            let away_score_bold = if notification.away_team_score_changed {
                "**"
            } else {
                ""
            };

            let content = match notification.new_fixture {
                false => {
                    format!(
                        "{}{} {}{} - {}{} {}{}",
                        home_score_bold,
                        notification.home_team,
                        notification.home_team_score,
                        home_score_bold,
                        away_score_bold,
                        notification.away_team_score,
                        notification.away_team,
                        away_score_bold
                    )
                }
                true => {
                    format!(
                        "**{}** vs **{}** has started.",
                        notification.home_team, notification.away_team,
                    )
                }
            };

            let title = match notification.new_fixture {
                false => "🔔 Fixture Score Update",
                true => "🔔 Fixture Kicked Off",
            };

            let embed = serenity::builder::CreateEmbed::new()
                .title(title)
                .description(content)
                .color((55, 200, 219));

            self.notification_channel
                .send_message(
                    &self.http,
                    serenity::builder::CreateMessage::new().add_embed(embed),
                )
                .await?;
        }
        Ok(())
    }
}
