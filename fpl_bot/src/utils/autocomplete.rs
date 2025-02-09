use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use poise::serenity_prelude as serenity;

use crate::Context;

pub async fn autocomplete_mini_league<'a>(
    ctx: Context<'_>,
    partial: &'a str,
) -> impl Iterator<Item = serenity::AutocompleteChoice> + 'a {
    let mini_league_names = match sqlx::query!(
        "SELECT name, id FROM discord_user_mini_leagues WHERE discord_id = $1 AND name IS NOT NULL",
        ctx.author().id.get() as i64
    )
    .map(|row| (row.name.unwrap(), row.id.unwrap()))
    .fetch_all(&*ctx.data().pool)
    .await
    {
        Ok(names) => names,
        Err(_) => Vec::new(),
    };

    mini_league_names
        .into_iter()
        .filter(|(name, _)| name.to_lowercase().starts_with(&partial.to_lowercase()))
        .map(|(name, id)| serenity::AutocompleteChoice::new(format!("{name}"), id))
}

pub async fn autocomplete_player<'a>(
    ctx: Context<'_>,
    partial: &'a str,
) -> impl Iterator<Item = serenity::AutocompleteChoice> + 'a {
    let player_names = match sqlx::query!("SELECT web_name, id FROM players",)
        .map(|row| (row.web_name, row.id))
        .fetch_all(&*ctx.data().pool)
        .await
    {
        Ok(names) => names,
        Err(_) => Vec::new(),
    };

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
