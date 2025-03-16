use std::env;

pub fn get_base_path() -> String {
    env::var("FPL_BASE_PATH").unwrap_or_else(|_| "/home/dyche".to_string())
}

pub fn get_player_image_path(code: impl ToString) -> String {
    format!(
        "{}/fpl_assets/player_images/{}.png",
        get_base_path(),
        code.to_string()
    )
}

pub fn get_generated_image_path(
    command: &str,
    user_id: impl ToString,
    server_id: impl ToString,
) -> String {
    format!(
        "{}/fpl_bot/generated/{}_{}_{}.png",
        get_base_path(),
        command,
        user_id.to_string(),
        server_id.to_string()
    )
}

pub fn get_static_image_path(filename: impl ToString) -> String {
    format!(
        "{}/fpl_bot/static/{}.png",
        get_base_path(),
        filename.to_string()
    )
}
