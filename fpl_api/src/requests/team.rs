use super::FplRequest;
use crate::responses::team::TeamResponse;
use fpl_common::types::TeamId;

#[derive(Debug)]
pub struct TeamRequest {
    pub team_id: TeamId,
}

impl TeamRequest {
    pub fn new(team_id: TeamId) -> Self {
        Self { team_id }
    }
}

impl FplRequest for TeamRequest {
    type Response = TeamResponse;

    fn to_url(&self, base_url: &str) -> String {
        format!("{}/entry/{}/", base_url, self.team_id)
    }
}
