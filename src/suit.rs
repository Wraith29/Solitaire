use raylib::prelude::Color;

#[derive(Debug, Clone, Copy)]
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

    pub fn is_red(&self) -> bool {
        match self {
            Suit::Hearts | Suit::Diamonds => true,
            Suit::Clubs | Suit::Spades => false,
        }
    }

    // Just a slightly nicer way than manually doing `!is_red`
    pub fn is_black(&self) -> bool {
        !self.is_red()
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
