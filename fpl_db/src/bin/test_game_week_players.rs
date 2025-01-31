use fpl_api::requests::GameWeekPlayersRequest;
use fpl_api::FplClient;
use fpl_common::types::GameWeekId;
use fpl_db::models::GameWeekPlayerDb;
use fpl_db::queries::game_week_player::upsert_game_week_players;
use sqlx::PgPool;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let total_start = Instant::now();

    dotenv::from_filename("../.env").ok();
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    let connect_start = Instant::now();
    let pool = PgPool::connect(&database_url).await?;
    println!("DB connection took: {:?}", connect_start.elapsed());

    let client = FplClient::new();

    for (idx, game_week) in GameWeekId::all_weeks_iter().enumerate() {
        let request = GameWeekPlayersRequest::new(game_week);

        let api_start = Instant::now();
        let game_week_players = client.get(request).await.unwrap();
        println!("API {idx} request took: {:?}", api_start.elapsed());

        let conversion_start = Instant::now();
        let game_week_players: Vec<GameWeekPlayerDb> = game_week_players
            .elements
            .into_iter()
            .map(|game_week_player| (game_week, game_week_player).into())
            .collect();
        println!("Conversion {idx} took: {:?}", conversion_start.elapsed());

        let upsert_start = Instant::now();
        upsert_game_week_players(&pool, &game_week_players).await?;
        println!("Upsert {idx} took: {:?}", upsert_start.elapsed());
    }

    println!("Total execution time: {:?}", total_start.elapsed());
    Ok(())
}
