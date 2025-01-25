use super::FplRequest;
use crate::responses::game_week_players_stats::GameWeekPlayersStatsResponse;
use fpl_common::types::GameWeekId;

pub struct GameWeekPlayersStatsRequest {
    pub game_week: GameWeekId,
}

impl GameWeekPlayersStatsRequest {
    pub fn new(game_week: GameWeekId) -> Self {
        Self { game_week }
    }
}

impl FplRequest for GameWeekPlayersStatsRequest {
    type Response = GameWeekPlayersStatsResponse;

    fn to_url(&self, base_url: &str) -> String {
        format!("{}/event/{}/live/", base_url, self.game_week)
    }

    fn process_response(
        &self,
        response: serde_json::Value,
    ) -> Result<Self::Response, serde_json::Error> {
        let mut resp: GameWeekPlayersStatsResponse = serde_json::from_value(response)?;
        resp.game_week = Some(self.game_week);
        Ok(resp)
    }
}
