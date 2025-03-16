use resvg::{render, usvg};
use std::collections::HashMap;
use svg::Document;
use tiny_skia::Pixmap;
use usvg::{Options, Tree};

use super::colours::{GREEN_COLOUR, WHITE_COLOUR};
use super::{
    calculate_player_card_xs, CenteredTextBox, CornerRounding, FontWeight, PlayerGameInfo,
    PlayerInfo,
};
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
        is_captain: bool,
        is_vice_captain: bool,
        opponents: String,
    ) -> Self {
        let games = vec![PlayerGameInfo::FreeText(opponents)];

        self.user_to_differentials
            .entry(key.to_key_string())
            .or_default()
            .push(PlayerInfo::new(
                player_name,
                code as u32,
                games,
                is_captain,
                is_vice_captain,
                true,
            ));

        self
    }
}

#[derive(Debug, Clone)]
pub struct DifferentialsRenderer {
    pub width: u32,
    pub team_boxes_per_row: u32,
    pub columns_per_team_box: u32,
    pub team_box_title_height: u32,
    pub player_row_height: u32,
    pub player_card_width: u32,
    pub internal_vertical_padding: u32,
}

impl Default for DifferentialsRenderer {
    fn default() -> Self {
        Self {
            width: 1000,
            team_boxes_per_row: 2,
            columns_per_team_box: 2,
            team_box_title_height: 100,
            player_row_height: 250,
            player_card_width: 175,
            internal_vertical_padding: 20,
        }
    }
}

impl DifferentialsRenderer {
    pub async fn render(&self, data: Differentials, path: &str) -> std::io::Result<()> {
        let mut teams_vec: Vec<(String, Vec<PlayerInfo>)> =
            data.user_to_differentials.into_iter().collect();

        // sort by vec length so team boxes are similar heights
        teams_vec.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

        let chunked_teams: Vec<_> = teams_vec.chunks(self.team_boxes_per_row as usize).collect();

        // Work out the total height. First element in each chunk always has the most players so most rows
        let mut total_height = 0;
        for chunk in chunked_teams.iter() {
            if let Some(team_box) = chunk.first() {
                total_height += self.calculate_team_box_height(team_box);
            }
        }

        let mut document = Document::new()
            .set("viewBox", (0, 0, self.width, total_height))
            .set("width", self.width)
            .set("height", total_height);

        let background = svg::node::element::Rectangle::new()
            .set("width", self.width)
            .set("height", total_height)
            .set("fill", WHITE_COLOUR);

        document = document.add(background);

        let team_box_width = self.width / 2;
        let player_card_height = self.player_row_height - (2 * self.internal_vertical_padding);

        // Add differentials
        let mut y_offset = 0;
        // Team box rows
        for row in chunked_teams.iter() {
            let mut max_team_box_y_offset = 0;
            // Individual team boxes per row
            for (col_index, team_box) in row.iter().enumerate() {
                let (team_name, players) = team_box;

                // Calculate the row height to adjust y_offset for the next row
                let row_height = self.calculate_team_box_height(team_box);
                max_team_box_y_offset = std::cmp::max(max_team_box_y_offset, row_height);

                let x_offset = col_index * team_box_width as usize;

                // Add title box
                let (team_box_title_bg, team_box_title_text) = CenteredTextBox::new()
                    .text(team_name)
                    .dimensions(team_box_width as f64, self.team_box_title_height as f64)
                    .position(x_offset as f64, y_offset as f64)
                    .background_color(PURPLE_COLOUR)
                    .font_color(WHITE_COLOUR)
                    .font_weight(FontWeight::Black)
                    .corner_rounding(CornerRounding::None)
                    .inner_padding(0.95)
                    .build()?;

                document = document.add(team_box_title_bg).add(team_box_title_text);

                // Chunk into rows of player cards
                let differential_player_chunks: Vec<_> =
                    players.chunks(self.columns_per_team_box as usize).collect();

                // Each row of player cards
                for (player_card_row_idx, players_chunk) in
                    differential_player_chunks.iter().enumerate()
                {
                    let player_card_y_offset = y_offset
                        + self.team_box_title_height
                        + (player_card_row_idx as u32 * self.player_row_height);

                    let player_card_xs: Vec<u32> = calculate_player_card_xs(
                        self.player_card_width,
                        team_box_width,
                        players_chunk.len() as u32,
                        x_offset as u32,
                    );

                    let player_y_pos = player_card_y_offset + self.internal_vertical_padding;

                    for (x_offset, player) in player_card_xs.iter().zip(players_chunk.iter()) {
                        let player_card = player.clone().border_color(PURPLE_COLOUR).to_card_svg(
                            *x_offset,
                            player_y_pos,
                            self.player_card_width,
                            player_card_height,
                        )?;
                        document = document.add(player_card);
                    }
                }
            }

            let vertical_divider = svg::node::element::Line::new()
                .set("x1", self.width / 2)
                .set("y1", y_offset)
                .set("x2", self.width / 2)
                .set("y2", y_offset + self.team_box_title_height)
                .set("stroke", GREEN_COLOUR)
                .set("stroke-width", 2);

            document = document.add(vertical_divider);

            y_offset += max_team_box_y_offset as u32;
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

    fn calculate_team_box_height(&self, team_box: &(String, Vec<PlayerInfo>)) -> usize {
        let (_, players) = team_box;
        let player_rows = (players.len() + self.columns_per_team_box as usize - 1)
            / self.columns_per_team_box as usize;

        self.team_box_title_height as usize + (player_rows * self.player_row_height as usize)
    }
}
