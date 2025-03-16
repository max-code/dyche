use fpl_common::types::{Chip, GameWeekId};
use resvg::{render, usvg};
use sqlx::prelude::FromRow;
use svg::node::element::Rectangle;
use svg::Document;
use thousands::Separable;
use tiny_skia::Pixmap;
use usvg::{Options, Tree};

use crate::images::util::PlayerInfo;

use super::{
    calculate_player_card_xs,
    colours::{
        DARK_PITCH_GREEN_COLOUR, GREEN_COLOUR, PITCH_GREEN_COLOUR, PURPLE_COLOUR, WHITE_COLOUR,
    },
    CenteredTextBox, CornerRounding, FontWeight,
};

#[derive(Debug, Clone, FromRow)]
pub struct TransferInfo {
    pub player_in_name: String,
    pub player_in_code: i32,
    pub player_in_cost: f64,
    pub player_out_name: String,
    pub player_out_code: i32,
    pub player_out_cost: f64,
}

impl TransferInfo {
    pub fn new(
        player_in_name: String,
        player_in_code: i32,
        player_in_cost: f64,
        player_out_name: String,
        player_out_code: i32,
        player_out_cost: f64,
    ) -> Self {
        Self {
            player_in_name,
            player_in_code,
            player_in_cost,
            player_out_name,
            player_out_code,
            player_out_cost,
        }
    }
}

#[derive(Debug, Default)]
pub struct TeamDataBuilder {
    team_name: Option<String>,
    gw_rank: Option<i64>,
    overall_rank: Option<i64>,
    points: Option<i64>,
    game_week: Option<GameWeekId>,
    goalkeeper: Option<PlayerInfo>,
    defenders: Vec<PlayerInfo>,
    midfielders: Vec<PlayerInfo>,
    forwards: Vec<PlayerInfo>,
    bench: Vec<PlayerInfo>,
    assman: Option<PlayerInfo>,
    transfers: Vec<TransferInfo>,
    chip: Option<Chip>,
}

impl TeamDataBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn team_name(mut self, name: impl Into<String>) -> Self {
        self.team_name = Some(name.into());
        self
    }

    pub fn gw_rank(mut self, rank: i64) -> Self {
        self.gw_rank = Some(rank);
        self
    }

    pub fn overall_rank(mut self, rank: i64) -> Self {
        self.overall_rank = Some(rank);
        self
    }

    pub fn points(mut self, points: i64) -> Self {
        self.points = Some(points);
        self
    }

    pub fn game_week(mut self, gw: GameWeekId) -> Self {
        self.game_week = Some(gw);
        self
    }

    pub fn goalkeeper(mut self, keeper: PlayerInfo) -> Self {
        self.goalkeeper = Some(keeper);
        self
    }

    pub fn add_defender(mut self, defender: PlayerInfo) -> Self {
        self.defenders.push(defender);
        self
    }

    pub fn add_midfielder(mut self, midfielder: PlayerInfo) -> Self {
        self.midfielders.push(midfielder);
        self
    }

    pub fn add_forward(mut self, forward: PlayerInfo) -> Self {
        self.forwards.push(forward);
        self
    }

    pub fn add_bench_player(mut self, player: PlayerInfo) -> Self {
        self.bench.push(player);
        self
    }

    pub fn add_manager(mut self, player: PlayerInfo) -> Self {
        self.assman = Some(player);
        self
    }

    pub fn add_transfer(mut self, transfer: TransferInfo) -> Self {
        self.transfers.push(transfer);
        self
    }

    pub fn add_chip(mut self, chip: Chip) -> Self {
        self.chip = Some(chip);
        self
    }

    pub fn build(self) -> Result<TeamData, &'static str> {
        let team_name = self.team_name.ok_or("Team Name Required")?;
        let gw_rank = self.gw_rank.ok_or("GW Rank Required")?;
        let overall_rank = self.overall_rank.ok_or("Overall Rank Required")?;
        let points = self.points.ok_or("Points Required")?;
        let game_week = self.game_week.ok_or("Game Week Required")?;
        let goalkeeper = self.goalkeeper.ok_or("Goalkeeper Required")?;

        Ok(TeamData {
            team_name,
            gw_rank,
            overall_rank,
            points,
            game_week,
            goalkeeper,
            defenders: self.defenders,
            midfielders: self.midfielders,
            forwards: self.forwards,
            bench: self.bench,
            assman: self.assman,
            transfers: self.transfers,
            chip: self.chip,
        })
    }
}

#[derive(Debug, Clone)]
pub struct TeamData {
    pub team_name: String,
    pub gw_rank: i64,
    pub overall_rank: i64,
    pub points: i64,
    pub game_week: GameWeekId,
    pub goalkeeper: PlayerInfo,
    pub defenders: Vec<PlayerInfo>,
    pub midfielders: Vec<PlayerInfo>,
    pub forwards: Vec<PlayerInfo>,
    pub assman: Option<PlayerInfo>,
    pub bench: Vec<PlayerInfo>,
    pub transfers: Vec<TransferInfo>,
    pub chip: Option<Chip>,
}

impl TeamData {
    pub fn builder() -> TeamDataBuilder {
        TeamDataBuilder::new()
    }

    pub fn get_player_rows(&self) -> Vec<Vec<PlayerInfo>> {
        vec![
            vec![self.goalkeeper.clone()],
            self.defenders.clone(),
            self.midfielders.clone(),
            self.forwards.clone(),
            self.assman
                .clone()
                .into_iter()
                .chain(self.bench.clone())
                .collect(),
        ]
    }
}

#[derive(Debug, Clone)]
pub struct TeamRenderer {
    pub width: u32,
    pub player_card_height: u32,
    pub player_card_width: u32,
    pub player_card_vertical_padding: u32,
    pub transfer_row_height: u32,
    pub header_height: u32,
    pub header_vertical_padding: u32,
    pub score_box_side_length: f64,
    pub score_box_radius: f64,
    pub chip_box_height: f64,
    pub side_box_height: f64,
    pub side_box_padding: f64,
    pub transfer_row_image_width: f64,
    pub transfer_row_horizontal_padding: f64,
    pub transfer_row_vertical_padding: f64,
}

impl Default for TeamRenderer {
    fn default() -> Self {
        Self {
            width: 1000,
            player_card_height: 200,
            player_card_width: 150,
            player_card_vertical_padding: 50,
            header_height: 200,
            header_vertical_padding: 25,
            score_box_side_length: 150.0,
            score_box_radius: 10.0,
            chip_box_height: 25.0,
            side_box_height: 50.0,
            side_box_padding: 25.0,
            transfer_row_height: 100,
            transfer_row_image_width: 100.0,
            transfer_row_horizontal_padding: 50.0,
            transfer_row_vertical_padding: 25.0,
        }
    }
}

impl TeamRenderer {
    pub async fn render(&self, data: TeamData, path: &str) -> std::io::Result<()> {
        let header_height = self.header_height + self.header_vertical_padding;
        let players_height =
            (4 * self.player_card_height) + (5 * self.player_card_vertical_padding);
        let mut bench_height = self.player_card_height + self.player_card_vertical_padding;
        // If we have no transfers, need an extra little bit of padding
        bench_height += if data.transfers.is_empty() {
            self.player_card_vertical_padding / 2
        } else {
            0
        };
        let transfer_rows = ((data.transfers.len() + 1) / 2) as u32;
        let transfers_height = if !data.transfers.is_empty() {
            ((transfer_rows + 1) * self.transfer_row_height)
                + (transfer_rows * self.transfer_row_vertical_padding as u32)
        } else {
            0
        };
        let total_height = self.header_height + players_height + bench_height + transfers_height;

        // TODO: Reset these to self.width and self.height
        let mut document: svg::node::element::SVG = Document::new()
            .set("viewBox", (0, 0, self.width, total_height))
            .set("width", self.width)
            .set("height", total_height);

        let header_background = Rectangle::new()
            .set("x", 0)
            .set("y", 0)
            .set("width", "100%")
            .set("height", header_height)
            .set("fill", WHITE_COLOUR);

        let players_background = Rectangle::new()
            .set("x", 0)
            .set("y", header_height)
            .set("width", "100%")
            .set("height", players_height)
            .set("fill", PITCH_GREEN_COLOUR);

        let bench_background = Rectangle::new()
            .set("x", 0)
            .set("y", header_height + players_height)
            .set("width", "100%")
            .set("height", bench_height)
            .set("fill", DARK_PITCH_GREEN_COLOUR);

        let transfers_background = Rectangle::new()
            .set("x", 0)
            .set("y", players_height + header_height + bench_height)
            .set("width", "100%")
            .set("height", transfers_height)
            .set("fill", WHITE_COLOUR);

        document = document
            .add(header_background)
            .add(players_background)
            .add(bench_background)
            .add(transfers_background);

        document = self.add_header(&data, document)?;
        document = self.add_player_cards(&data, document)?;
        if !data.transfers.is_empty() {
            document = self.add_transfers(&data, document)?;
        }
        let svg_string = document.to_string();
        let mut opt: Options<'_> = Options::default();
        opt.fontdb_mut().load_system_fonts();

        let tree = Tree::from_str(&svg_string, &opt).unwrap();
        let size = tree.size();
        let mut pixmap = Pixmap::new(size.width() as u32, size.height() as u32).unwrap();
        render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

        std::fs::write(path, pixmap.encode_png().unwrap())
    }

    fn add_player_cards(
        &self,
        data: &TeamData,
        mut document: Document,
    ) -> Result<Document, std::io::Error> {
        let mut y_offset: u32 = self.header_height + (2 * self.header_vertical_padding);
        for (idx, row) in data.get_player_rows().iter().enumerate() {
            let xs: Vec<u32> =
                calculate_player_card_xs(self.player_card_width, self.width, row.len() as u32, 0);
            if idx == 4 {
                y_offset += self.player_card_vertical_padding;
            }

            for (x_offset, player) in xs.into_iter().zip(row.iter()) {
                let opacity = match player.has_fixture {
                    true => 1.0,
                    false => 0.4,
                };
                // TODO: Rethink this clone
                let mut player_clone = player.clone();
                let player_card = player_clone.card_opactiy(opacity).to_card_svg(
                    x_offset,
                    y_offset,
                    self.player_card_width,
                    self.player_card_height,
                )?;
                document = document.add(player_card);
            }
            y_offset += self.player_card_height + self.player_card_vertical_padding;
        }

        Ok(document)
    }

    fn add_header(
        &self,
        data: &TeamData,
        mut document: Document,
    ) -> Result<Document, std::io::Error> {
        let rounding = match data.chip {
            Some(_) => CornerRounding::Top,
            None => CornerRounding::All,
        };

        // POINTS BOX
        let (points_bg, points_text) = CenteredTextBox::new()
            .text(data.points.to_string())
            .dimensions(self.score_box_side_length, self.score_box_side_length)
            .position(
                (self.width as f64 - self.score_box_side_length) / 2.0,
                (self.header_height as f64 - self.score_box_side_length) / 2.0,
            )
            .background_color(PURPLE_COLOUR)
            .font_color(GREEN_COLOUR)
            .font_weight(FontWeight::Bold)
            .corner_rounding(rounding)
            .radius(self.score_box_radius)
            .build()?;

        document = document.add(points_bg).add(points_text);

        // CHIP
        if let Some(chip) = data.chip {
            let (chip_bg, chip_text) = CenteredTextBox::new()
                .text(chip.pretty_name())
                .dimensions(self.score_box_side_length, self.chip_box_height)
                .position(
                    (self.width as f64 - self.score_box_side_length) / 2.0,
                    self.header_height as f64 - self.chip_box_height,
                )
                .background_color(GREEN_COLOUR)
                .font_color(PURPLE_COLOUR)
                .font_weight(FontWeight::Regular)
                .corner_rounding(CornerRounding::Bottom)
                .radius(self.score_box_radius)
                .build()?;

            document = document.add(chip_bg).add(chip_text);
        }

        let main_box_y = (self.header_height as f64 - self.side_box_height) / 2.0;
        let sub_box_y = main_box_y + self.side_box_height;

        // TEAM NAME
        let team_name_box_width = ((self.width as f64 - self.score_box_side_length) / 2.0)
            - (2.0 * self.side_box_padding);
        let (team_name_bg, team_name_text) = CenteredTextBox::new()
            .text(&data.team_name)
            .dimensions(team_name_box_width, self.side_box_height)
            .position(self.side_box_padding, main_box_y)
            .background_color(WHITE_COLOUR)
            .font_color(PURPLE_COLOUR)
            .font_weight(FontWeight::SemiBold)
            .corner_rounding(CornerRounding::All)
            .radius(self.score_box_radius)
            .inner_padding(0.95)
            .build()?;

        document = document.add(team_name_bg).add(team_name_text);

        // GWXY thing
        let (game_week_bg, game_week_text) = CenteredTextBox::new()
            .text(format!("GW{}", data.game_week))
            .dimensions(team_name_box_width / 4.0, self.side_box_height / 1.75)
            .position(
                self.side_box_padding + (team_name_box_width * 0.375),
                sub_box_y,
            )
            .background_color(PURPLE_COLOUR)
            .font_color(GREEN_COLOUR)
            .font_weight(FontWeight::Bold)
            .corner_rounding(CornerRounding::All)
            .radius(self.score_box_radius)
            .inner_padding(0.90)
            .build()?;

        document = document.add(game_week_bg).add(game_week_text);

        // GW RANK
        let gw_rank_text = if data.gw_rank == 0 {
            "-".to_string()
        } else {
            data.gw_rank.separate_with_commas()
        };
        let rank_box_width = (((self.width as f64 - self.score_box_side_length) / 2.0)
            - (3.0 * self.side_box_padding))
            / 2.0;
        let game_week_rank_x =
            ((self.width as f64 + self.score_box_side_length) / 2.0) + self.side_box_padding;
        let (game_week_rank_bg, game_week_rank_text) = CenteredTextBox::new()
            .text(gw_rank_text)
            .dimensions(rank_box_width, self.side_box_height)
            .position(game_week_rank_x, main_box_y)
            .background_color(WHITE_COLOUR)
            .font_color(PURPLE_COLOUR)
            .font_weight(FontWeight::SemiBold)
            .corner_rounding(CornerRounding::All)
            .radius(self.score_box_radius)
            .inner_padding(0.95)
            .build()?;

        document = document.add(game_week_rank_bg).add(game_week_rank_text);

        let (game_week_rank_title_bg, game_week_rank_title_text) = CenteredTextBox::new()
            .text("GW Rank".to_string())
            .dimensions(rank_box_width / 2.0, self.side_box_height / 1.75)
            .position(game_week_rank_x + (rank_box_width * 0.25), sub_box_y)
            .background_color(PURPLE_COLOUR)
            .font_color(GREEN_COLOUR)
            .font_weight(FontWeight::Bold)
            .corner_rounding(CornerRounding::All)
            .radius(self.score_box_radius)
            .inner_padding(0.90)
            .build()?;

        document = document
            .add(game_week_rank_title_bg)
            .add(game_week_rank_title_text);

        // OVERALL RANK
        let overall_rank_x = game_week_rank_x + rank_box_width + self.side_box_padding;
        let (overall_rank_bg, overall_rank_text) = CenteredTextBox::new()
            .text(data.overall_rank.separate_with_commas())
            .dimensions(rank_box_width, self.side_box_height)
            .position(overall_rank_x, main_box_y)
            .background_color(WHITE_COLOUR)
            .font_color(PURPLE_COLOUR)
            .font_weight(FontWeight::SemiBold)
            .corner_rounding(CornerRounding::All)
            .radius(self.score_box_radius)
            .inner_padding(0.95)
            .build()?;

        document = document.add(overall_rank_bg).add(overall_rank_text);

        let (overall_rank_title_bg, overall_rank_title_text) = CenteredTextBox::new()
            .text("Rank".to_string())
            .dimensions(rank_box_width / 2.0, self.side_box_height / 1.75)
            .position(overall_rank_x + (rank_box_width * 0.25), sub_box_y)
            .background_color(PURPLE_COLOUR)
            .font_color(GREEN_COLOUR)
            .font_weight(FontWeight::Bold)
            .corner_rounding(CornerRounding::All)
            .radius(self.score_box_radius)
            .inner_padding(0.90)
            .build()?;

        document = document
            .add(overall_rank_title_bg)
            .add(overall_rank_title_text);

        Ok(document)
    }

    fn add_transfers(
        &self,
        data: &TeamData,
        mut document: Document,
    ) -> Result<Document, std::io::Error> {
        let mut y_offset = self.header_height as f64
            + (5.0 * self.player_card_height as f64)
            + (6.0 * self.player_card_vertical_padding as f64)
            + self.transfer_row_vertical_padding;

        let (title_bg, title_text_box) = CenteredTextBox::new()
            .text("Transfers")
            .dimensions(200.0, self.transfer_row_vertical_padding * 2.0)
            .position(400.0, y_offset)
            .background_color(PURPLE_COLOUR)
            .font_color(WHITE_COLOUR)
            .font_weight(FontWeight::Bold)
            .corner_rounding(CornerRounding::Bottom)
            .radius(self.score_box_radius)
            .inner_padding(0.95)
            .build()?;

        document = document.add(title_bg).add(title_text_box);

        y_offset += self.transfer_row_vertical_padding;

        // 1 or 2 blocks, but always have them have the same width
        // for 2 blocks the row looks like
        // [padding] [[image] [text] [image]] [padding] [[image] [text] [image]] [padding]
        // padding and images have set width, text fills that width
        let transfer_block_width =
            (self.width as f64 - (3.0 * self.transfer_row_horizontal_padding)) / 2.0;
        let transfer_text_width = transfer_block_width - (2.0 * self.transfer_row_image_width);

        let mut is_first_chunk = true;

        for transfer_chunk in data.transfers.chunks(2) {
            if !is_first_chunk {
                // Draw a separator line before this chunk (except for the first chunk)
                let line_width = self.width as f64 - (2.0 * self.transfer_row_horizontal_padding);
                let line_x_start = (self.width as f64 - line_width) / 2.0;
                let line_x_end = line_x_start + line_width;

                let separator_line = svg::node::element::Line::new()
                    .set("x1", line_x_start)
                    .set(
                        "y1",
                        y_offset + (self.transfer_row_height as f64 / 2.0)
                            - self.transfer_row_vertical_padding / 2.0,
                    )
                    .set("x2", line_x_end)
                    .set(
                        "y2",
                        y_offset + (self.transfer_row_height as f64 / 2.0)
                            - self.transfer_row_vertical_padding / 2.0,
                    )
                    .set("stroke", PURPLE_COLOUR)
                    .set("stroke-width", 2);

                document = document.add(separator_line);
            } else {
                is_first_chunk = false;
            }

            let mut row_x = if transfer_chunk.len() == 2 {
                self.transfer_row_horizontal_padding
            } else {
                (self.width as f64 - transfer_block_width) / 2.0
            };

            let row_y: f64 = y_offset + (self.transfer_row_height as f64 / 2.0);

            for transfer in transfer_chunk {
                let out_image_path =
                    fpl_common::paths::get_player_image_path(transfer.player_out_code);

                let out_image = svg::node::element::Image::new()
                    .set("x", row_x)
                    .set("y", row_y)
                    .set("width", self.transfer_row_image_width)
                    .set("height", self.transfer_row_height)
                    .set("href", out_image_path)
                    .set("preserveAspectRatio", "xMidYMid meet");

                row_x += self.transfer_row_image_width;

                let transfer_text = format!(
                    "{} —› {}",
                    transfer.player_out_name, transfer.player_in_name,
                );

                let (transfer_text_bg, transfer_text_box) = CenteredTextBox::new()
                    .text(transfer_text)
                    .dimensions(transfer_text_width, self.transfer_row_height as f64 * 0.65)
                    .position(row_x, row_y)
                    .background_color(WHITE_COLOUR)
                    .font_color(PURPLE_COLOUR)
                    .font_weight(FontWeight::Bold)
                    .inner_padding(0.95)
                    .build()?;

                let transfer_cost_text = format!(
                    "£{} —› £{}",
                    transfer.player_out_cost, transfer.player_in_cost,
                );

                let (transfer_cost_bg, transfer_cost_box) = CenteredTextBox::new()
                    .text(transfer_cost_text)
                    .dimensions(
                        transfer_text_width * 0.7,
                        self.transfer_row_height as f64 * 0.35,
                    )
                    .position(
                        row_x + (transfer_text_width * 0.15),
                        row_y + (self.transfer_row_height as f64 * 0.65),
                    )
                    .background_color(PURPLE_COLOUR)
                    .font_color(GREEN_COLOUR)
                    .font_weight(FontWeight::ExtraBold)
                    .corner_rounding(CornerRounding::All)
                    .build()?;

                row_x += transfer_text_width;
                let in_image_path =
                    fpl_common::paths::get_player_image_path(transfer.player_in_code);
                let in_image = svg::node::element::Image::new()
                    .set("x", row_x)
                    .set("y", row_y)
                    .set("width", self.transfer_row_image_width)
                    .set("height", self.transfer_row_height)
                    .set("href", in_image_path)
                    .set("preserveAspectRatio", "xMidYMid meet");

                document = document
                    .add(in_image)
                    .add(out_image)
                    .add(transfer_text_bg)
                    .add(transfer_text_box)
                    .add(transfer_cost_bg)
                    .add(transfer_cost_box);

                row_x += self.transfer_row_image_width + self.transfer_row_horizontal_padding;
            }

            y_offset += self.transfer_row_height as f64 + self.transfer_row_vertical_padding;
        }

        Ok(document)
    }
}
