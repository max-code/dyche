use crate::utils::embed_builder::EmbedBuilder;
use crate::{Context, Error};
use std::collections::HashSet;
use tracing::{error, info};

use poise::CreateReply;

use fpl_api::requests::{MiniLeagueRequest, TeamRequest};
use fpl_api::FplClient;
use fpl_common::types::{LeagueId, TeamId};
use fpl_db::models::{DiscordUser, MiniLeague, MiniLeagueStanding, Team};
use fpl_db::queries::discord::{get_discord_user, insert_discord_user};
use fpl_db::queries::mini_league::{upsert_mini_league_standings, upsert_mini_leagues};
use fpl_db::queries::team::upsert_teams;

const REGISTER_COMMAND: &str = "/register";


async fn

#[poise::command(slash_command)]
pub async fn register(
    ctx: Context<'_>,
    #[description = "Team ID from the FPL website"] team_id: TeamId,
) -> Result<(), Error> {
    info!("Register command called");

    let embed = EmbedBuilder::new(
        REGISTER_COMMAND,
        format!(
            "Registering user {} with Team ID {}",
            ctx.author().name,
            team_id
        )
        .as_str(),
    );

    let message = ctx
        .send(CreateReply::default().embed(embed.clone().build()))
        .await?;

    // If the discord user is already registered, cant reregister
    match get_discord_user(&ctx.data().pool, ctx.author().id.into()).await {
        Err(err) => {
            message
                .edit(
                    ctx,
                    CreateReply::default().embed(embed.error("Error registering.").clone().build()),
                )
                .await?;
            return Err(format!(
                "Error checking if discord user already exists when processing command {}: {}",
                REGISTER_COMMAND, err
            )
            .into());
        }
        Ok(maybe_discord_user) => {
            if let Some(_) = maybe_discord_user {
                message
                    .edit(
                        ctx,
                        CreateReply::default()
                            .embed(embed.error("User already registered.").clone().build()),
                    )
                    .await?;
                return Ok(());
            }
        }
    }

    // TODOS:
    // - Properly handle all errors. Get rid of all .unwrap and ?
    // - Split it into function
    // - Get all the transfers, subs, picks, game weeks, etc etc JUST for the registering user
    //   Can 'lazily' get that data for everyone else in their league
    // - Better error handling
    //

    let client = FplClient::new();
    let team_response = client.get(TeamRequest::new(team_id)).await;
    let team = match team_response {
        Ok(team) => team,
        Err(err) => {
            error!("Failed to get team for user: {}", err);

            let embed = embed
                .error(format!("Failed to get team from FPL for Team ID {}", team_id).as_str());
            message
                .edit(ctx, CreateReply::default().embed(embed.clone().build()))
                .await?;
            return Err(err.into());
        }
    };

    let team_row: Team = (&team).into();
    let mut teams = vec![team_row];

    // GET ALL THE LEAGUES THEYRE IN
    let user_league_ids: HashSet<LeagueId> = team
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
        let request = MiniLeagueRequest::new(mini_league, 1);
        let mini_league_response = client.get(request).await.unwrap();
        let mini_league_row: MiniLeague = (&mini_league_response).into();
        mini_leagues.push(mini_league_row);

        for standing in mini_league_response.standings.results.iter() {
            let mini_league_standing: MiniLeagueStanding =
                (&mini_league_response.league.id, standing).into();
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
        let team_response = client.get(request).await.unwrap();
        let team_row: Team = (&team_response).into();
        teams.push(team_row);
    }

    // UPSERT MINI LEAGUES AND TEAMS
    upsert_mini_leagues(&ctx.data().pool, mini_leagues.as_slice()).await?;
    upsert_teams(&ctx.data().pool, teams.as_slice()).await?;
    upsert_mini_league_standings(&ctx.data().pool, mini_league_standings.as_slice()).await?;

    let discord_user = DiscordUser::new(ctx.author().id.into(), team_id);
    insert_discord_user(&ctx.data().pool, &discord_user).await?;

    let embed = embed
        .success(format!("Registered Team ID {}.", team_id).as_str())
        .build()
        .field("Mini League Entries", mini_leagues.len().to_string(), true)
        .field("Associated Teams", teams.len().to_string(), true);

    message
        .edit(ctx, CreateReply::default().embed(embed))
        .await?;

    Ok(())
}
