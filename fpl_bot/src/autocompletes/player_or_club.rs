use poise::serenity_prelude as serenity;

use crate::Context;

use super::{get_club_name_autocompletes, get_player_name_autocompletes};

pub async fn autocomplete_player_or_club<'a>(
    _ctx: Context<'_>,
    _partial: &'a str,
) -> impl Iterator<Item = String> + 'a {
    ["Club".to_string(), "Player".to_string()].into_iter()
}

pub async fn autocomplete_player_or_club_value<'a>(
    ctx: Context<'_>,
    partial: &'a str,
) -> impl Iterator<Item = serenity::AutocompleteChoice> + 'a {
    let interaction = match ctx {
        Context::Application(ctx) => &ctx.interaction.clone(),
        _ => {
            return vec![].into_iter();
        }
    };

    let club_or_player = interaction
        .data
        .options
        .iter()
        .find(|opt| opt.name == "player_or_club")
        .and_then(|opt| match &opt.value {
            serenity::CommandDataOptionValue::String(s) => Some(s.as_str()),
            _ => None,
        })
        .unwrap_or("");

    let choices = match club_or_player {
        "Club" => get_club_name_autocompletes(ctx, partial, true)
            .await
            .collect::<Vec<_>>(),
        "Player" => get_player_name_autocompletes(ctx, partial, true)
            .await
            .collect::<Vec<_>>(),
        _ => vec![],
    };

    choices.into_iter()
}
