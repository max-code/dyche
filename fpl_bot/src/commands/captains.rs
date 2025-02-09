use crate::utils::autocomplete::autocomplete_mini_league;
use crate::utils::embed_builder::EmbedBuilder;
use crate::{Context, Error};

use fpl_common::types::{LeagueId, PlayerId};
use fpl_db::queries::game_week::get_current_game_week;
use poise::CreateReply;
use tracing::info;

const COMMAND: &str = "/captains";

#[poise::command(slash_command)]
pub async fn captains(
    ctx: Context<'_>,
    #[description = "Who to greet"]
    #[autocomplete = "autocomplete_mini_league"]
    league_id: LeagueId,
) -> Result<(), Error> {
    info!(
        "{} called by {} with league_id({})",
        COMMAND,
        ctx.author().id,
        league_id
    );

    let embed = EmbedBuilder::new(
        COMMAND,
        format!("Fetching captains for Mini League ID {}", league_id).as_str(),
    );

    let message = ctx
        .send(CreateReply::default().embed(embed.clone().build()))
        .await?;

    let captains = get_captains(&ctx, league_id).await?;
    let embed = embed.success(captains.as_str()).build();
    message
        .edit(ctx, CreateReply::default().embed(embed))
        .await?;

    Ok(())
}

async fn get_captains(ctx: &Context<'_>, league_id: LeagueId) -> Result<String, Error> {
    let current_game_week = get_current_game_week(&ctx.data().pool).await?;
    let captains = sqlx::query!(
        r#"
        SELECT
            mls.player_name,
            p.id as "id!",
            p.web_name
        FROM mini_league_standings mls
        JOIN mini_leagues ml ON mls.league_id = ml.id
        JOIN team_game_week_picks tgwp ON tgwp.team_id = mls.team_id
        JOIN players p ON p.id = tgwp.player_id
        WHERE tgwp.game_week_id = $1
        AND ml.id = $2
        AND tgwp.is_captain = true
        "#,
        i16::from(current_game_week.id),
        i32::from(league_id)
    )
    .fetch_all(&*ctx.data().pool)
    .await
    .map(|rows| {
        rows.into_iter()
            .map(|row| (row.player_name, PlayerId::from(row.id), row.web_name))
            .collect::<Vec<(String, PlayerId, String)>>()
    })?;

    let captains_text = captains
        .into_iter()
        .map(|(player_name, _, web_name)| format!("**{}** captained **{}**", player_name, web_name))
        .collect::<Vec<String>>()
        .join("\n");

    Ok(captains_text)
}
