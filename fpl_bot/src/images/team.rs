use fpl_common::types::Chip;
use resvg::{render, usvg};
use svg::node::element::{Group, Rectangle, Text};
use svg::Document;
use tiny_skia::Pixmap;
use usvg::{Options, Tree};

use crate::images::constants::colours::PURPLE_COLOUR;
use crate::images::constants::fonts::FPL_FONT_NAME;

use crate::images::util::PlayerInfo;

use super::player_card;

#[derive(Debug, Clone)]
pub struct TransferInfo {
    pub player_in_code: u32,
    pub player_in_cost: f64,
    pub player_out_code: u32,
    pub player_out_cost: f64,
}

impl TransferInfo {
    pub fn new(
        player_in_code: u32,
        player_in_cost: f64,
        player_out_code: u32,
        player_out_cost: f64,
    ) -> Self {
        Self {
            player_in_code,
            player_in_cost,
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
        let team_name = self.team_name.ok_or("Team name is required")?;
        let gw_rank = self.gw_rank.ok_or("Gameweek rank is required")?;
        let overall_rank = self.overall_rank.ok_or("Overall rank is required")?;
        let goalkeeper = self.goalkeeper.ok_or("Goalkeeper is required")?;

        Ok(TeamData {
            team_name,
            gw_rank,
            overall_rank,
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
}

impl Default for TeamRenderer {
    fn default() -> Self {
        Self {
            width: 1000,
            player_card_height: 200,
            player_card_width: 150,
            player_card_vertical_padding: 50,
            transfer_row_height: 200,
            header_height: 300,
        }
    }
}

impl TeamRenderer {
    pub async fn render(&self, data: TeamData, path: &str) -> std::io::Result<()> {
        let total_height = self.header_height
            + (5 * self.player_card_height)
            + (5 * self.player_card_vertical_padding)
            + (data.transfers.len() as u32 * self.transfer_row_height);

        // TODO: Reset these to self.width and self.height
        let mut document: svg::node::element::SVG = Document::new()
            .set("viewBox", (0, 0, 1000, total_height))
            .set("width", 1000)
            .set("height", total_height);

        let background = Rectangle::new()
            .set("x", 0)
            .set("y", 0)
            .set("width", "100%")
            .set("height", "100%")
            .set("fill", "#4ac24c");

        document = document.add(background);

        document = self.add_player_cards(&data, document)?;

        let svg_string = document.to_string();
        let mut opt: Options<'_> = Options::default();
        opt.fontdb_mut().load_system_fonts();

        let tree = Tree::from_str(&svg_string, &opt).unwrap();
        let size = tree.size();
        let mut pixmap = Pixmap::new(size.width() as u32, size.height() as u32).unwrap();
        render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

        std::fs::write(path, pixmap.encode_png().unwrap())
    }

    fn calculate_player_card_xs(&self, num_cards: usize) -> Vec<u32> {
        let total_box_width = self.player_card_width * (num_cards as u32);
        let remaining_space = self.width.saturating_sub(total_box_width);

        let gaps = num_cards + 1;
        let gap_width = remaining_space / (gaps as u32);
        (0..num_cards)
            .map(|i| gap_width + i as u32 * (self.player_card_width + gap_width))
            .collect()
    }

    fn add_player_cards(
        &self,
        data: &TeamData,
        mut document: Document,
    ) -> Result<Document, std::io::Error> {
        let mut y_offset = self.header_height + self.player_card_vertical_padding;
        for row in data.get_player_rows() {
            let xs = self.calculate_player_card_xs(row.len());

            for (x_offset, player) in xs.iter().zip(row.iter()) {
                let player_card = player.to_card_svg(
                    *x_offset,
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
}
