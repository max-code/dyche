use std::collections::HashSet;

use crate::Context;
use ::serenity::all::Member;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use once_cell::sync::Lazy;
use poise::serenity_prelude as serenity;

static MATCHER: Lazy<SkimMatcherV2> = Lazy::new(SkimMatcherV2::default);

fn get_fuzzy_matches<'a, T>(
    partial: &str,
    rows: Vec<(String, T)>,
    id_as_string: bool,
) -> impl Iterator<Item = serenity::AutocompleteChoice> + 'a
where
    T: ToString + Copy + Into<i64> + 'a,
{
    let partial_lower = partial.to_lowercase();

    let mut matches: Vec<_> = rows
        .into_iter()
        .filter_map(|(name, id)| {
            let name_lower = name.to_lowercase();
            MATCHER
                .fuzzy_match(&name_lower, &partial_lower)
                .map(|score| (score, name, id))
        })
        .collect();

    if matches.len() > 1 {
        matches.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    }

    matches.truncate(8);

    matches
        .into_iter()
        .map(move |(_, name, id)| match id_as_string {
            true => serenity::AutocompleteChoice::new(name, id.to_string()),
            false => serenity::AutocompleteChoice::new(name, id.into()),
        })
}

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

    get_fuzzy_matches(partial, mini_league_names, as_string)
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

    get_fuzzy_matches(partial, club_names, as_string)
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

    get_fuzzy_matches(partial, player_names, as_string)
}

// TODO: Can easily have a cache here for the registered_discord_ids.
// Can even probably just have one for server id: [registered members]
pub(crate) async fn get_registered_users_autocompletes<'a>(
    ctx: Context<'_>,
    members: &[Member],
    partial: &'a str,
) -> impl Iterator<Item = serenity::AutocompleteChoice> + 'a {
    let registered_discord_ids: HashSet<i64> = sqlx::query!("SELECT discord_id FROM discord_users")
        .fetch_all(&*ctx.data().pool)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|row| row.discord_id)
        .collect();

    let filtered_members = members
        .iter()
        .filter(|member| registered_discord_ids.contains(&(member.user.id.get() as i64)))
        .filter_map(|member| {
            let name = format!("{} ({})", member.display_name(), member.user.name);
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
        .collect::<Vec<_>>();

    filtered_members.into_iter()
}
