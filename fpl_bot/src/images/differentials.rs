use resvg::{render, usvg};
use std::collections::HashMap;
use svg::node::element::Rectangle;
use svg::Document;
use tiny_skia::Pixmap;
use usvg::{Options, Tree};

use super::colours::{OFF_WHITE_COLOUR, WHITE_COLOUR};
use super::{calculate_player_card_xs, PlayerGameInfo, PlayerInfo};
use crate::images::constants::colours::PURPLE_COLOUR;

#[derive(Debug, Clone)]
pub struct DifferentialKey {
    pub team_name: String,
    pub user_first_name: String,
    pub user_last_name: String,
}

impl DifferentialKey {
    fn to_key_string(&self) -> String {
        format!(
            "{} {} ({})",
            self.user_first_name, self.user_last_name, self.team_name
        )
    }
}

#[derive(Debug, Clone)]
pub struct Differentials {
    pub user_to_differentials: HashMap<String, Vec<PlayerInfo>>,
}

impl Default for Differentials {
    fn default() -> Self {
        Self::new()
    }
}

impl Differentials {
    pub fn new() -> Self {
        Self {
            user_to_differentials: HashMap::new(),
        }
    }

    pub fn add_differential(
        mut self,
        key: DifferentialKey,
        player_name: String,
        code: i32,
        multiplier: i16,
        is_captain: bool,
        is_vice_captain: bool,
    ) -> Self {
        let text = if multiplier == 0 {
            "Benched".to_string()
        } else {
            "Starter".to_string()
        };
        let games = vec![PlayerGameInfo::FreeText(text)];

        self.user_to_differentials
            .entry(key.to_key_string())
            .or_default()
            .push(PlayerInfo::new(
                player_name,
                code as u32,
                games,
                is_captain,
                is_vice_captain,
            ));

        self
    }
}
