use poise::serenity_prelude as serenity;
use tracing::error;

use crate::autocompletes::get_mini_league_name_autocompletes;
use crate::Context;

use super::helpers::get_registered_users_autocompletes;

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

    let choices = match league_or_user {
        "League" => get_mini_league_name_autocompletes(ctx, partial, true)
            .await
            .collect::<Vec<_>>(),
        "User" => {
            match guild_id
                .members(&ctx.serenity_context().http, None, None)
                .await
            {
                Ok(members) => get_registered_users_autocompletes(ctx, &members, partial)
                    .await
                    .collect::<Vec<_>>(),
                Err(err) => {
                    error!("{:#?}", err);
                    vec![]
                }
            }
        }
        _ => vec![],
    };

    choices.into_iter()
}
