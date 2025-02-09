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
