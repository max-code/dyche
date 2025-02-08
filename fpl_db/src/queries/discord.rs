use sqlx::PgPool;
use tracing::debug;

use crate::models::DiscordUser;

pub async fn insert_discord_user(pool: &PgPool, user: &DiscordUser) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    sqlx::query!(
        r#"
        INSERT INTO discord_users (
        discord_id, team_id
        )
        VALUES ($1, $2)
        "#,
        user.discord_id,
        i32::from(user.team_id)
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    debug!("Insert Completed");
    Ok(())
}

pub async fn get_discord_user(
    pool: &PgPool,
    user_id: i64,
) -> Result<Option<DiscordUser>, sqlx::Error> {
    sqlx::query_as!(
        DiscordUser,
        "SELECT discord_id, team_id FROM discord_users WHERE discord_id = $1",
        user_id
    )
    .fetch_optional(pool)
    .await
}
