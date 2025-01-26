use fpl_api::requests::FixtureRequest;
use fpl_api::FplClient;
use fpl_db::models::fixture::Fixture;
use fpl_db::queries::fixture::upsert_fixtures;
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
    let request = FixtureRequest::new();

    let api_start = Instant::now();
    let fixtures = client.get(request).await.unwrap();
    println!("API request took: {:?}", api_start.elapsed());

    let conversion_start = Instant::now();
    let fixtures_rows: Vec<Fixture> = fixtures.iter().map(|f| f.into()).collect();
    println!("Conversion took: {:?}", conversion_start.elapsed());

    let upsert_start = Instant::now();
    upsert_fixtures(&pool, &fixtures_rows).await?;
    println!("Upsert took: {:?}", upsert_start.elapsed());

    println!("Total execution time: {:?}", total_start.elapsed());
    Ok(())
}
