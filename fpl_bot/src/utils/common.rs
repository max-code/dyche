use sqlx::PgPool;

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

pub fn get_not_registered_title_and_message(discord_id: i64) -> (String, String) {
    ("User not registered!".to_string(),
    format!("User <@{}> not registered with FplBot!\nThey should use **/register [Team ID]** for this command to work\n(To find Team ID: https://fpl.team/find-id/)", discord_id))
}
