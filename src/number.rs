#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Number {
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}

impl Number {
    pub fn next(&self) -> Number {
        match self {
            Number::Ace => Number::Two,
            Number::Two => Number::Three,
            Number::Three => Number::Four,
            Number::Four => Number::Five,
            Number::Five => Number::Six,
            Number::Six => Number::Seven,
            Number::Seven => Number::Eight,
            Number::Eight => Number::Nine,
            Number::Nine => Number::Ten,
            Number::Ten => Number::Jack,
            Number::Jack => Number::Queen,
            Number::Queen => Number::King,
            Number::King => Number::Ace,
        }
    }
}

impl ToString for Number {
    fn to_string(&self) -> String {
        match self {
            Number::Ace => "A",
            Number::Two => "2",
            Number::Three => "3",
            Number::Four => "4",
            Number::Five => "5",
            Number::Six => "6",
            Number::Seven => "7",
            Number::Eight => "8",
            Number::Nine => "9",
            Number::Ten => "10",
            Number::Jack => "J",
            Number::Queen => "Q",
            Number::King => "K",
        }
        .into()
    }
}
