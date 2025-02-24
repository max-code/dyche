use svg::node::element::{Group, Rectangle};

use crate::images::colours::{GREEN_COLOUR, GREY_COLOUR, WHITE_COLOUR};

use super::super::constants::colours::PURPLE_COLOUR;
use super::{CenteredTextBox, CornerRounding};

#[derive(Debug, Clone)]
pub enum GameStatus {
    NotPlayed,
    Played(i16),
}

#[derive(Debug, Clone)]
pub enum PlayerGameInfo {
    Status(GameStatus),
    Fixture(String),
}

impl PlayerGameInfo {
    fn pretty_str(&self) -> String {
        match self {
            PlayerGameInfo::Status(status) => match status {
                GameStatus::NotPlayed => "-".to_string(),
                GameStatus::Played(points) => format!("{} pts", points),
            },
            PlayerGameInfo::Fixture(fixture) => fixture.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerInfo {
    pub name: String,
    pub code: u32,
    pub games: Vec<PlayerGameInfo>,
    pub captain: bool,
    pub vice_captain: bool,
    pub multiplier: i16,
}

impl PlayerInfo {
    pub fn new(
        name: String,
        code: u32,
        games: Vec<PlayerGameInfo>,
        captain: bool,
        vice_captain: bool,
        multiplier: i16,
    ) -> Self {
        Self {
            name,
            code,
            games,
            captain,
            vice_captain,
            multiplier,
        }
    }

    pub fn to_card_svg(
        &self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Result<Group, std::io::Error> {
        let image_height = height * 2 / 3;
        let text_row_height = (height - image_height) / 2;
        let border_radius = 8;

        let group = Group::new();

        // Semi-transparent border with more visible border
        let background = Rectangle::new()
            .set("x", x)
            .set("y", y)
            .set("width", width)
            .set("height", height)
            .set("rx", border_radius)
            .set("ry", border_radius)
            .set("fill", "rgba(27, 12, 12, 0.1)")
            .set("stroke", GREEN_COLOUR)
            .set("stroke-width", 2);

        // Player image
        let image_path = format!(
            "/Users/maxjordan/code/dyche/fpl_assets/player_images/{}.png",
            self.code
        );
        let image = svg::node::element::Image::new()
            .set("x", x + 5)
            .set("y", y + 5)
            .set("width", width - 10)
            .set("height", image_height)
            .set("href", image_path)
            .set("preserveAspectRatio", "xMidYMid meet");

        // TOP ROW: Name
        let name = match (self.captain, self.vice_captain) {
            (true, _) => format!("{} (C)", self.name),
            (_, true) => format!("{} (VC)", self.name),
            (_, _) => self.name.clone(),
        };

        let name_y = y + image_height;
        let (name_bg, name_text) = CenteredTextBox::new()
            .text(&name)
            .dimensions(width.into(), text_row_height.into())
            .position(x.into(), name_y.into())
            .background_color(WHITE_COLOUR)
            .font_color(PURPLE_COLOUR)
            .build()?;

        // BOTTOM ROW: Game Info (pts or opponent)
        let status_text = self
            .games
            .iter()
            .map(|game| game.pretty_str())
            .collect::<Vec<String>>()
            .join(", ");

        let status_bg_colour = match self
            .games
            .iter()
            .any(|f| matches!(f, PlayerGameInfo::Status(GameStatus::Played(_))))
        {
            true => PURPLE_COLOUR,
            false => GREY_COLOUR,
        };

        let status_y = name_y + text_row_height;
        let (status_bg, status_text) = CenteredTextBox::new()
            .text(&status_text)
            .dimensions(width.into(), text_row_height.into())
            .position(x.into(), status_y.into())
            .background_color(status_bg_colour)
            .font_color(WHITE_COLOUR)
            .corner_rounding(CornerRounding::Bottom)
            .radius(border_radius as f64)
            .build()?;

        // Assemble all elements in correct order
        Ok(group
            .add(status_bg)
            .add(background)
            .add(image)
            .add(name_bg)
            .add(name_text)
            .add(status_text))
    }
}
