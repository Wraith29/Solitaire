use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::card::Card;
use crate::dimensions::Dimensions;
use crate::number::Number;
use crate::suit::Suit;

pub const DECK: [Card; 52] = [
    Card {
        number: Number::Ace,
        suit: Suit::Hearts,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
        flipped: true,
    },
    Card {
        number: Number::Two,
        suit: Suit::Hearts,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },

        flipped: true,
    },
    Card {
        number: Number::Three,
        suit: Suit::Hearts,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
        flipped: true,
    },
    Card {
        number: Number::Four,
        suit: Suit::Hearts,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
        flipped: true,
    },
    Card {
        number: Number::Five,
        suit: Suit::Hearts,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
        flipped: true,
    },
    Card {
        number: Number::Six,
        suit: Suit::Hearts,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
        flipped: true,
    },
    Card {
        number: Number::Seven,
        suit: Suit::Hearts,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
        flipped: true,
    },
    Card {
        number: Number::Eight,
        suit: Suit::Hearts,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
        flipped: true,
    },
    Card {
        number: Number::Nine,
        suit: Suit::Hearts,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
        flipped: true,
    },
    Card {
        number: Number::Ten,
        suit: Suit::Hearts,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
        flipped: true,
    },
    Card {
        number: Number::Jack,
        suit: Suit::Hearts,
        flipped: true,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
    Card {
        number: Number::Queen,
        flipped: true,
        suit: Suit::Hearts,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
    Card {
        number: Number::King,
        suit: Suit::Hearts,
        flipped: true,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
    Card {
        number: Number::Ace,
        suit: Suit::Clubs,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
        flipped: true,
    },
    Card {
        number: Number::Two,
        suit: Suit::Clubs,
        flipped: true,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
    Card {
        number: Number::Three,
        suit: Suit::Clubs,
        flipped: true,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
    Card {
        number: Number::Four,
        suit: Suit::Clubs,
        flipped: true,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
    Card {
        number: Number::Five,
        suit: Suit::Clubs,
        flipped: true,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
    Card {
        number: Number::Six,
        suit: Suit::Clubs,
        flipped: true,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
    Card {
        number: Number::Seven,
        flipped: true,
        suit: Suit::Clubs,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
    Card {
        number: Number::Eight,
        suit: Suit::Clubs,
        flipped: true,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
    Card {
        number: Number::Nine,
        suit: Suit::Clubs,
        flipped: true,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
    Card {
        number: Number::Ten,
        suit: Suit::Clubs,
        flipped: true,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
    Card {
        number: Number::Jack,
        suit: Suit::Clubs,
        flipped: true,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
    Card {
        number: Number::Queen,
        suit: Suit::Clubs,
        flipped: true,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
    Card {
        number: Number::King,
        suit: Suit::Clubs,
        flipped: true,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
    Card {
        number: Number::Ace,
        suit: Suit::Diamonds,
        flipped: true,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
    Card {
        number: Number::Two,
        suit: Suit::Diamonds,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
        flipped: true,
    },
    Card {
        number: Number::Three,
        suit: Suit::Diamonds,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
        flipped: true,
    },
    Card {
        number: Number::Four,
        suit: Suit::Diamonds,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
        flipped: true,
    },
    Card {
        number: Number::Five,
        suit: Suit::Diamonds,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
        flipped: true,
    },
    Card {
        number: Number::Six,
        suit: Suit::Diamonds,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
        flipped: true,
    },
    Card {
        number: Number::Seven,
        suit: Suit::Diamonds,
        flipped: true,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
    Card {
        number: Number::Eight,
        suit: Suit::Diamonds,
        flipped: true,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
    Card {
        number: Number::Nine,
        suit: Suit::Diamonds,
        flipped: true,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
    Card {
        number: Number::Ten,
        suit: Suit::Diamonds,
        flipped: true,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
    Card {
        number: Number::Jack,
        suit: Suit::Diamonds,
        flipped: true,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
    Card {
        number: Number::Queen,
        suit: Suit::Diamonds,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
        flipped: true,
    },
    Card {
        number: Number::King,
        suit: Suit::Diamonds,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
        flipped: true,
    },
    Card {
        number: Number::Ace,
        suit: Suit::Spades,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
        flipped: true,
    },
    Card {
        number: Number::Two,
        suit: Suit::Spades,
        flipped: true,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
    Card {
        number: Number::Three,
        suit: Suit::Spades,
        flipped: true,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
    Card {
        number: Number::Four,
        flipped: true,
        suit: Suit::Spades,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
    Card {
        number: Number::Five,
        flipped: true,
        suit: Suit::Spades,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
    Card {
        number: Number::Six,
        suit: Suit::Spades,
        flipped: true,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
    Card {
        number: Number::Seven,
        suit: Suit::Spades,
        flipped: true,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
    Card {
        number: Number::Eight,
        suit: Suit::Spades,
        flipped: true,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
    Card {
        number: Number::Nine,
        suit: Suit::Spades,
        flipped: true,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
    Card {
        number: Number::Ten,
        flipped: true,
        suit: Suit::Spades,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
    Card {
        number: Number::Jack,
        suit: Suit::Spades,
        flipped: true,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
    Card {
        number: Number::Queen,
        flipped: true,
        suit: Suit::Spades,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
    Card {
        number: Number::King,
        flipped: true,
        suit: Suit::Spades,
        dimensions: Dimensions {
            x: 0,
            y: 0,
            w: None,
            h: None,
        },
    },
];

pub fn get_random_deck() -> Vec<Card> {
    let mut res = DECK.to_vec();
    res.shuffle(&mut thread_rng());
    res
}
