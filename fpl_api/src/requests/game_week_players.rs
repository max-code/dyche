use super::{FplRequest, FplResponseType};
use crate::responses::game_week_players::{
    GameWeekPlayersStatsResponse, GameWeekPlayersStatsResponseWrapper,
};
use fpl_common::types::GameWeekId;
use serde::de::Error;

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
        response: FplResponseType,
    ) -> Result<Self::Response, Box<dyn std::error::Error>> {
        match response {
            FplResponseType::Json(value) => {
                let wrapper: GameWeekPlayersStatsResponseWrapper = serde_json::from_value(value)?;

                match wrapper {
                    GameWeekPlayersStatsResponseWrapper::Success(mut response) => {
                        response.game_week = Some(self.game_week);
                        Ok(response)
                    }
                    GameWeekPlayersStatsResponseWrapper::PlainText(message) => {
                        Err(Box::new(serde_json::Error::custom(message)))
                    }
                }
            }
            FplResponseType::Binary(_) => Err("Expected JSON response, got binary".into()),
        }
    }
}
