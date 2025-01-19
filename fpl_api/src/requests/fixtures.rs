use super::FplRequest;
use crate::responses::fixtures::FixturesResponse;

pub struct FixtureRequest {}

impl Default for FixtureRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl FixtureRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl FplRequest for FixtureRequest {
    type Response = FixturesResponse;

    fn to_url(&self, base_url: &str) -> String {
        format!("{}/fixtures/", base_url)
    }
}
