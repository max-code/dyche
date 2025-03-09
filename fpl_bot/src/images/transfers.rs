use resvg::{render, usvg};
use std::collections::HashMap;
use svg::Document;
use tiny_skia::Pixmap;
use usvg::{Options, Tree};

use super::colours::{GREEN_COLOUR, WHITE_COLOUR};
use super::{calculate_player_card_xs, CenteredTextBox, CornerRounding, FontWeight, PlayerInfo};
use crate::images::constants::colours::PURPLE_COLOUR;

#[derive(Debug, Clone)]
pub struct TransfersKey {
    pub team_name: String,
    pub user_first_name: String,
    pub user_last_name: String,
}

impl TransfersKey {
    fn to_key_string(&self) -> String {
        format!(
            "{} {} ({})",
            self.user_first_name, self.user_last_name, self.team_name
        )
    }
}

#[derive(Debug, Clone)]
pub struct Transfers {
    pub user_to_transfers: HashMap<String, Vec<(PlayerInfo, PlayerInfo)>>,
}

impl Default for Transfers {
    fn default() -> Self {
        Self::new()
    }
}

impl Transfers {
    pub fn new() -> Self {
        Self {
            user_to_transfers: HashMap::new(),
        }
    }

    pub fn add_transfer(
        mut self,
        key: TransfersKey,
        in_player: PlayerInfo,
        out_player: PlayerInfo,
    ) -> Self {
        self.user_to_transfers
            .entry(key.to_key_string())
            .or_default()
            .push((in_player, out_player));

        self
    }
}

#[derive(Debug, Clone)]
pub struct TransfersRenderer {
    pub width: u32,
    pub transfer_boxes_per_row: u32,
    pub transfer_box_title_height: u32,
    pub player_row_height: u32,
    pub player_card_width: u32,
    pub internal_vertical_padding: u32,
}

impl Default for TransfersRenderer {
    fn default() -> Self {
        Self {
            width: 1000,
            transfer_boxes_per_row: 2,
            transfer_box_title_height: 80,
            player_row_height: 150,
            player_card_width: 100,
            internal_vertical_padding: 20,
        }
    }
}

impl TransfersRenderer {
    pub async fn render(&self, data: Transfers, path: &str) -> std::io::Result<()> {
        let mut transfers_vec: Vec<(String, Vec<(PlayerInfo, PlayerInfo)>)> =
            data.user_to_transfers.into_iter().collect();

        // sort by vec length so team boxes are similar heights
        transfers_vec.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

        let chunked_transfers: Vec<_> = transfers_vec
            .chunks(self.transfer_boxes_per_row as usize)
            .collect();

        // Work out the total height. First element in each chunk always has the most transfers so most rows
        let mut total_height = 0;
        for chunk in chunked_transfers.iter() {
            if let Some(transfer_box) = chunk.first() {
                total_height += self.calculate_transfer_box_height(transfer_box);
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

        let player_card_height = self.player_row_height - (2 * self.internal_vertical_padding);

        let mut y_offset = 0;
        // Transfer box rows
        for row in chunked_transfers.iter() {
            let chunk_size = row.len();
            let transfer_box_width = self.width / chunk_size as u32;
            let transfer_text_width = (transfer_box_width - (2 * self.player_card_width)) / 4;

            let mut max_transfer_box_y_offset = 0;
            // Individual team boxes per row
            for (col_index, transfer_box) in row.iter().enumerate() {
                let (team_name, transfers) = transfer_box;

                // Calculate the row height to adjust y_offset for the next row
                let row_height = self.calculate_transfer_box_height(transfer_box);
                max_transfer_box_y_offset = std::cmp::max(max_transfer_box_y_offset, row_height);

                let x_offset = col_index * transfer_box_width as usize;

                // Add title box
                let (team_box_title_bg, team_box_title_text) = CenteredTextBox::new()
                    .text(team_name)
                    .dimensions(
                        transfer_box_width as f64,
                        self.transfer_box_title_height as f64,
                    )
                    .position(x_offset as f64, y_offset as f64)
                    .background_color(PURPLE_COLOUR)
                    .font_color(WHITE_COLOUR)
                    .font_weight(FontWeight::Black)
                    .corner_rounding(CornerRounding::None)
                    .inner_padding(0.95)
                    .build()?;

                document = document.add(team_box_title_bg).add(team_box_title_text);

                for (idx, (player_out, player_in)) in transfers.iter().enumerate() {
                    let row_y = y_offset
                        + self.transfer_box_title_height
                        + (idx as u32 * self.player_row_height);
                    let player_card_y_pos = row_y + self.internal_vertical_padding;

                    let players = [player_out, player_in];

                    let player_card_xs: Vec<u32> = calculate_player_card_xs(
                        self.player_card_width,
                        transfer_box_width,
                        players.len() as u32,
                        x_offset as u32,
                    );

                    for (x_offset, player) in player_card_xs.iter().zip(players.iter()) {
                        let player_card =
                            (**player).clone().border_color(PURPLE_COLOUR).to_card_svg(
                                *x_offset,
                                player_card_y_pos,
                                self.player_card_width,
                                player_card_height,
                            )?;

                        document = document.add(player_card);
                    }

                    let text_x = x_offset as u32 + (transfer_box_width - transfer_text_width) / 2;
                    let (transfer_text_bg, transfer_text_box) = CenteredTextBox::new()
                        .text("to")
                        .dimensions(transfer_text_width as f64, player_card_height as f64)
                        .position(
                            text_x as f64,
                            (row_y + self.internal_vertical_padding) as f64,
                        )
                        .font_color(PURPLE_COLOUR)
                        .background_color(WHITE_COLOUR)
                        .font_weight(FontWeight::Bold)
                        .inner_padding(0.6)
                        .build()?;

                    document = document.add(transfer_text_bg).add(transfer_text_box);
                }
            }

            if chunk_size > 1 {
                for i in 1..chunk_size {
                    let divider_x = (self.width * i as u32) / chunk_size as u32;
                    let vertical_divider = svg::node::element::Line::new()
                        .set("x1", divider_x)
                        .set("y1", y_offset)
                        .set("x2", divider_x)
                        .set("y2", y_offset + self.transfer_box_title_height)
                        .set("stroke", GREEN_COLOUR)
                        .set("stroke-width", 2);

                    document = document.add(vertical_divider);
                }
            }

            y_offset += max_transfer_box_y_offset;
        }

        // Convert SVG to PNG
        let svg_string = document.to_string();
        let mut opt: Options<'_> = Options::default();
        opt.fontdb_mut().load_system_fonts();

        let tree: Tree = Tree::from_str(&svg_string, &opt).unwrap();
        let size = tree.size();
        let mut pixmap = Pixmap::new(size.width() as u32, size.height() as u32).unwrap();
        render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

        std::fs::write(path, pixmap.encode_png().unwrap())
    }

    fn calculate_transfer_box_height(
        &self,
        transfer_box: &(String, Vec<(PlayerInfo, PlayerInfo)>),
    ) -> u32 {
        self.transfer_box_title_height + (transfer_box.1.len() as u32 * self.player_row_height)
    }
}
