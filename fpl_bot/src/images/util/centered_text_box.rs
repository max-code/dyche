use rusttype::{point, Font, Scale};
use std::fs::read as read_file;
use std::path::PathBuf;
use svg::node::element::{Rectangle, Text};

use super::super::fonts::FPL_FONT_NAME;

pub struct TextMetrics {
    width: f64,
    height: f64,
}

#[derive(Default)]
pub struct CenteredTextBox {
    text: String,
    width: f64,
    height: f64,
    x: f64,
    y: f64,
    background_color: String,
    font_color: String,
    font_path: PathBuf,
}

impl CenteredTextBox {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            width: 0.0,
            height: 0.0,
            x: 0.0,
            y: 0.0,
            background_color: String::from("#800080"),
            font_color: String::from("#000000"),
            font_path: PathBuf::from(
                "/Users/maxjordan/code/dyche/fpl_assets/fonts/Radikal-Bold.otf",
            ),
        }
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    pub fn dimensions(mut self, width: f64, height: f64) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn position(mut self, x: f64, y: f64) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn background_color(mut self, color: impl Into<String>) -> Self {
        self.background_color = color.into();
        self
    }

    pub fn font_color(mut self, color: impl Into<String>) -> Self {
        self.font_color = color.into();
        self
    }

    pub fn font_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.font_path = path.into();
        self
    }

    fn measure_text(&self, text: &str, font_size: f64, font_data: &[u8]) -> TextMetrics {
        let font = Font::try_from_bytes(font_data).expect("Error constructing Font");
        let scale = Scale::uniform(font_size as f32);
        let v_metrics = font.v_metrics(scale);
        let height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;

        let glyphs: Vec<_> = font
            .layout(text, scale, point(0.0, v_metrics.ascent))
            .collect();

        let width = if glyphs.is_empty() {
            0.0
        } else {
            let min_x = glyphs
                .first()
                .unwrap()
                .pixel_bounding_box()
                .map(|b| b.min.x)
                .unwrap_or(0);
            let max_x = glyphs
                .last()
                .unwrap()
                .pixel_bounding_box()
                .map(|b| b.max.x)
                .unwrap_or(0);
            (max_x - min_x) as f64
        };

        TextMetrics {
            width,
            height: height as f64,
        }
    }

    fn calculate_optimal_font_size(&self, font_data: &[u8]) -> f64 {
        let mut low = 1.0;
        let mut high = self.height;
        let tolerance = 0.5;

        while high - low > tolerance {
            let mid = (low + high) / 2.0;
            let metrics = self.measure_text(&self.text, mid, font_data);

            if metrics.width > self.width * 0.85 || metrics.height > self.height * 0.85 {
                high = mid;
            } else {
                low = mid;
            }
        }

        low
    }

    pub fn build(&self) -> Result<(Rectangle, Text), std::io::Error> {
        let font_data = read_file(&self.font_path)?;

        // Create the background rectangle
        let rect = Rectangle::new()
            .set("x", self.x)
            .set("y", self.y)
            .set("width", self.width)
            .set("height", self.height)
            .set("fill", self.background_color.clone());

        // Calculate optimal font size and center positions
        let font_size = self.calculate_optimal_font_size(&font_data);
        let x_centered = self.x + (self.width / 2.0);
        let y_centered = self.y + (self.height / 2.0);

        // Create the text element
        let text_element = Text::new(&self.text)
            .set("x", x_centered)
            .set("y", y_centered)
            .set("font-family", FPL_FONT_NAME)
            .set("font-size", font_size)
            .set("fill", self.font_color.clone())
            .set("text-anchor", "middle")
            .set("dominant-baseline", "central");

        Ok((rect, text_element))
    }
}
