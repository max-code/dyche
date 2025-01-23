use fpl_api::requests::FixtureRequest;
use fpl_api::FplClient;
use fpl_db::models::fixtures::FixturesRow;
use fpl_db::queries::fixtures::upsert_fixtures;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_filename(".env").ok();
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    let pool = PgPool::connect(&database_url).await?;

    let client = FplClient::new();
    let request = FixtureRequest::new();
    let fixtures = client.get(request).await.unwrap();

    let fixtures_rows: Vec<FixturesRow> = fixtures
        .into_iter()
        .map(|f| f.try_into().unwrap())
        .collect();
    upsert_fixtures(&pool, &fixtures_rows).await?;

    Ok(())
}
