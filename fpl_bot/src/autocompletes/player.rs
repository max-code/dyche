use crate::Context;
use poise::serenity_prelude as serenity;

use super::get_player_name_autocompletes;

pub async fn autocomplete_player<'a>(
    ctx: Context<'_>,
    partial: &'a str,
) -> impl Iterator<Item = serenity::AutocompleteChoice> + 'a {
    get_player_name_autocompletes(ctx, partial, false).await
}
