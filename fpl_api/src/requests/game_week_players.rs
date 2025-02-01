use super::FplRequest;
use crate::responses::game_week_players::{
    GameWeekPlayersStatsResponse, GameWeekPlayersStatsResponseWrapper,
};
use fpl_common::types::GameWeekId;

#[derive(Debug)]
pub struct GameWeekPlayersRequest {
    pub game_week: GameWeekId,
}

impl GameWeekPlayersRequest {
    pub fn new(game_week: GameWeekId) -> Self {
        Self { game_week }
    }
}

impl FplRequest for GameWeekPlayersRequest {
    type Response = GameWeekPlayersStatsResponse;

    fn to_url(&self, base_url: &str) -> String {
        format!("{}/event/{}/live/", base_url, self.game_week)
    }

    fn process_response(
        &self,
        response: serde_json::Value,
    ) -> Result<Self::Response, serde_json::Error> {
        let wrapper: GameWeekPlayersStatsResponseWrapper = serde_json::from_value(response)?;

        match wrapper {
            GameWeekPlayersStatsResponseWrapper::Success(mut response) => {
                response.game_week = Some(self.game_week);
                Ok(response)
            }
            GameWeekPlayersStatsResponseWrapper::PlainText(message) => {
                Err(serde::de::Error::custom(message))
            }
        }
    }
}
