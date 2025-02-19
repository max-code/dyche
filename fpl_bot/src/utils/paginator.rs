use poise::serenity_prelude as serenity;

use crate::utils::embed_builder::EmbedBuilder;
use crate::Error;
use crate::{constants::text_response, Context};
use poise::CreateReply;
use serenity::CreateEmbedFooter;

/*

Stole from https://docs.rs/poise/latest/src/poise/builtins/paginate.rs.html#35-94
but with my own embeds

*/

pub async fn paginate(
    ctx: Context<'_>,
    command: &str,
    pages: &[&str],
) -> Result<(), serenity::Error> {
    // Define some unique identifiers for the navigation buttons
    let ctx_id = ctx.id();
    let prev_button_id = format!("{}prev", ctx_id);
    let next_button_id = format!("{}next", ctx_id);

    // Send the embed with the first page as content
    let reply = {
        let components = serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new(&prev_button_id)
                .emoji('◀')
                .style(serenity::ButtonStyle::Secondary),
            serenity::CreateButton::new(&next_button_id)
                .emoji('▶')
                .style(serenity::ButtonStyle::Secondary),
        ]);

        let embed = EmbedBuilder::new(command, pages[0])
            .success(pages[0])
            .build()
            .footer(CreateEmbedFooter::new(format!(
                "📖 Page {}/{}",
                1,
                pages.len()
            )));

        CreateReply::default()
            .embed(embed)
            .components(vec![components])
    };

    ctx.send(reply).await?;

    // Loop through incoming interactions with the navigation buttons
    let mut current_page = 0;
    while let Some(press) = serenity::collector::ComponentInteractionCollector::new(ctx)
        // We defined our button IDs to start with `ctx_id`. If they don't, some other command's
        // button was pressed
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
        // Timeout when no navigation button has been pressed for 24 hours
        .timeout(std::time::Duration::from_secs(3600 * 24))
        .await
    {
        // Depending on which button was pressed, go to next or previous page
        if press.data.custom_id == next_button_id {
            current_page += 1;
            if current_page >= pages.len() {
                current_page = 0;
            }
        } else if press.data.custom_id == prev_button_id {
            current_page = current_page.checked_sub(1).unwrap_or(pages.len() - 1);
        } else {
            // This is an unrelated button interaction
            continue;
        }

        let embed = EmbedBuilder::new(command, pages[current_page])
            .success(pages[current_page])
            .build()
            .footer(CreateEmbedFooter::new(format!(
                "📖 Page {}/{}",
                current_page + 1,
                pages.len()
            )));

        press
            .create_response(
                ctx.serenity_context(),
                serenity::CreateInteractionResponse::UpdateMessage(
                    serenity::CreateInteractionResponseMessage::new().embed(embed),
                ),
            )
            .await?;
    }

    Ok(())
}

/*
This will send a new message if a previous message has already been sent.
Not designed to work with processing messages.
*/
pub async fn maybe_paginate_rows(
    ctx: Context<'_>,
    rows: Vec<String>,
    command: &str,
) -> Result<(), Error> {
    match rows.len() {
        1..=text_response::MAX_ROWS_PER_PAGE => {
            let embed = EmbedBuilder::new(command, "")
                .success(rows.join("\n").as_str())
                .build();

            ctx.send(CreateReply::default().embed(embed)).await?;
        }
        _ => {
            let chunks = rows
                .chunks(text_response::MAX_ROWS_PER_PAGE)
                .map(|chunk| chunk.join("\n"))
                .collect::<Vec<String>>();

            paginate(
                ctx,
                command,
                &chunks.iter().map(|page| page.as_str()).collect::<Vec<_>>(),
            )
            .await?;
        }
    }

    Ok(())
}
