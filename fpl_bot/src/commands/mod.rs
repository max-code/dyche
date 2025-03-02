pub mod captains;
pub mod chips;
pub mod deadline;
pub mod hits;
pub mod loglevel;
pub mod register;
pub mod table;
pub mod team;
pub mod unique;
pub mod whohas;

pub use captains::*;
pub use chips::*;
pub use deadline::*;
pub use hits::*;
pub use loglevel::*;
pub use register::*;
pub use table::*;
pub use team::*;
pub use unique::*;
pub use whohas::*;

use crate::Context;

pub fn get_image_file_path(command: &str, ctx: &Context<'_>) -> String {
    let user_id = ctx.author().id.get() as i64;
    let server_id = match ctx.guild() {
        Some(guild) => guild.id.get() as i64,
        None => 0,
    };

    format!(
        "/Users/maxjordan/code/dyche/fpl_bot/generated/{}_{}_{}.png",
        command, user_id, server_id
    )
}
