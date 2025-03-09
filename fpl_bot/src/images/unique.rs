use resvg::{render, usvg};
use svg::node::element::Rectangle;
use svg::Document;
use tiny_skia::Pixmap;
use usvg::{Options, Tree};

use super::colours::{GREY_COLOUR, OFF_WHITE_COLOUR, WHITE_COLOUR};
use super::{calculate_player_card_xs, PlayerGameInfo, PlayerInfo};
use crate::images::constants::colours::PURPLE_COLOUR;

#[derive(Debug, Clone)]
pub struct UniquePlayers {
    pub players: Vec<PlayerInfo>,
}

impl Default for UniquePlayers {
    fn default() -> Self {
        Self::new()
    }
}

impl UniquePlayers {
    pub fn new() -> Self {
        Self { players: vec![] }
    }

    pub fn add_player(
        mut self,
        name: String,
        code: u32,
        multiplier: i16,
        is_captain: bool,
        is_vice_captain: bool,
        opponents: String,
    ) -> Self {
        let games = vec![PlayerGameInfo::FreeText(opponents)];

        // Grey them out if they arent playing. Will be active_bg_colour as we are using free text
        let bg_colour = match multiplier {
            0 => GREY_COLOUR,
            _ => PURPLE_COLOUR,
        };

        self.players.push(
            PlayerInfo::new(name, code, games, is_captain, is_vice_captain)
                .status_active_bg_color(bg_colour),
        );

        self
    }
}

#[derive(Debug, Clone)]
pub struct UniqueRenderer {
    pub width: u32,
    pub player_row_height: u32,
    pub player_card_width: u32,
    pub players_per_row: usize,
    pub internal_vertical_padding: u32,
}

impl Default for UniqueRenderer {
    fn default() -> Self {
        Self {
            width: 1000,
            player_row_height: 300,
            player_card_width: 200,
            players_per_row: 3,
            internal_vertical_padding: 20,
        }
    }
}

impl UniqueRenderer {
    pub async fn render(&self, mut data: UniquePlayers, path: &str) -> std::io::Result<()> {
        let num_rows = (data.players.len() as f32 / self.players_per_row as f32).ceil() as u32;
        let total_height = num_rows * self.player_row_height;
        let player_card_height = self.player_row_height - (2 * self.internal_vertical_padding);
        let mut document = Document::new()
            .set("viewBox", (0, 0, self.width, total_height))
            .set("width", self.width)
            .set("height", total_height);

        // Add rows
        for (index, players_chunk) in data.players.chunks_mut(self.players_per_row).enumerate() {
            let y_pos = index as u32 * self.player_row_height;
            // Row background
            let bg_color = if index % 2 == 0 {
                WHITE_COLOUR
            } else {
                OFF_WHITE_COLOUR
            };
            let row_bg = Rectangle::new()
                .set("x", 0)
                .set("y", y_pos)
                .set("width", self.width)
                .set("height", self.player_row_height)
                .set("fill", bg_color);

            let bottom_border = svg::node::element::Line::new()
                .set("x1", 0)
                .set("y1", y_pos - 1)
                .set("x2", self.width)
                .set("y2", y_pos - 1)
                .set("stroke", PURPLE_COLOUR)
                .set("stroke-width", 2);

            document = document.add(row_bg).add(bottom_border);

            let player_card_xs: Vec<u32> = calculate_player_card_xs(
                self.player_card_width,
                self.width,
                players_chunk.len() as u32,
                0,
            );
            let player_y_pos = y_pos + self.internal_vertical_padding;

            for (x_offset, player) in player_card_xs.iter().zip(players_chunk.iter_mut()) {
                let player_card = player.border_color(PURPLE_COLOUR).to_card_svg(
                    *x_offset,
                    player_y_pos,
                    self.player_card_width,
                    player_card_height,
                )?;
                document = document.add(player_card);
            }
        }

        // Convert SVG to PNG
        let svg_string = document.to_string();
        let mut opt: Options<'_> = Options::default();
        opt.fontdb_mut().load_system_fonts();

        let tree = Tree::from_str(&svg_string, &opt).unwrap();
        let size = tree.size();
        let mut pixmap = Pixmap::new(size.width() as u32, size.height() as u32).unwrap();
        render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

        std::fs::write(path, pixmap.encode_png().unwrap())
    }
}
