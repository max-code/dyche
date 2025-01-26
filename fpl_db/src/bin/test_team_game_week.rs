use fpl_api::requests::{GameStateRequest, PlayerRequest};
use fpl_api::FplClient;
use fpl_common::types::PlayerId;
use fpl_db::models::{Player, PlayerFixtureDb, PlayerHistoryDb, PlayerHistoryPastDb};
use fpl_db::queries::player::{
    upsert_player_fixtures, upsert_player_histories, upsert_player_history_past, upsert_players,
};
use sqlx::PgPool;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let total_start = Instant::now();
    let database_url = dotenv::var("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url).await?;
    let client = FplClient::new();

    // Get game state and insert players
    let game_state = client.get(GameStateRequest::new()).await?;
    let players: Vec<Player> = game_state
        .elements
        .iter()
        .map(TryInto::try_into)
        .collect::<Result<_, _>>()?;
    upsert_players(&pool, &players).await?;

    // Process players concurrently in chunks
    let player_chunks: Vec<_> = players
        .iter()
        .map(|p| p.id)
        .collect::<Vec<_>>()
        .chunks(10)
        .map(|c| c.to_vec())
        .collect();

    for chunk in player_chunks {
        let futures: Vec<_> = chunk
            .into_iter()
            .map(|player_id| process_player(pool.clone(), client.clone(), player_id))
            .collect();
        futures::future::join_all(futures).await;
    }

    println!("Total execution time: {:?}", total_start.elapsed());
    Ok(())
}

async fn process_player(
    pool: PgPool,
    client: FplClient,
    player_id: PlayerId,
) -> Result<(), Box<dyn std::error::Error>> {
    let player = client.get(PlayerRequest::new(player_id)).await?;

    // Process fixtures
    let fixtures: Vec<PlayerFixtureDb> = player
        .fixtures
        .iter()
        .map(|f| (player_id, f).try_into())
        .collect::<Result<_, _>>()?;
    upsert_player_fixtures(&pool, &fixtures).await?;

    // Process history
    let history: Vec<PlayerHistoryDb> = player
        .history
        .iter()
        .map(TryInto::try_into)
        .collect::<Result<_, _>>()?;
    upsert_player_histories(&pool, &history).await?;

    // Process history past
    let history_past: Vec<PlayerHistoryPastDb> = player
        .history_past
        .iter()
        .map(|f| (player_id, f).try_into())
        .collect::<Result<_, _>>()?;
    upsert_player_history_past(&pool, &history_past).await?;

    Ok(())
}
