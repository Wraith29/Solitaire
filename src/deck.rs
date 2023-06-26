use rand::{seq::SliceRandom, thread_rng};
use raylib::prelude::Color;

use crate::{
    card::Card,
    constants::{CARD_HEIGHT, CARD_WIDTH},
    entity::Entity,
    number::Number,
    suit::Suit,
};

const CARD_BASE_ENTITY: Entity = Entity {
    x: 0,
    y: 0,
    width: CARD_WIDTH,
    height: CARD_HEIGHT,
    colour: Color::WHITE,
    border_colour: Color::BLACK,
    border_offset: 5,
};

const DECK: [Card; 52] = [
    Card {
        suit: Suit::Hearts,
        number: Number::Ace,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Hearts,
        number: Number::Two,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Hearts,
        number: Number::Three,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Hearts,
        number: Number::Four,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Hearts,
        number: Number::Five,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Hearts,
        number: Number::Six,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Hearts,
        number: Number::Seven,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Hearts,
        number: Number::Eight,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Hearts,
        number: Number::Nine,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Hearts,
        number: Number::Ten,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Hearts,
        number: Number::Jack,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Hearts,
        number: Number::Queen,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Hearts,
        number: Number::King,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Diamonds,
        number: Number::Ace,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Diamonds,
        number: Number::Two,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Diamonds,
        number: Number::Three,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Diamonds,
        number: Number::Four,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Diamonds,
        number: Number::Five,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Diamonds,
        number: Number::Six,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Diamonds,
        number: Number::Seven,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Diamonds,
        number: Number::Eight,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Diamonds,
        number: Number::Nine,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Diamonds,
        number: Number::Ten,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Diamonds,
        number: Number::Jack,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Diamonds,
        number: Number::Queen,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Diamonds,
        number: Number::King,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Clubs,
        number: Number::Ace,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Clubs,
        number: Number::Two,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Clubs,
        number: Number::Three,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Clubs,
        number: Number::Four,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Clubs,
        number: Number::Five,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Clubs,
        number: Number::Six,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Clubs,
        number: Number::Seven,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Clubs,
        number: Number::Eight,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Clubs,
        number: Number::Nine,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Clubs,
        number: Number::Ten,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Clubs,
        number: Number::Jack,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Clubs,
        number: Number::Queen,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Clubs,
        number: Number::King,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Spades,
        number: Number::Ace,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Spades,
        number: Number::Two,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Spades,
        number: Number::Three,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Spades,
        number: Number::Four,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Spades,
        number: Number::Five,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Spades,
        number: Number::Six,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Spades,
        number: Number::Seven,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Spades,
        number: Number::Eight,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Spades,
        number: Number::Nine,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Spades,
        number: Number::Ten,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Spades,
        number: Number::Jack,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Spades,
        number: Number::Queen,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
    Card {
        suit: Suit::Spades,
        number: Number::King,
        entity: CARD_BASE_ENTITY,
        hidden: true,
    },
];

pub fn get_shuffled_deck() -> Vec<Card> {
    let mut deck_copy = DECK.to_vec();
    deck_copy.shuffle(&mut thread_rng());

    deck_copy
}
