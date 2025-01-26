use fpl_api::requests::TransfersRequest;
use fpl_api::FplClient;
use fpl_common::types::TeamId;
use fpl_db::models::Transfer;
use fpl_db::queries::team::get_all_team_ids;
use fpl_db::queries::transfers::upsert_transfers;
use sqlx::PgPool;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let total_start = Instant::now();
    let database_url = dotenv::var("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url).await?;
    let client = FplClient::new();

    let ids = get_all_team_ids(&pool).await?;

    let team_game_week_chunks: Vec<_> = ids.chunks(100).map(|c| c.to_vec()).collect();

    for chunk in team_game_week_chunks {
        let futures: Vec<_> = chunk
            .into_iter()
            .map(|team_id| process_transfers(pool.clone(), client.clone(), team_id))
            .collect();
        let results = futures::future::join_all(futures).await;
        for result in results {
            result?;
        }
    }

    println!("Total execution time: {:?}", total_start.elapsed());
    Ok(())
}

async fn process_transfers(
    pool: PgPool,
    client: FplClient,
    team_id: TeamId,
) -> Result<(), Box<dyn std::error::Error>> {
    let transfers_response = client.get(TransfersRequest::new(team_id)).await?;
    let transfers: Vec<Transfer> = transfers_response.iter().map(|t| t.into()).collect();
    upsert_transfers(&pool, &transfers).await?;

    Ok(())
}
