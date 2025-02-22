use super::{FplRequest, FplResponseType};
use crate::responses::mini_league::{MiniLeagueResponse, MiniLeagueResponseWrapper};
use fpl_common::types::LeagueId;

#[derive(Debug)]
pub struct MiniLeagueRequest {
    pub league_id: LeagueId,
    pub page: u8,
}

impl MiniLeagueRequest {
    pub fn new(league_id: LeagueId, page: u8) -> Self {
        Self { league_id, page }
    }
}

impl FplRequest for MiniLeagueRequest {
    type Response = MiniLeagueResponse;

    fn to_url(&self, base_url: &str) -> String {
        format!(
            "{}/leagues-classic/{}/standings/?page_standings={}",
            base_url, self.league_id, self.page
        )
    }

    fn process_response(
        &self,
        response: FplResponseType,
    ) -> Result<Self::Response, Box<dyn std::error::Error>> {
        match response {
            FplResponseType::Json(value) => {
                let wrapper: MiniLeagueResponseWrapper = serde_json::from_value(value)?;
                match wrapper {
                    MiniLeagueResponseWrapper::Success(response) => Ok(response),
                    MiniLeagueResponseWrapper::PlainText(message) => Err(message.into()),
                }
            }
            FplResponseType::Binary(_) => Err("Expected JSON response, got binary".into()),
        }
    }
}
