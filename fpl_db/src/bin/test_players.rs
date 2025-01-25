use fpl_api::requests::GameStateRequest;
use fpl_api::FplClient;
use fpl_db::models::Player;
use fpl_db::queries::player::upsert_players;
use sqlx::PgPool;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let total_start = Instant::now();

    dotenv::from_filename(".env").ok();
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    let connect_start = Instant::now();
    let pool = PgPool::connect(&database_url).await?;
    println!("DB connection took: {:?}", connect_start.elapsed());

    let client = FplClient::new();
    let request = GameStateRequest::new();

    let api_start = Instant::now();
    let game_state = client.get(request).await.unwrap();
    println!("API request took: {:?}", api_start.elapsed());

    let players = game_state.elements;
    println!("Got {} players", players.len());

    let conversion_start = Instant::now();
    let players_rows: Vec<Player> = players.into_iter().map(|f| f.try_into().unwrap()).collect();
    println!("Conversion took: {:?}", conversion_start.elapsed());

    let upsert_start = Instant::now();
    upsert_players(&pool, &players_rows).await?;
    println!("Upsert took: {:?}", upsert_start.elapsed());

    println!("Total execution time: {:?}", total_start.elapsed());
    Ok(())
}
