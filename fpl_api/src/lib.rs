pub mod requests;
pub mod responses;

use requests::FplRequest;
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;
use thiserror::Error;
use tracing::debug;

pub const REQ_TIMEOUT_SECONDS: u64 = 30;

#[derive(Error, Debug)]
pub enum FplClientError {
    #[error("HTTP {status} from {url}: {message}")]
    RequestError {
        status: reqwest::StatusCode,
        url: String,
        message: String,
    },
    #[error("JSON parsing error (status: {1}): {0}")]
    JsonError(serde_json::Error, reqwest::StatusCode),
    #[error("Response body missing extra detail that should have been added in process_response.")]
    MissingExtraDetailError,
}

impl From<(serde_json::Error, reqwest::StatusCode)> for FplClientError {
    fn from((error, status): (serde_json::Error, reqwest::StatusCode)) -> Self {
        Self::JsonError(error, status)
    }
}

impl From<reqwest::Error> for FplClientError {
    fn from(e: reqwest::Error) -> Self {
        Self::RequestError {
            status: e.status().unwrap_or_default(),
            url: e.url().map_or("unknown".to_string(), |u| u.to_string()),
            message: e.to_string(),
        }
    }
}

#[derive(Clone)]
pub struct FplClient {
    client: Client,
    base_url: String,
}

impl Default for FplClient {
    fn default() -> Self {
        Self::new()
    }
}

impl FplClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(REQ_TIMEOUT_SECONDS))
            .build()
            .expect("Failed to build FplClient. Reqwest client cant build");

        Self {
            client,
            base_url: "https://fantasy.premierleague.com/api".to_string(),
        }
    }

    pub async fn get<T: FplRequest + std::fmt::Debug>(
        &self,
        request: T,
    ) -> Result<T::Response, FplClientError> {
        let url = request.to_url(&self.base_url);
        debug!("Making {:?} with URL {}", request, url);
        let response = self.client.get(&url).send().await?;
        let status = response.status();
        let body = response.text().await?;
        let value: Value = serde_json::from_str(&body).map_err(|e| (e, status))?;
        request
            .process_response(value)
            .map_err(|e| (e, status).into())
    }
}

#[cfg(test)]
mod tests {

    use fpl_common::types::{GameWeekId, LeagueId, PlayerId, TeamId};
    use requests::{
        FixtureRequest, GameStateRequest, GameWeekPlayersRequest, MiniLeagueRequest, PlayerRequest,
        TransfersRequest,
    };

    use super::*;
    use crate::requests::TeamGameWeekRequest;
    use crate::requests::TeamRequest;

    #[tokio::test]
    async fn test_team_request() {
        let client = FplClient::new();
        let request = TeamRequest::new(TeamId::new(1871038));

        let response = client.get(request).await.unwrap();
        println!("Response: {:#?}", response);
    }

    #[tokio::test]
    async fn test_team_game_week_request() {
        // Arrange
        let client = FplClient::new();
        let game_week = GameWeekId::new(23);
        assert!(game_week.is_ok(), "GameWeek 22 should be valid");

        // Act
        let request = TeamGameWeekRequest::new(TeamId::new(1871038), game_week.unwrap());
        let response = client.get(request).await.unwrap();

        // Assert
        println!("Response: {:#?}", response);
    }

    #[tokio::test]
    async fn test_mini_league_request() {
        // Arrange
        let client = FplClient::new();

        // Act
        let request = MiniLeagueRequest::new(LeagueId::new(550971));
        let response = client.get(request).await.unwrap();

        // Assert
        println!("Response: {:#?}", response);
        // Add any assertions about the response here
    }

    #[tokio::test]
    async fn test_player_request() {
        // Arrange
        let client = FplClient::new();

        // Act
        let request = PlayerRequest::new(PlayerId::new(180));
        let response = client.get(request).await.unwrap();

        // Assert
        println!("Response: {:#?}", response);
        // Add any assertions about the response here
    }

    #[tokio::test]
    async fn test_fixture_request() {
        // Arrange
        let client = FplClient::new();

        // Act
        let request = FixtureRequest::new();
        let response = client.get(request).await.unwrap();

        // Assert
        println!("Response: {:#?}", response);
        // Add any assertions about the response here
    }

    #[tokio::test]
    async fn test_game_week_players_request() {
        // Arrange
        let client = FplClient::new();
        let gw = GameWeekId::new(25);
        assert!(gw.is_ok(), "GameWeek 20 should be valid.");

        // Act
        let request = GameWeekPlayersRequest::new(gw.unwrap());
        let response = client.get(request).await.unwrap();

        // Assert
        println!("Response: {:#?}", response);
    }

    #[tokio::test]
    async fn test_transfers_request() {
        // Arrange
        let client = FplClient::new();

        // Act
        let request = TransfersRequest::new(TeamId::new(1871038));
        let response = client.get(request).await.unwrap();

        // Assert
        println!("Response: {:#?}", response);
    }

    #[tokio::test]
    async fn test_game_state_request() {
        // Arrange
        let client = FplClient::new();

        // Act
        let request = GameStateRequest::new();
        let response = client.get(request).await.unwrap();

        // Assert
        // println!("Response: {:#?}", response);
    }
}
