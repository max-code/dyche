use fpl_api::requests::GameStateRequest;
use fpl_api::FplClient;
use fpl_db::models::{GameWeek, GameWeekChipPlay, GameWeekTopElement};
use fpl_db::queries::game_week::{
    upsert_game_week_chip_plays, upsert_game_week_top_elements, upsert_game_weeks,
};
use sqlx::PgPool;
use std::time::Instant;
use tracing::{debug, info};

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
    let request = GameStateRequest::new();

    let api_start = Instant::now();
    let game_state = client.get(request).await.unwrap();
    println!("API request took: {:?}", api_start.elapsed());

    let gameweeks = game_state.events;

    /*
    GAME WEEKS
     */
    let conversion_1_start = Instant::now();
    let game_weeks_rows: Vec<GameWeek> = gameweeks.iter().map(|f| f.into()).collect();
    println!(
        "Conversion to GameWeek took: {:?}",
        conversion_1_start.elapsed()
    );

    let upsert_1_start = Instant::now();
    upsert_game_weeks(&pool, &game_weeks_rows).await?;
    println!("Upsert to game_weeks took: {:?}", upsert_1_start.elapsed());

    /*
    CHIPS
     */
    let conversion_2_start = Instant::now();
    let game_weeks_chips_rows: Vec<GameWeekChipPlay> = gameweeks
        .iter()
        .map(|gameweek| GameWeekChipPlay::from_overview(gameweek))
        .flatten()
        .collect();
    println!(
        "Conversion to GameWeek chips playestook: {:?}",
        conversion_2_start.elapsed()
    );

    let upsert_2_start = Instant::now();
    upsert_game_week_chip_plays(&pool, &game_weeks_chips_rows).await?;
    println!(
        "Upsert to game_weeks chip plays took: {:?}",
        upsert_2_start.elapsed()
    );

    /*
    TOP ELEMENTS
    */
    let conversion_3_start = Instant::now();
    let game_weeks_chips_rows: Vec<GameWeekTopElement> = gameweeks
        .iter()
        .filter_map(|gameweek| GameWeekTopElement::from_overview(gameweek))
        .collect();
    println!(
        "Conversion to GameWeek top elements took: {:?}",
        conversion_3_start.elapsed()
    );

    let upsert_3_start = Instant::now();
    upsert_game_week_top_elements(&pool, &game_weeks_chips_rows).await?;
    println!(
        "Upsert to game_weeks top elements took: {:?}",
        upsert_3_start.elapsed()
    );

    println!("Total execution time: {:?}", total_start.elapsed());
    Ok(())
}
