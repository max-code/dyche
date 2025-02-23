use super::{super::fonts::FPL_FONT_NAME, FontWeight};
use font_kit::{
    family_name::FamilyName,
    handle::Handle,
    properties::{Properties, Weight},
    source::SystemSource,
};
use rusttype::{point, Font, Scale};
use std::io;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::node::element::Text;

/*
move_to((a, b))
-Sets the starting point to coordinate (a, b).
horizontal_line_to(x)
-Draws a horizontal line from the current point to x (y remains unchanged).
vertical_line_to(y)
-Draws a vertical line from the current point to y (x remains unchanged).
elliptical_arc_by((rx, ry, rotation, large_arc_flag, sweep_flag, dx, dy))
-Draws an arc relative to the current point where:
-rx, ry: the radii of the ellipse in x and y directions
-rotation: the x-axis rotation of the ellipse (in degrees)
-large_arc_flag: 0 for an arc ≤180°, 1 for >180°
-sweep_flag: 0 for counterclockwise, 1 for clockwise
-dx, dy: the relative offset to the arc's end point
close()
-Closes the path by drawing a line back to the starting point.
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CornerRounding {
    #[default]
    All,
    Top,
    Bottom,
    None,
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
    font_family: String,
    font_weight: FontWeight,
    corner_rounding: CornerRounding,
    radius: f64,
    inner_padding: f64,
}

pub struct TextMetrics {
    width: f64,
    height: f64,
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
            font_family: FPL_FONT_NAME.to_string(),
            font_weight: FontWeight::SemiBold,
            corner_rounding: CornerRounding::None,
            radius: 10.0,
            inner_padding: 0.85,
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

    pub fn font_weight(mut self, weight: FontWeight) -> Self {
        self.font_weight = weight;
        self
    }

    pub fn font_family(mut self, family: &str) -> Self {
        self.font_family = family.into();
        self
    }

    pub fn corner_rounding(mut self, rounding: CornerRounding) -> Self {
        self.corner_rounding = rounding;
        self
    }

    pub fn radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }

    pub fn inner_padding(mut self, padding: f64) -> Self {
        self.inner_padding = padding;
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

            if metrics.width > self.width * self.inner_padding
                || metrics.height > self.height * self.inner_padding
            {
                high = mid;
            } else {
                low = mid;
            }
        }

        low
    }

    pub fn build(&self) -> Result<(Path, Text), std::io::Error> {
        let family = FamilyName::Title(self.font_family.clone());
        let weight_val: u16 = self.font_weight.to_number().parse().map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid font weight: {}", e),
            )
        })?;
        let properties = Properties {
            weight: Weight(weight_val as f32),
            ..Properties::default()
        };

        let handle = SystemSource::new()
            .select_best_match(&[family], &properties)
            .map_err(|e| io::Error::new(io::ErrorKind::NotFound, e.to_string()))?;

        let font_data = match handle {
            Handle::Path {
                path,
                font_index: _,
            } => std::fs::read(&path)?,
            Handle::Memory {
                bytes,
                font_index: _,
            } => bytes.to_vec(),
        };

        let data = match self.corner_rounding {
            CornerRounding::All => Data::new()
                .move_to((self.x + self.radius, self.y))
                .horizontal_line_to(self.x + self.width - self.radius)
                .elliptical_arc_by((self.radius, self.radius, 0, 0, 1, self.radius, self.radius))
                .vertical_line_to(self.y + self.height - self.radius)
                .elliptical_arc_by((self.radius, self.radius, 0, 0, 1, -self.radius, self.radius))
                .horizontal_line_to(self.x + self.radius)
                .elliptical_arc_by((
                    self.radius,
                    self.radius,
                    0,
                    0,
                    1,
                    -self.radius,
                    -self.radius,
                ))
                .vertical_line_to(self.y + self.radius)
                .elliptical_arc_by((self.radius, self.radius, 0, 0, 1, self.radius, -self.radius))
                .close(),
            CornerRounding::Top => Data::new()
                .move_to((self.x + self.radius, self.y))
                .horizontal_line_to(self.x + self.width - self.radius)
                .elliptical_arc_by((self.radius, self.radius, 0, 0, 1, self.radius, self.radius))
                .vertical_line_to(self.y + self.height)
                .horizontal_line_to(self.x)
                .vertical_line_to(self.y + self.radius)
                .elliptical_arc_by((self.radius, self.radius, 0, 0, 1, self.radius, -self.radius))
                .close(),
            CornerRounding::Bottom => Data::new()
                .move_to((self.x, self.y))
                .horizontal_line_to(self.x + self.width)
                .vertical_line_to(self.y + self.height - self.radius)
                .elliptical_arc_by((self.radius, self.radius, 0, 0, 1, -self.radius, self.radius))
                .horizontal_line_to(self.x + self.radius)
                .elliptical_arc_by((
                    self.radius,
                    self.radius,
                    0,
                    0,
                    1,
                    -self.radius,
                    -self.radius,
                ))
                .vertical_line_to(self.y)
                .close(),
            CornerRounding::None => Data::new()
                .move_to((self.x, self.y))
                .horizontal_line_to(self.x + self.width)
                .vertical_line_to(self.y + self.height)
                .horizontal_line_to(self.x)
                .vertical_line_to(self.y)
                .close(),
        };

        let path = Path::new()
            .set("d", data)
            .set("fill", self.background_color.clone());

        let font_size = self.calculate_optimal_font_size(&font_data);

        let x_centered = self.x + (self.width / 2.0);
        let y_centered = self.y + (self.height / 2.0);

        let text_element = Text::new(&self.text)
            .set("x", x_centered)
            .set("y", y_centered)
            .set("font-family", self.font_family.as_str())
            .set("font-weight", self.font_weight.to_number())
            .set("font-size", font_size)
            .set("fill", self.font_color.clone())
            .set("text-anchor", "middle")
            .set("dominant-baseline", "mathematical");

        Ok((path, text_element))
    }
}
