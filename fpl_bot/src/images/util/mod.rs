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

pub fn calculate_player_card_xs(
    player_card_width: u32,
    total_width: u32,
    num_cards: u32,
) -> Vec<u32> {
    let total_box_width = player_card_width * num_cards;
    let remaining_space = total_width.saturating_sub(total_box_width);

    let gaps = num_cards + 1;
    let gap_width = remaining_space / gaps;
    (0..num_cards)
        .map(|i| gap_width + i * (player_card_width + gap_width))
        .collect()
}
