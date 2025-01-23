pub mod requests;
pub mod responses;

use requests::FplRequest;
use reqwest::Client;
use std::time::Duration;

pub const REQ_TIMEOUT_SECONDS: u64 = 30;

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

    pub async fn get<T: FplRequest>(&self, request: T) -> Result<T::Response, reqwest::Error> {
        let url = request.to_url(&self.base_url);
        println!("Making request with URL: {}", url);
        self.client
            .get(&url)
            .send()
            .await?
            .json::<T::Response>()
            .await
    }
}

#[cfg(test)]
mod tests {

    use fpl_common::types::{GameWeekId, LeagueId, PlayerId, TeamId};
    use requests::{
        FixtureRequest, GameStateRequest, GameWeekPlayersStatsRequest, MiniLeagueRequest,
        PlayerRequest, TransfersRequest,
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
        let game_week = GameWeekId::new(22);
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
    async fn test_game_week_players_stats_request() {
        // Arrange
        let client = FplClient::new();
        let gw = GameWeekId::new(20);
        assert!(gw.is_ok(), "GameWeek 20 should be valid.");

        // Act
        let request = GameWeekPlayersStatsRequest::new(gw.unwrap());
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
        println!("Response: {:#?}", response);
    }
}
