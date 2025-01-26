use fpl_api::requests::TeamGameWeekRequest;
use fpl_api::FplClient;
use fpl_common::types::{GameWeekId, TeamId};
use fpl_db::models::{TeamGameWeek, TeamGameWeekAutomaticSub, TeamGameWeekPick};
use fpl_db::queries::team::get_all_team_ids;
use fpl_db::queries::team_game_week::{
    upsert_team_game_week, upsert_team_game_week_automatic_subs, upsert_team_game_week_picks,
};
use sqlx::PgPool;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let total_start = Instant::now();
    let database_url = dotenv::var("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url).await?;
    let client = FplClient::new();

    let ids = get_all_team_ids(&pool).await?;

    let mut team_game_week_requests = vec![];
    for team_id in ids {
        for game_week_id in GameWeekId::weeks_range_iter(1, 23) {
            team_game_week_requests.push((team_id, game_week_id));
        }
    }
    let team_game_week_chunks: Vec<_> = team_game_week_requests
        .chunks(10)
        .map(|c| c.to_vec())
        .collect();

    for chunk in team_game_week_chunks {
        let futures: Vec<_> = chunk
            .into_iter()
            .map(|team_gw_pair| process_team_game_week(pool.clone(), client.clone(), team_gw_pair))
            .collect();
        futures::future::join_all(futures).await;
    }

    println!("Total execution time: {:?}", total_start.elapsed());
    Ok(())
}

async fn process_team_game_week(
    pool: PgPool,
    client: FplClient,
    (team_id, game_week_id): (TeamId, GameWeekId),
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Making request for Team ID {team_id}, GW {game_week_id}");
    let team_game_week_response = client
        .get(TeamGameWeekRequest::new(team_id, game_week_id))
        .await?;

    // Process tgw
    let team_game_week: TeamGameWeek = (team_id, game_week_id, &team_game_week_response).into();
    upsert_team_game_week(&pool, &team_game_week).await?;

    // Process tgw picks
    let team_game_week_picks: Vec<TeamGameWeekPick> = team_game_week_response
        .picks
        .iter()
        .map(|pick| (team_id, game_week_id, pick).into())
        .collect();
    upsert_team_game_week_picks(&pool, &team_game_week_picks).await?;

    // Process tgw auto subs
    let team_game_week_auto_subs: Vec<TeamGameWeekAutomaticSub> = team_game_week_response
        .automatic_subs
        .iter()
        .map(|auto_sub| auto_sub.into())
        .collect();
    upsert_team_game_week_automatic_subs(&pool, &team_game_week_auto_subs).await?;

    Ok(())
}
