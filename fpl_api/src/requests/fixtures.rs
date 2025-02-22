use super::{FplRequest, FplResponseType};
use crate::responses::fixtures::FixturesResponse;

#[derive(Debug)]
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

    fn process_response(
        &self,
        response: FplResponseType,
    ) -> Result<Self::Response, Box<dyn std::error::Error>> {
        match response {
            FplResponseType::Json(value) => Ok(serde_json::from_value(value)?),
            FplResponseType::Binary(_) => Err("Expected JSON response, got binary".into()),
        }
    }
}
