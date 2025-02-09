use crate::utils::embed_builder::{EmbedBuilder, Processing};
use crate::{Context, Error};
use fpl_api::responses::mini_league::{MiniLeagueResponse, Standing};
use fpl_api::responses::team::{ClassicLeague, TeamResponse};
use fpl_db::queries::game_week::get_current_game_week;
use fpl_db::queries::team_game_week::{
    upsert_team_game_week_automatic_subs, upsert_team_game_week_picks, upsert_team_game_weeks,
};
use futures::StreamExt;
use sqlx::PgPool;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
use tracing::info;

use poise::{CreateReply, ReplyHandle};

use fpl_api::requests::{MiniLeagueRequest, TeamGameWeekRequest, TeamRequest};
use fpl_api::FplClient;
use fpl_common::types::{GameWeekId, LeagueId, TeamId};
use fpl_db::models::{DiscordUser, MiniLeague, MiniLeagueStanding, Team};
use fpl_db::queries::discord::{get_discord_user, insert_discord_user};
use fpl_db::queries::mini_league::{upsert_mini_league_standings, upsert_mini_leagues};
use fpl_db::queries::team::upsert_teams;

const REGISTER_COMMAND: &str = "/register";

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

    let client = Arc::new(FplClient::new());

    if let Err(err) =
        check_user_registered(&ctx, &message, embed.clone(), Arc::clone(&ctx.data().pool)).await
    {
        return Err(err);
    }

    // TODOS:
    // - Get all the transfers, subs, picks, game weeks, etc etc JUST for the registering user
    //   Can 'lazily' get that data for everyone else in their league

    let embed = embed.update("Fetching team information.");
    message
        .edit(ctx, CreateReply::default().embed(embed.clone().build()))
        .await?;

    let team =
        match get_team_information(&ctx, &message, embed.clone(), Arc::clone(&client), team_id)
            .await
        {
            Ok(team) => team,
            Err(err) => return Err(err),
        };

    let team_db: Vec<Team> = vec![team.clone().into()];
    let teams_upsert = upsert_teams(&ctx.data().pool, &team_db).await;
    if let Err(err) = teams_upsert {
        return Err(err.into());
    }

    if let Err(err) = get_and_upsert_team_game_week_information(
        &ctx,
        &message,
        embed.clone(),
        Arc::clone(&client),
        Arc::clone(&ctx.data().pool),
        team_id,
    )
    .await
    {
        return Err(err);
    };

    let embed = embed.update("Fetching related mini league and team information.");
    message
        .edit(ctx, CreateReply::default().embed(embed.clone().build()))
        .await?;

    if let Err(err) = get_and_upsert_related_mini_leagues_and_teams(
        &ctx,
        &message,
        embed.clone(),
        Arc::clone(&client),
        Arc::clone(&ctx.data().pool),
        team.leagues.classic,
    )
    .await
    {
        return Err(err);
    };

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
    pool: Arc<PgPool>,
) -> Result<(), Error> {
    // If the discord user is already registered, cant reregister
    match get_discord_user(&pool, ctx.author().id.into()).await {
        Err(err) => {
            message
                .edit(
                    *ctx,
                    CreateReply::default().embed(embed.clone().error("Error registering.").build()),
                )
                .await?;
            Err(format!(
                "Error checking if discord user already exists when processing command {}: {}",
                REGISTER_COMMAND, err
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

async fn get_team_information(
    ctx: &Context<'_>,
    message: &ReplyHandle<'_>,
    embed: EmbedBuilder<Processing>,
    client: Arc<FplClient>,
    team_id: TeamId,
) -> Result<TeamResponse, Error> {
    let team_response = client.get(TeamRequest::new(team_id)).await;
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

    Ok(team)
}

async fn get_and_upsert_team_game_week_information(
    ctx: &Context<'_>,
    message: &ReplyHandle<'_>,
    embed: EmbedBuilder<Processing>,
    client: Arc<FplClient>,
    pool: Arc<PgPool>,
    team_id: TeamId,
) -> Result<(), Error> {
    let current_game_week = match get_current_game_week(&pool).await {
        Ok(current_game_week) => current_game_week,
        Err(err) => return Err(err.into()),
    };

    let current_week = i16::from(current_game_week.id);
    let game_week_range = GameWeekId::weeks_range_iter(1, current_week);

    let mut stream = futures::stream::iter(game_week_range)
        .map(|game_week_id| {
            let client = Arc::clone(&client);
            async move {
                client
                    .get(TeamGameWeekRequest::new(team_id, game_week_id))
                    .await
            }
        })
        .buffer_unordered(5);

    let mut game_week_picks = Vec::with_capacity(current_week as usize * 15);
    let mut game_week_automatic_subs = Vec::with_capacity(current_week as usize * 4);
    let mut team_game_weeks = Vec::with_capacity(current_week as usize);

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
                                .error("Error fetching weekly team information when registering.")
                                .build(),
                        ),
                    )
                    .await?;
                return Err(e.into());
            }
        };
        let game_week_id = match response.game_week_id {
            Some(game_week_id) => game_week_id,
            None => return Err("Error getting game_week_id from TeamGameWeekResponse".into()),
        };

        game_week_picks.extend(
            response
                .picks
                .iter()
                .map(|pick| (team_id, game_week_id, pick).into()),
        );

        game_week_automatic_subs.extend(response.automatic_subs.iter().map(|sub| sub.into()));

        team_game_weeks.push((team_id, game_week_id, &response).into());
    }
    let tgw_upsert = upsert_team_game_weeks(&pool, &team_game_weeks).await;
    if let Err(err) = tgw_upsert {
        return Err(err.into());
    }

    let tgw_picks_upsert = upsert_team_game_week_picks(&pool, &game_week_picks).await;
    if let Err(err) = tgw_picks_upsert {
        return Err(err.into());
    }

    let tgw_subs_upsert =
        upsert_team_game_week_automatic_subs(&pool, &game_week_automatic_subs).await;
    if let Err(err) = tgw_subs_upsert {
        return Err(err.into());
    }

    Ok(())
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
    client: Arc<FplClient>,
    user_league_ids: HashSet<LeagueId>,
) -> Result<(Vec<MiniLeague>, Vec<MiniLeagueStanding>), Error> {
    let mut stream = futures::stream::iter(user_league_ids)
        .map(|league_id| {
            let client = Arc::clone(&client);
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
                return Err(e.into());
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
    client: Arc<FplClient>,
) -> Result<Vec<Team>, Error> {
    let mut related_teams = Vec::new();

    let mut stream = futures::stream::iter(all_team_ids)
        .map(|team_id| {
            let client = Arc::clone(&client);
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
    client: Arc<FplClient>,
    pool: Arc<PgPool>,
    mini_leagues: Vec<ClassicLeague>,
) -> Result<(), Error> {
    // GET ALL THE LEAGUES THEYRE IN
    let user_league_ids: HashSet<LeagueId> = mini_leagues
        .iter()
        .filter(|league| league.admin_entry.is_some() && league.rank_count <= 100)
        .map(|league| LeagueId::new(league.id))
        .collect();

    let (leagues_info, leagues_standing_info) = match get_mini_leagues(
        &ctx,
        &message,
        embed.clone(),
        Arc::clone(&client),
        user_league_ids,
    )
    .await
    {
        Ok((leagues_info, leagues_standing_info)) => (leagues_info, leagues_standing_info),
        Err(err) => return Err(err),
    };

    let ml_upsert = upsert_mini_leagues(&pool, &leagues_info).await;
    if let Err(err) = ml_upsert {
        return Err(err.into());
    }

    let mls_upsert = upsert_mini_league_standings(&pool, &leagues_standing_info).await;
    if let Err(err) = mls_upsert {
        return Err(err.into());
    }

    let all_team_ids = leagues_standing_info
        .iter()
        .map(|standing| TeamId::from(standing.team_id))
        .collect();

    let all_teams = match get_all_related_teams(
        &ctx,
        &message,
        embed.clone(),
        all_team_ids,
        Arc::clone(&client),
    )
    .await
    {
        Ok(all_teams) => all_teams,
        Err(err) => return Err(err),
    };

    let teams_upsert = upsert_teams(&pool, &all_teams).await;
    if let Err(err) = teams_upsert {
        return Err(err.into());
    }

    Ok(())
}
