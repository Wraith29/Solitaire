use raylib::prelude::Color;

#[derive(PartialEq, Clone, Copy)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl ToString for Suit {
    fn to_string(&self) -> String {
        match self {
            Suit::Hearts => "H",
            Suit::Diamonds => "D",
            Suit::Clubs => "C",
            Suit::Spades => "S",
        }
        .into()
    }
}

impl Suit {
    pub fn get_colour(&self) -> Color {
        match self {
            Suit::Hearts | Suit::Diamonds => Color::RED,
            Suit::Clubs | Suit::Spades => Color::BLACK,
        }
    }
}
