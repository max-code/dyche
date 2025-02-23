pub mod centered_text_box;
pub mod player_card;

pub use centered_text_box::*;
pub use player_card::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FontWeight {
    UltraThin,
    Thin,
    Light,
    Regular,
    #[default]
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
}

impl FontWeight {
    pub fn to_number(self) -> &'static str {
        match self {
            FontWeight::UltraThin => "100",
            FontWeight::Thin => "200",
            FontWeight::Light => "300",
            FontWeight::Regular => "400",
            FontWeight::Medium => "500",
            FontWeight::SemiBold => "600",
            FontWeight::Bold => "700",
            FontWeight::ExtraBold => "800",
            FontWeight::Black => "900",
        }
    }
}
