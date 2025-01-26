use super::FplRequest;
use crate::responses::team_game_week::TeamGameWeekResponse;
use fpl_common::types::{GameWeekId, TeamId};

pub struct TeamGameWeekRequest {
    pub team_id: TeamId,
    pub game_week: GameWeekId,
}

impl TeamGameWeekRequest {
    pub fn new(team_id: TeamId, game_week: GameWeekId) -> Self {
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

    fn process_response(
        &self,
        response: serde_json::Value,
    ) -> Result<Self::Response, serde_json::Error> {
        let mut resp: TeamGameWeekResponse = serde_json::from_value(response)?;
        resp.team_id = Some(self.team_id);
        resp.game_week_id = Some(self.game_week);
        Ok(resp)
    }
}
