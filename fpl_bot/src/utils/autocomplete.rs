use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use poise::serenity_prelude as serenity;

use crate::Context;

pub async fn autocomplete_mini_league<'a>(
    ctx: Context<'_>,
    partial: &'a str,
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
        .map(|(name, id)| serenity::AutocompleteChoice::new(name, id))
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

pub async fn autocomplete_league_or_player<'a>(
    _ctx: Context<'_>,
    _partial: &'a str,
) -> impl Iterator<Item = String> + 'a {
    ["League".to_string(), "User".to_string()].into_iter()
}

pub async fn autocomplete_league_or_player_stub<'a>(
    ctx: Context<'_>,
    partial: &'a str,
) -> impl Iterator<Item = String> + 'a {
    // Access the interaction from context
    let interaction = match ctx {
        Context::Application(ctx) => &ctx.interaction.clone(),
        _ => {
            return vec![].into_iter();
        }
    };

    // Get arg1's value from the interaction data
    let league_or_user = interaction
        .data
        .options
        .iter()
        .find(|opt| opt.name == "arg1")
        .and_then(|opt| match &opt.value {
            serenity::CommandDataOptionValue::String(s) => Some(s.as_str()),
            _ => None,
        })
        .unwrap_or("");

    // Return appropriate choices based on arg1
    match league_or_user {
        "League" => (1..=5)
            .map(|n| n.to_string())
            .filter(|n| n.starts_with(partial))
            .collect::<Vec<_>>(),
        "User" => vec!["10", "20", "30", "40", "50"]
            .into_iter()
            .map(String::from)
            .filter(|n| n.starts_with(partial))
            .collect::<Vec<_>>(),
        _ => vec![],
    }
    .into_iter()
}
