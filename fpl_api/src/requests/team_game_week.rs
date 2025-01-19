use super::FplRequest;
use crate::responses::team_game_week::TeamGameWeekResponse;
use crate::types::{GameWeek, TeamId};

pub struct TeamGameWeekRequest {
    pub team_id: TeamId,
    pub game_week: GameWeek,
}

impl TeamGameWeekRequest {
    pub fn new(team_id: TeamId, game_week: GameWeek) -> Self {
        Self { team_id, game_week }
    }
}

impl FplRequest for TeamGameWeekRequest {
    type Response = TeamGameWeekResponse;

    fn to_url(&self, base_url: &str) -> String {
        format!(
            "{}/entry/{}/event/{}/picks/",
            base_url, self.team_id, self.game_week
        )
    }
}
