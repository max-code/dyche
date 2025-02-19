use resvg::{render, usvg};
use svg::node::element::{Group, Rectangle, Text};
use svg::Document;
use tiny_skia::Pixmap;
use usvg::{Options, Tree};

use crate::images::constants::colours::PURPLE_COLOUR;
use crate::images::constants::fonts::FPL_FONT_NAME;

use super::colours::GREEN_COLOUR;

#[derive(Debug, Clone)]
pub struct TableRow {
    pub name: String,
    pub team_name: String,
    pub confirmed_points: u16,
    pub live_points: u16,
    pub caller: bool,
}

impl TableRow {
    pub fn new(
        name: String,
        team_name: String,
        confirmed_points: u16,
        live_points: u16,
        caller: bool,
    ) -> Self {
        Self {
            name,
            team_name,
            confirmed_points,
            live_points,
            caller,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TableData {
    pub title: String,
    pub rows: Vec<TableRow>,
}

impl TableData {
    pub fn new(title: String) -> Self {
        Self {
            title,
            rows: Vec::new(),
        }
    }

    pub fn add_row(
        &mut self,
        name: String,
        team_name: String,
        confirmed_points: u16,
        live_points: u16,
        caller: bool,
    ) {
        self.rows.push(TableRow::new(
            name,
            team_name,
            confirmed_points,
            live_points,
            caller,
        ));
    }
}

#[derive(Debug, Clone)]
pub struct TableRenderer {
    pub width: u32,
    pub row_height: u32,
    pub header_height: u32,
    pub title_height: u32,
    pub padding: u32,
}

impl Default for TableRenderer {
    fn default() -> Self {
        Self {
            width: 1000,
            row_height: 80,
            header_height: 60,
            title_height: 80,
            padding: 20,
        }
    }
}

impl TableRenderer {
    pub async fn render(&self, data: TableData, path: &str) -> std::io::Result<()> {
        let total_height =
            self.title_height + self.header_height + (data.rows.len() as u32 * self.row_height);

        let mut document = Document::new()
            .set("viewBox", (0, 0, self.width, total_height))
            .set("width", self.width)
            .set("height", total_height);

        // Add title background and text
        let title_bg = Rectangle::new()
            .set("x", 0)
            .set("y", 0)
            .set("width", self.width)
            .set("height", self.title_height)
            .set("fill", PURPLE_COLOUR);

        let title_text = Text::new(&data.title)
            .set("x", self.width / 2)
            .set("y", self.title_height / 2 + 10) // +10 for vertical centering
            .set("text-anchor", "middle")
            .set("fill", "#FFFFFF")
            .set("font-family", FPL_FONT_NAME)
            .set("font-weight", "900")
            .set("font-size", "32");

        document = document.add(title_bg).add(title_text);

        // Add header background
        let header_bg = Rectangle::new()
            .set("x", 0)
            .set("y", self.title_height)
            .set("width", self.width)
            .set("height", self.header_height)
            .set("fill", "#F0F0F0");

        let header_center = self.title_height + (self.header_height / 2);

        let header_group = Group::new()
            .add(header_bg)
            .add(
                Text::new("Rank")
                    .set("x", self.padding)
                    .set("y", header_center)
                    .set("fill", PURPLE_COLOUR)
                    .set("font-family", FPL_FONT_NAME)
                    .set("font-weight", "bold")
                    .set("font-size", "28")
                    .set("dominant-baseline", "middle")
                    .set("alignment-baseline", "middle"),
            )
            .add(
                Text::new("Team and Manager")
                    .set("x", 125)
                    .set("y", header_center)
                    .set("fill", PURPLE_COLOUR)
                    .set("font-family", FPL_FONT_NAME)
                    .set("font-weight", "bold")
                    .set("font-size", "28")
                    .set("dominant-baseline", "middle")
                    .set("alignment-baseline", "middle"),
            )
            .add(
                Text::new("Confirmed Points")
                    .set("x", self.width - 500)
                    .set("y", header_center)
                    .set("fill", PURPLE_COLOUR)
                    .set("font-family", FPL_FONT_NAME)
                    .set("font-weight", "bold")
                    .set("font-size", "28")
                    .set("dominant-baseline", "middle")
                    .set("alignment-baseline", "middle"),
            )
            .add(
                Text::new("Live Points")
                    .set("x", self.width - 200)
                    .set("y", header_center)
                    .set("fill", PURPLE_COLOUR)
                    .set("font-family", FPL_FONT_NAME)
                    .set("font-weight", "bold")
                    .set("font-size", "28")
                    .set("dominant-baseline", "middle")
                    .set("alignment-baseline", "middle"),
            );

        document = document.add(header_group);

        // Add rows
        for (index, row) in data.rows.iter().enumerate() {
            let y_pos = self.title_height + self.header_height + (index as u32 * self.row_height);

            // Row background
            let bg_color = if index % 2 == 0 { "#FFFFFF" } else { "#F0F0F0" };
            let row_bg_colour = if row.caller { GREEN_COLOUR } else { bg_color };
            let row_bg = Rectangle::new()
                .set("x", 0)
                .set("y", y_pos)
                .set("width", self.width)
                .set("height", self.row_height)
                .set("fill", row_bg_colour);

            let bottom_border = svg::node::element::Line::new()
                .set("x1", 0)
                .set("y1", y_pos)
                .set("x2", self.width)
                .set("y2", y_pos)
                .set("stroke", PURPLE_COLOUR)
                .set("stroke-width", 1);

            let row_group = Group::new()
                .add(row_bg)
                .add(bottom_border)
                .add(
                    Text::new((index + 1).to_string())
                        .set("x", self.padding)
                        .set("y", y_pos + 45)
                        .set("fill", PURPLE_COLOUR)
                        .set("font-family", FPL_FONT_NAME)
                        .set("font-size", "24")
                        .set("font-weight", "bold"),
                )
                .add(
                    Text::new("")
                        .set("x", 125)
                        .set("y", y_pos + (self.row_height / 2))
                        .set("fill", PURPLE_COLOUR)
                        .set("font-family", FPL_FONT_NAME)
                        .set("dominant-baseline", "middle")
                        .add(
                            svg::node::element::TSpan::new(&row.team_name)
                                .set("x", 125)
                                .set("dy", "-0.5em")
                                .set("font-weight", "800")
                                .set("font-size", "20"),
                        )
                        .add(
                            svg::node::element::TSpan::new(&row.name)
                                .set("x", 125)
                                .set("dy", "1.3em")
                                .set("font-size", "20"),
                        ),
                )
                .add(
                    Text::new(row.confirmed_points.to_string())
                        .set("x", self.width - 500)
                        .set("y", y_pos + 45)
                        .set("fill", PURPLE_COLOUR)
                        .set("font-family", FPL_FONT_NAME)
                        .set("font-size", "24"),
                )
                .add(
                    Text::new(row.live_points.to_string())
                        .set("x", self.width - 200)
                        .set("y", y_pos + 45)
                        .set("fill", PURPLE_COLOUR)
                        .set("font-family", FPL_FONT_NAME)
                        .set("font-size", "24"),
                );

            document = document.add(row_group);
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
