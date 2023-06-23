#[derive(PartialEq, Clone, Copy)]
pub enum Number {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Ace,
    Jack,
    Queen,
    King,
}

impl ToString for Number {
    fn to_string(&self) -> String {
        match self {
            Number::Two => "2",
            Number::Three => "3",
            Number::Four => "4",
            Number::Five => "5",
            Number::Six => "6",
            Number::Seven => "7",
            Number::Eight => "8",
            Number::Nine => "9",
            Number::Ten => "10",
            Number::Ace => "A",
            Number::Jack => "J",
            Number::Queen => "Q",
            Number::King => "K",
        }
        .into()
    }
}
