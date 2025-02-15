use serenity::all::CreateEmbed;
use sqlx::PgPool;

use super::embed_builder::EmbedBuilder;

pub async fn check_discord_user_registered(
    pool: &PgPool,
    discord_id: i64,
) -> Result<bool, sqlx::Error> {
    let registered = sqlx::query!(
        "SELECT COUNT(*) as count FROM discord_users WHERE discord_id=$1;",
        discord_id
    )
    .fetch_one(pool)
    .await?;

    Ok(registered.count.unwrap_or(0) > 0)
}

pub fn get_not_registered_message(command: &str, discord_id: i64) -> CreateEmbed {
    EmbedBuilder::new(command, "").error(format!("User <@{}> not registered with FplBot!\nThey should use **/register [Team ID]** for this command to work\n(To find Team ID: https://fpl.team/find-id/)", discord_id).as_str()).build()
}
