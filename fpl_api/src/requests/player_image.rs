use std::fs;
use std::path::PathBuf;

use serde::de::Error;

use super::{FplRequest, FplResponseType};

#[derive(Debug)]
pub struct PlayerPhotoRequest {
    pub player_code: u32,
    pub output_path: PathBuf,
}

impl PlayerPhotoRequest {
    pub fn new(player_code: u32, output_path: impl Into<PathBuf>) -> Self {
        Self {
            player_code,
            output_path: output_path.into(),
        }
    }
}

impl FplRequest for PlayerPhotoRequest {
    type Response = (); // No data to return, just saving the file

    fn to_url(&self, _base_url: &str) -> String {
        format!(
            "https://resources.premierleague.com/premierleague/photos/players/250x250/p{}.png",
            self.player_code
        )
    }

    fn is_binary(&self) -> bool {
        true
    }

    fn process_response(
        &self,
        response: FplResponseType,
    ) -> Result<Self::Response, Box<dyn std::error::Error>> {
        match response {
            FplResponseType::Binary(bytes) => {
                // Create parent directories if they don't exist
                if let Some(parent) = self.output_path.parent() {
                    fs::create_dir_all(parent)?;
                }

                // Save the bytes to the specified path
                fs::write(&self.output_path, bytes)?;
                Ok(())
            }
            FplResponseType::Json(_) => Err(Box::new(serde_json::Error::custom(
                "Expected binary response, got JSON",
            ))),
        }
    }
}
