use fpl_common::types::TeamId;

#[derive(Debug, sqlx::FromRow)]
pub struct DiscordUser {
    pub discord_id: i64,
    pub team_id: TeamId,
}

impl DiscordUser {
    pub fn new(discord_id: i64, team_id: TeamId) -> Self {
        Self {
            discord_id,
            team_id,
        }
    }
}
