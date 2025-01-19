use super::FplRequest;
use crate::responses::mini_league::MiniLeagueResponse;
use crate::types::LeagueId;

pub struct MiniLeagueRequest {
    pub league_id: LeagueId,
}

impl MiniLeagueRequest {
    pub fn new(league_id: LeagueId) -> Self {
        Self { league_id }
    }
}

impl FplRequest for MiniLeagueRequest {
    type Response = MiniLeagueResponse;

    fn to_url(&self, base_url: &str) -> String {
        format!(
            "{}/leagues-classic/{}/standings/?page_standings=1",
            base_url, self.league_id
        )
    }
}
