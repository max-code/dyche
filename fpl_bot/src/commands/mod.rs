pub mod captains;
pub mod chips;
pub mod deadline;
pub mod differentials;
pub mod hits;
pub mod loglevel;
pub mod register;
pub mod table;
pub mod team;
pub mod transfers;
pub mod unique;
pub mod whohas;

pub use captains::*;
pub use chips::*;
pub use deadline::*;
pub use differentials::*;
pub use hits::*;
pub use loglevel::*;
pub use register::*;
pub use table::*;
pub use team::*;
pub use transfers::*;
pub use unique::*;
pub use whohas::*;

use crate::Context;

pub fn get_image_file_path(command: &str, ctx: &Context<'_>) -> String {
    let user_id = ctx.author().id.get() as i64;
    let server_id = match ctx.guild() {
        Some(guild) => guild.id.get() as i64,
        None => 0,
    };

    fpl_common::paths::get_generated_image_path(command, user_id, server_id)
}

/// Macro to handle database query errors, creating a new embed if needed
#[macro_export]
macro_rules! handle_async_fallible {
    // Version with existing embed
    ($ctx:expr, $embed:expr, $query:expr, $error_message:expr) => {
        match $query.await {
            Ok(result) => result,
            Err(e) => {
                $embed
                    .error()
                    .body(format!("Error when calling {}", COMMAND))
                    .send()
                    .await?;
                return Err(format!("{}: {}", $error_message, e).into());
            }
        }
    };

    // Version without existing embed (creates a new one)
    ($ctx:expr, $query:expr, $error_message:expr) => {
        match $query.await {
            Ok(result) => result,
            Err(e) => {
                Embed::from_ctx($ctx)?
                    .error()
                    .body(format!("Error when calling {}", COMMAND))
                    .send()
                    .await?;
                return Err(format!("{}: {}", $error_message, e).into());
            }
        }
    };
}

/// Macro to handle string parsing with error reporting, creating a new embed if needed
#[macro_export]
macro_rules! handle_parse_value {
    // Version with existing embed
    ($ctx:expr, $embed:expr, $value:expr, $type:ty, $error_message:expr) => {
        match $value.parse::<$type>() {
            Ok(v) => v,
            Err(e) => {
                $embed.error().body($error_message).send().await?;
                return Err(e.into());
            }
        }
    };

    // Version without existing embed (creates a new one)
    ($ctx:expr, $value:expr, $type:ty, $error_message:expr) => {
        match $value.parse::<$type>() {
            Ok(v) => v,
            Err(e) => {
                Embed::from_ctx($ctx)?
                    .error()
                    .body($error_message)
                    .send()
                    .await?;
                return Err(e.into());
            }
        }
    };
}

#[macro_export]
macro_rules! render {
    // Version with embed that shows errors
    ($ctx:expr, $embed:expr, $renderer:expr, $data:expr, $file_name:expr, $error_message:expr) => {
        match $renderer.render($data, $file_name).await {
            Ok(result) => result,
            Err(e) => {
                $embed
                    .error()
                    .body(format!("{}: {}", $error_message, e))
                    .send()
                    .await?;
                return Err(e.into());
            }
        }
    };

    // Version without existing embed
    ($ctx:expr, $renderer:expr, $data:expr, $file_name:expr, $error_message:expr) => {
        match $renderer.render($data, $file_name).await {
            Ok(result) => result,
            Err(e) => {
                Embed::from_ctx($ctx)?
                    .error()
                    .body(format!("{}: {}", $error_message, e))
                    .send()
                    .await?;
                return Err(e.into());
            }
        }
    };
}
