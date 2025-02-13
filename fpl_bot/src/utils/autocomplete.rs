use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use poise::serenity_prelude as serenity;

use crate::Context;

pub async fn autocomplete_mini_league<'a>(
    ctx: Context<'_>,
    partial: &'a str,
) -> impl Iterator<Item = serenity::AutocompleteChoice> + 'a {
    get_mini_league_name_autocompletes(ctx, partial, false).await
}

async fn get_mini_league_name_autocompletes<'a>(
    ctx: Context<'_>,
    partial: &'a str,
    as_string: bool,
) -> impl Iterator<Item = serenity::AutocompleteChoice> + 'a {
    let mini_league_names = (sqlx::query!(
        "SELECT name, id FROM discord_user_mini_leagues WHERE discord_id = $1 AND name IS NOT NULL",
        ctx.author().id.get() as i64
    )
    .map(|row| (row.name.unwrap(), row.id.unwrap()))
    .fetch_all(&*ctx.data().pool)
    .await)
        .unwrap_or_default();

    mini_league_names
        .into_iter()
        .filter(|(name, _)| name.to_lowercase().starts_with(&partial.to_lowercase()))
        .map(move |(name, id)| match as_string {
            true => serenity::AutocompleteChoice::new(name, id.to_string()),
            false => serenity::AutocompleteChoice::new(name, id),
        })
}

pub async fn autocomplete_player<'a>(
    ctx: Context<'_>,
    partial: &'a str,
) -> impl Iterator<Item = serenity::AutocompleteChoice> + 'a {
    let player_names = (sqlx::query!("SELECT web_name, id FROM players",)
        .map(|row| (row.web_name, row.id))
        .fetch_all(&*ctx.data().pool)
        .await)
        .unwrap_or_default();

    let matcher = SkimMatcherV2::default();
    let partial_lower = partial.to_lowercase();

    let mut matches: Vec<_> = player_names
        .into_iter()
        .filter_map(|(name, id)| {
            let name_lower = name.to_lowercase();
            matcher
                .fuzzy_match(&name_lower, &partial_lower)
                .map(|score| (score, name, id))
        })
        .collect();

    matches.sort_by(|a, b| b.0.cmp(&a.0));

    matches
        .into_iter()
        .map(|(_, name, id)| serenity::AutocompleteChoice::new(name, id))
}

pub async fn autocomplete_league_or_user<'a>(
    _ctx: Context<'_>,
    _partial: &'a str,
) -> impl Iterator<Item = String> + 'a {
    ["League".to_string(), "User".to_string()].into_iter()
}

pub async fn autocomplete_league_or_user_value<'a>(
    ctx: Context<'_>,
    partial: &'a str,
) -> impl Iterator<Item = serenity::AutocompleteChoice> + 'a {
    // Access the interaction from context
    let interaction = match ctx {
        Context::Application(ctx) => &ctx.interaction.clone(),
        _ => {
            return vec![].into_iter();
        }
    };

    let guild_id = if let Some(guild_id) = interaction.guild_id {
        guild_id
    } else {
        return vec![].into_iter();
    };

    println!("guild_id: {}", guild_id);

    let league_or_user = interaction
        .data
        .options
        .iter()
        .find(|opt| opt.name == "league_or_user")
        .and_then(|opt| match &opt.value {
            serenity::CommandDataOptionValue::String(s) => Some(s.as_str()),
            _ => None,
        })
        .unwrap_or("");

    println!("league_or_user: {}", league_or_user);

    let choices = match league_or_user {
        "League" => get_mini_league_name_autocompletes(ctx, partial, true)
            .await
            .collect::<Vec<_>>(),
        "User" => {
            match guild_id
                .members(&ctx.serenity_context().http, None, None)
                .await
            {
                Ok(members) => {
                    println!("members {:#?}", members);
                    members
                        .into_iter()
                        .filter_map(|member| {
                            let name = member.display_name().to_string();
                            if name.to_lowercase().contains(&partial.to_lowercase()) {
                                Some(serenity::AutocompleteChoice::new(
                                    name,
                                    member.user.id.to_string(),
                                ))
                            } else {
                                None
                            }
                        })
                        .take(25)
                        .collect::<Vec<_>>()
                }
                Err(err) => {
                    println!("{:#?}", err);
                    vec![]
                }
            }
        }
        _ => vec![],
    };

    choices.into_iter()
}
