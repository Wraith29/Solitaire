use raylib::prelude::Color;

#[derive(Clone, Copy)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    pub fn colour(&self) -> Color {
        match self {
            Suit::Hearts | Suit::Diamonds => Color::RED,
            Suit::Clubs | Suit::Spades => Color::BLACK,
        }
    }
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
