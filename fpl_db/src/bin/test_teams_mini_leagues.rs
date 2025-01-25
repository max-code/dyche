use fpl_api::requests::{MiniLeagueRequest, TeamRequest};
use fpl_api::FplClient;
use fpl_common::types::{LeagueId, TeamId};
use fpl_db::models::{MiniLeague, MiniLeagueStanding, Team};
use fpl_db::queries::mini_league::{upsert_mini_league_standings, upsert_mini_leagues};
use fpl_db::queries::team::upsert_teams;
use sqlx::PgPool;
use std::collections::HashSet;
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

    // USER REGISTERS
    let client = FplClient::new();
    let registering_user = TeamId::new(5040430);
    let request = TeamRequest::new(registering_user);

    let api_start = Instant::now();
    let team_response = client.get(request).await.unwrap();
    println!("Team API request took: {:?}", api_start.elapsed());

    let conversion_start = Instant::now();
    let team_row: Team = (&team_response).try_into().unwrap();
    println!("Team Conversion took: {:?}", conversion_start.elapsed());

    let mut teams = vec![team_row];

    // GET ALL THE LEAGUES THEYRE IN
    let user_league_ids: HashSet<LeagueId> = team_response
        .leagues
        .classic
        .iter()
        .filter(|league| league.admin_entry.is_some() && league.rank_count <= 100)
        .map(|league| LeagueId::new(league.id))
        .collect();

    // FOR ALL THE LEAGUES THEYRE IN, GET ALL OF THE INFO ABOUT THE LEAGUE
    // AND THE STANDINGS
    let mut mini_leagues = vec![];
    let mut mini_league_standings = vec![];

    for mini_league in user_league_ids {
        let request = MiniLeagueRequest::new(mini_league);

        let api_start = Instant::now();
        let mini_league_response = client.get(request).await.unwrap();
        println!("ML API request took: {:?}", api_start.elapsed());

        let conversion_start = Instant::now();
        let mini_league_row: MiniLeague = (&mini_league_response).try_into().unwrap();
        println!("ML Conversion took: {:?}", conversion_start.elapsed());

        mini_leagues.push(mini_league_row);

        for standing in mini_league_response.standings.results.iter() {
            let mini_league_standing: MiniLeagueStanding =
                (&mini_league_response.league.id, standing)
                    .try_into()
                    .unwrap();
            mini_league_standings.push(mini_league_standing);
        }
    }

    // USE THE STANDINGS TO GET ALL PLAYERS IN ALL OF THEIR LEAGUES
    let mini_league_standings_team_ids = mini_league_standings
        .iter()
        .map(|standing| TeamId::from(standing.team_id))
        .collect::<Vec<TeamId>>();

    // GET THE TEAM INFO FOR ALL OF THE PLAYERS IN ALL OF THEIR LEAGUES
    for team_id in mini_league_standings_team_ids.iter() {
        let request = TeamRequest::new(*team_id);

        let api_start = Instant::now();
        let team_response = client.get(request).await.unwrap();
        println!("Team API request took: {:?}", api_start.elapsed());

        let conversion_start = Instant::now();
        let team_row: Team = (&team_response).try_into().unwrap();
        println!("Team Conversion took: {:?}", conversion_start.elapsed());

        teams.push(team_row);
    }

    // UPSERT MINI LEAGUES AND TEAMS
    let upsert_ml_start = Instant::now();
    upsert_mini_leagues(&pool, mini_leagues.as_slice()).await?;
    println!("ML Upsert took: {:?}", upsert_ml_start.elapsed());

    let upsert_start = Instant::now();
    upsert_teams(&pool, teams.as_slice()).await?;
    println!("Team Upsert took: {:?}", upsert_start.elapsed());

    // UPSERT MINI LEAGUE STANDINGS LAST
    let upsert_mls_start = Instant::now();
    upsert_mini_league_standings(&pool, mini_league_standings.as_slice()).await?;
    println!("MLS Upsert took: {:?}", upsert_mls_start.elapsed());

    println!("Total execution time: {:?}", total_start.elapsed());
    Ok(())
}
