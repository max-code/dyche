use std::{
    cmp::Ordering,
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use fpl_common::types::GameWeekId;
use fpl_db::queries::game_week::get_current_game_week_id;
use itertools::Itertools;
use serenity::all::{ChannelId, Http};
use sqlx::PgPool;
use tracing::{error, info};

use crate::Error;

#[derive(Debug)]
pub struct LiveOwners {
    pub web_name: String,
    pub code: i32,
    pub total_points: i16,
    pub player_id: i16,
    pub owners: Vec<i64>,
}

pub struct Points {
    player_points: Mutex<HashMap<i16, i16>>,
    pool: Arc<PgPool>,
    http: Arc<Http>,
    notification_channel: ChannelId,
    current_game_week: Mutex<Option<GameWeekId>>,
}

pub struct PointsNotification {
    pub web_name: String,
    pub code: i32,
    pub old_points: i16,
    pub new_points: i16,
    pub owners: Vec<i64>,
}

impl PointsNotification {
    pub fn owners_to_str(&self) -> String {
        self.owners
            .iter()
            .map(|owner| format!("<@{owner}>"))
            .join(" ,")
    }
}

impl Points {
    /*

    Updates logic:

    - If GW has changed, reset player_points and update current_game_week, return and poll again later

    - Can now assume we are in the same GW as before so player_points are relevant
    - - If player_points is empty, populate and do nothing (bot startup or just refreshed after GW change)
    - - Otherwise, query the live_owners view and compare against stored player_points. If anything is different, need notif
    - - - Group all notifs then format and send in send_notifications()

     */
    pub fn new(pool: Arc<PgPool>, http: Arc<Http>, notification_channel: ChannelId) -> Self {
        Self {
            pool,
            http,
            notification_channel,
            current_game_week: Mutex::new(None),
            player_points: Mutex::new(HashMap::new()),
        }
    }

    pub async fn start(self: Arc<Self>) -> Result<(), Error> {
        info!("Starting live points tracking & notifications");
        // start the polling thread here
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(2));

            loop {
                interval.tick().await;
                if let Err(e) = self.poll().await {
                    error!("Error when polling live points notifications: {}", e);
                }
            }
        });
        Ok(())
    }

    pub async fn poll(&self) -> Result<(), Error> {
        info!("Polling live points");

        // Check for GW change. Extra scope so the mutex guard on stored_gw is dropped.
        {
            let current_game_week = get_current_game_week_id(&self.pool).await?;
            let mut stored_gw = self.current_game_week.lock().unwrap();
            if let Some(s_gw) = *stored_gw {
                if s_gw != current_game_week {
                    info!(
                        "Game Week has changed from GW{} to GW{}. Refreshing live owners.",
                        s_gw, current_game_week
                    );

                    *stored_gw = Some(current_game_week);
                    self.player_points.lock().unwrap().clear();
                    return Ok(());
                }
            } else {
                info!("Game Week didn't exist, setting to GW{}", current_game_week);
                *stored_gw = Some(current_game_week);
                return Ok(());
            }
        }

        self.check_live_owners().await?;

        Ok(())
    }

    async fn check_live_owners(&self) -> Result<(), Error> {
        let raw_owners = sqlx::query!(
            r#"    
            SELECT 
                web_name as "web_name!",
                code as "code!",
                total_points as "total_points!",
                player_id as "player_id!",
                owners as "owners!"
            FROM live_owners;
            "#
        )
        .fetch_all(&*self.pool)
        .await?;

        if raw_owners.is_empty() {
            info!("No entries in the live_owners view. Returning.");
            return Ok(());
        }

        // Parse out the owners csv into a Vec<i64>
        let new_live_owners: Vec<LiveOwners> = raw_owners
            .into_iter()
            .map(|row| LiveOwners {
                web_name: row.web_name,
                code: row.code,
                total_points: row.total_points,
                player_id: row.player_id,
                owners: match row.owners {
                    owners_str if !owners_str.is_empty() => owners_str
                        .split(',')
                        .filter_map(|id| id.parse::<i64>().ok())
                        .collect(),
                    _ => Vec::new(),
                },
            })
            .collect();

        let notifications = {
            let mut stored_player_points = self.player_points.lock().unwrap();
            let mut notifications_vec: Vec<PointsNotification> = Vec::new();

            match stored_player_points.len() {
                0 => {
                    // No notifications on first load
                    info!("Got information from live_owners but nothing currently stored. Not sending notifications.\nNew live owners: {:?}", new_live_owners);
                    for owner in new_live_owners {
                        stored_player_points.insert(owner.player_id, owner.total_points);
                    }
                    Vec::new()
                }
                _ => {
                    for info in new_live_owners {
                        let existing_points = stored_player_points
                            .entry(info.player_id)
                            .or_insert(info.total_points);

                        if *existing_points != info.total_points {
                            notifications_vec.push(PointsNotification {
                                web_name: info.web_name,
                                code: info.code,
                                old_points: *existing_points,
                                new_points: info.total_points,
                                owners: info.owners.clone(),
                            });

                            *existing_points = info.total_points;
                        }
                    }
                    notifications_vec
                }
            }
        };

        if !notifications.is_empty() {
            self.send_updates(&notifications).await?;
        }

        Ok(())
    }

    async fn send_updates(&self, notifications: &[PointsNotification]) -> Result<(), Error> {
        info!("Sending {} point update notifications", notifications.len());
        for notification in notifications {
            let red_arrow = "<:arrow_green:1284491445323169835>";
            let green_arrow = "<:arrow_red:1284491446564687902>";
            let question_mark = "❔";

            let emoji = match notification.old_points.cmp(&notification.new_points) {
                Ordering::Less => red_arrow,
                Ordering::Greater => green_arrow,
                _ => question_mark,
            };

            let content = format!(
                "**{}** {} **{}**\n\nOwners: {}",
                notification.old_points,
                emoji,
                notification.new_points,
                notification.owners_to_str()
            );

            let image_path = format!(
                "/Users/maxjordan/code/dyche/fpl_assets/player_images/{}.png",
                notification.code
            );

            let image_attachment = serenity::builder::CreateAttachment::path(image_path).await?;
            let image_filename = image_attachment.filename.clone();

            let embed = serenity::builder::CreateEmbed::new()
                .title(format!("🔔 {} Points Update", notification.web_name))
                .description(content)
                .color((252, 186, 3))
                .thumbnail(format!("attachment://{}", image_filename));

            self.notification_channel
                .send_message(
                    &self.http,
                    serenity::builder::CreateMessage::new()
                        .add_embed(embed)
                        .add_file(image_attachment),
                )
                .await?;
        }
        Ok(())
    }
}
