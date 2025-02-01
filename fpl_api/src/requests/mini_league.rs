use super::FplRequest;
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
        response: serde_json::Value,
    ) -> Result<Self::Response, serde_json::Error> {
        let wrapper: MiniLeagueResponseWrapper = serde_json::from_value(response)?;

        match wrapper {
            MiniLeagueResponseWrapper::Success(response) => Ok(response),
            MiniLeagueResponseWrapper::PlainText(message) => Err(serde::de::Error::custom(message)),
        }
    }
}
