use crate::Context;
use poise::serenity_prelude as serenity;

use crate::autocompletes::helpers::get_mini_league_name_autocompletes;

pub async fn autocomplete_mini_league<'a>(
    ctx: Context<'_>,
    partial: &'a str,
) -> impl Iterator<Item = serenity::AutocompleteChoice> + 'a {
    get_mini_league_name_autocompletes(ctx, partial, false).await
}
