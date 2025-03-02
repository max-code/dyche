use crate::Context;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use poise::serenity_prelude as serenity;

pub(crate) async fn get_mini_league_name_autocompletes<'a>(
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

pub(crate) async fn get_club_name_autocompletes<'a>(
    ctx: Context<'_>,
    partial: &'a str,
    as_string: bool,
) -> impl Iterator<Item = serenity::AutocompleteChoice> + 'a {
    let club_names = (sqlx::query!("SELECT name, id FROM clubs ORDER BY name ASC")
        .map(|row| (row.name, row.id))
        .fetch_all(&*ctx.data().pool)
        .await)
        .unwrap_or_default();

    club_names
        .into_iter()
        .filter(|(name, _)| name.to_lowercase().starts_with(&partial.to_lowercase()))
        .map(move |(name, id)| match as_string {
            true => serenity::AutocompleteChoice::new(name, id.to_string()),
            false => serenity::AutocompleteChoice::new(name, id),
        })
}

pub(crate) async fn get_player_name_autocompletes<'a>(
    ctx: Context<'_>,
    partial: &'a str,
    as_string: bool,
) -> impl Iterator<Item = serenity::AutocompleteChoice> + 'a {
    let player_names = (sqlx::query!(
        "SELECT first_name, second_name, web_name, id FROM players ORDER BY web_name ASC",
    )
    .map(|row| {
        (
            format!("{} {} ({})", row.first_name, row.second_name, row.web_name),
            row.id,
        )
    })
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
        .map(move |(_, name, id)| match as_string {
            true => serenity::AutocompleteChoice::new(name, id.to_string()),
            false => serenity::AutocompleteChoice::new(name, id),
        })
}
