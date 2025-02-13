use crate::utils::embed_builder::{EmbedBuilder, Processing};
use crate::{Context, Error};

use fpl_api::responses::mini_league::{MiniLeagueResponse, Standing};
use fpl_api::responses::team::{ClassicLeague, TeamResponse};
use fpl_db::queries::game_week::get_current_game_week;
use fpl_db::queries::team_game_week::{
    upsert_team_game_week_automatic_subs, upsert_team_game_week_picks, upsert_team_game_weeks,
};
use futures::StreamExt;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
use tracing::{info, warn};

use poise::{CreateReply, ReplyHandle};

use fpl_api::requests::{MiniLeagueRequest, TeamGameWeekRequest, TeamRequest};
use fpl_api::FplClient;
use fpl_common::types::{GameWeekId, LeagueId, TeamId};
use fpl_db::models::{DiscordUser, MiniLeague, MiniLeagueStanding, Team};
use fpl_db::queries::discord::{get_discord_user, insert_discord_user};
use fpl_db::queries::mini_league::{upsert_mini_league_standings, upsert_mini_leagues};
use fpl_db::queries::team::upsert_teams;

const COMMAND: &str = "/register";
const MAX_MINI_LEAGUE_ENTRIES: i32 = 25;

#[poise::command(slash_command)]
pub async fn register(
    ctx: Context<'_>,
    #[description = "Team ID from the FPL website"] team_id: TeamId,
) -> Result<(), Error> {
    info!("{} called by {}", COMMAND, ctx.author().id);

    let embed = EmbedBuilder::new(
        COMMAND,
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

    check_user_registered(&ctx, &message, embed.clone()).await?;

    let embed = embed.update("Fetching team information.");
    message
        .edit(ctx, CreateReply::default().embed(embed.clone().build()))
        .await?;

    let team = get_and_upsert_team_information(&ctx, &message, embed.clone(), team_id).await?;
    get_and_upsert_team_game_week_information(&ctx, &message, embed.clone(), team_id).await?;

    let embed = embed.update("Fetching related mini league and team information.");
    message
        .edit(ctx, CreateReply::default().embed(embed.clone().build()))
        .await?;

    get_and_upsert_related_mini_leagues_and_teams(
        &ctx,
        &message,
        embed.clone(),
        team.leagues.classic,
    )
    .await?;

    let discord_user = DiscordUser::new(ctx.author().id.into(), team_id);
    insert_discord_user(&ctx.data().pool, &discord_user).await?;

    let embed = embed
        .success(format!("Registered Team ID {}.", team_id).as_str())
        .build();

    message
        .edit(ctx, CreateReply::default().embed(embed))
        .await?;

    Ok(())
}

async fn check_user_registered(
    ctx: &Context<'_>,
    message: &ReplyHandle<'_>,
    embed: EmbedBuilder<Processing>,
) -> Result<(), Error> {
    // If the discord user is already registered, cant reregister
    match get_discord_user(&ctx.data().pool, ctx.author().id.into()).await {
        Err(err) => {
            message
                .edit(
                    *ctx,
                    CreateReply::default().embed(embed.clone().error("Error registering.").build()),
                )
                .await?;
            Err(format!(
                "Error checking if discord user already exists when processing command {}: {}",
                COMMAND, err
            )
            .into())
        }
        Ok(maybe_discord_user) => match maybe_discord_user {
            Some(_) => {
                message
                    .edit(
                        *ctx,
                        CreateReply::default()
                            .embed(embed.clone().error("User already registered.").build()),
                    )
                    .await?;
                Err("User already registered".into())
            }
            None => Ok(()),
        },
    }
}

async fn get_and_upsert_team_information(
    ctx: &Context<'_>,
    message: &ReplyHandle<'_>,
    embed: EmbedBuilder<Processing>,
    team_id: TeamId,
) -> Result<TeamResponse, Error> {
    let team_response = ctx.data().client.get(TeamRequest::new(team_id)).await;
    let team = match team_response {
        Ok(team) => team,
        Err(err) => {
            let embed = embed
                .clone()
                .error(format!("Failed to get team from FPL for Team ID {}", team_id).as_str());
            message
                .edit(*ctx, CreateReply::default().embed(embed.clone().build()))
                .await?;
            return Err(err.into());
        }
    };

    upsert_teams(&ctx.data().pool, &[team.clone().into()]).await?;
    Ok(team)
}

async fn process_team_game_week_data(
    pool: &sqlx::PgPool,
    client: &FplClient,
    team_id: TeamId,
) -> Result<(), Error> {
    let current_game_week = get_current_game_week(pool).await?.id.into();
    let game_week_range = GameWeekId::weeks_range_iter(1, current_game_week);

    let mut stream = futures::stream::iter(game_week_range)
        .map(|game_week_id| {
            let client = client.clone();
            async move {
                client
                    .get(TeamGameWeekRequest::new(team_id, game_week_id))
                    .await
            }
        })
        .buffer_unordered(5); // 5 concurrent requests per team

    let mut game_week_picks = Vec::new();
    let mut game_week_automatic_subs = Vec::new();
    let mut team_game_weeks = Vec::new();

    while let Some(result) = stream.next().await {
        let response = match result {
            Ok(response) => response,
            Err(e) => {
                warn!("{}", e);
                continue;
            }
        };
        let game_week_id = response
            .game_week_id
            .ok_or("Missing game_week_id in response")?;

        game_week_picks.extend(
            response
                .picks
                .iter()
                .map(|pick| (team_id, game_week_id, pick).into()),
        );

        game_week_automatic_subs.extend(response.automatic_subs.iter().map(|sub| sub.into()));

        team_game_weeks.push((team_id, game_week_id, &response).into());
    }

    upsert_team_game_weeks(pool, &team_game_weeks).await?;
    upsert_team_game_week_picks(pool, &game_week_picks).await?;
    upsert_team_game_week_automatic_subs(pool, &game_week_automatic_subs).await?;

    Ok(())
}

async fn get_and_upsert_team_game_week_information(
    ctx: &Context<'_>,
    message: &ReplyHandle<'_>,
    embed: EmbedBuilder<Processing>,
    team_id: TeamId,
) -> Result<(), Error> {
    let embed = embed.update("Fetching team game week information.");
    message
        .edit(*ctx, CreateReply::default().embed(embed.clone().build()))
        .await?;

    process_team_game_week_data(&ctx.data().pool, &ctx.data().client, team_id).await
}

async fn handle_mini_league_requests(
    client: Arc<FplClient>,
    league_id: LeagueId,
) -> Result<(MiniLeagueResponse, Vec<Standing>), Error> {
    let mut mini_league_standings: Vec<Standing> = Vec::new();
    let mut page = 1;
    let mut current_page = client.get(MiniLeagueRequest::new(league_id, page)).await?;
    mini_league_standings.extend(current_page.standings.results.clone());
    while current_page.standings.has_next {
        tokio::time::sleep(Duration::from_millis(100)).await;
        page += 1;
        current_page = client.get(MiniLeagueRequest::new(league_id, page)).await?;
        mini_league_standings.extend(current_page.standings.results.clone());
    }

    Ok((current_page, mini_league_standings))
}

async fn get_mini_leagues(
    ctx: &Context<'_>,
    message: &ReplyHandle<'_>,
    embed: EmbedBuilder<Processing>,
    user_league_ids: HashSet<LeagueId>,
) -> Result<(Vec<MiniLeague>, Vec<MiniLeagueStanding>), Error> {
    let mut stream = futures::stream::iter(user_league_ids)
        .map(|league_id| {
            let client = Arc::clone(&ctx.data().client);
            async move { handle_mini_league_requests(client, league_id).await }
        })
        .buffer_unordered(5);

    let mut leagues_info: Vec<MiniLeague> = Vec::new();
    let mut leagues_standing_info: Vec<MiniLeagueStanding> = Vec::new();

    while let Some(result) = stream.next().await {
        let response = match result {
            Ok(response) => response,
            Err(e) => {
                message
                    .edit(
                        *ctx,
                        CreateReply::default().embed(
                            embed
                                .clone()
                                .error("Error fetching related mini leagues when registering.")
                                .build(),
                        ),
                    )
                    .await?;
                return Err(e);
            }
        };

        let (league, standings) = response;

        leagues_info.push((&league).into());
        leagues_standing_info.extend(
            standings
                .into_iter()
                .map(|standing| (&league.league.id, &standing).into())
                .collect::<Vec<MiniLeagueStanding>>(),
        )
    }

    Ok((leagues_info, leagues_standing_info))
}

async fn get_all_related_teams(
    ctx: &Context<'_>,
    message: &ReplyHandle<'_>,
    embed: EmbedBuilder<Processing>,
    all_team_ids: Vec<TeamId>,
) -> Result<Vec<Team>, Error> {
    let mut related_teams = Vec::new();

    let mut stream = futures::stream::iter(all_team_ids)
        .map(|team_id| {
            let client = Arc::clone(&ctx.data().client);
            async move { client.get(TeamRequest::new(team_id)).await }
        })
        .buffer_unordered(5);

    while let Some(result) = stream.next().await {
        let response = match result {
            Ok(response) => response,
            Err(e) => {
                message
                    .edit(
                        *ctx,
                        CreateReply::default().embed(
                            embed
                                .clone()
                                .error("Error fetching related teams when registering.")
                                .build(),
                        ),
                    )
                    .await?;
                return Err(e.into());
            }
        };

        related_teams.push(response.into());
    }

    Ok(related_teams)
}

async fn get_and_upsert_related_mini_leagues_and_teams(
    ctx: &Context<'_>,
    message: &ReplyHandle<'_>,
    embed: EmbedBuilder<Processing>,
    mini_leagues: Vec<ClassicLeague>,
) -> Result<(), Error> {
    // GET ALL THE LEAGUES THEYRE IN
    let user_league_ids: HashSet<LeagueId> = mini_leagues
        .iter()
        .filter(|league| {
            league.admin_entry.is_some() && league.rank_count <= MAX_MINI_LEAGUE_ENTRIES
        })
        .map(|league| LeagueId::new(league.id))
        .collect();

    let (leagues_info, leagues_standing_info) =
        get_mini_leagues(ctx, message, embed.clone(), user_league_ids).await?;

    upsert_mini_leagues(&ctx.data().pool, &leagues_info).await?;
    upsert_mini_league_standings(&ctx.data().pool, &leagues_standing_info).await?;

    let all_team_ids = leagues_standing_info
        .iter()
        .map(|standing| standing.team_id)
        .collect();

    let all_teams = get_all_related_teams(ctx, message, embed.clone(), all_team_ids).await?;
    upsert_teams(&ctx.data().pool, &all_teams).await?;

    // Process each related team's game week data
    let all_team_ids: Vec<TeamId> = all_teams.into_iter().map(|t| t.id).collect();
    let pool = ctx.data().pool.clone();
    let client = ctx.data().client.clone();

    let mut stream = futures::stream::iter(all_team_ids)
        .map(|team_id| {
            let pool = pool.clone();
            let client = client.clone();
            async move {
                if let Err(e) = process_team_game_week_data(&pool, &client, team_id).await {
                    tracing::error!("Error processing game weeks for team {}: {}", team_id, e);
                }
                Ok::<(), Error>(())
            }
        })
        .buffer_unordered(2); // Process 2 teams concurrently

    while let Some(result) = stream.next().await {
        result?;
    }

    Ok(())
}
