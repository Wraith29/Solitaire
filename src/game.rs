use raylib::prelude::{Color, RaylibDrawHandle};

use crate::{
    card::Card,
    constants::{CARD_HEIGHT, CARD_WIDTH, WINDOW_HEIGHT},
    deck::get_shuffled_deck,
    entity::Entity,
    foundation::Foundation,
    suit::Suit,
    tableau::Tableau,
};

pub struct Game {
    deck: Vec<Card>,
    tableaus: Vec<Tableau>,
    foundations: Vec<Foundation>,
}

impl Game {
    pub fn new() -> Game {
        let deck = get_shuffled_deck();

        let mut total = 0;

        let tableaus = (0..=6)
            .map(|i| {
                let cards = deck[total..total + (i as usize)].to_vec();
                let tableau = Tableau::new(i, cards);

                total += i as usize;
                return tableau;
            })
            .collect::<Vec<_>>();

        let foundation_y_pos = WINDOW_HEIGHT - CARD_HEIGHT - 15;
        let foundations = vec![
            Foundation::new(
                Suit::Hearts,
                Entity::new(
                    15,
                    foundation_y_pos,
                    CARD_WIDTH,
                    CARD_HEIGHT,
                    Color::WHITE,
                    Color::BLACK,
                    5,
                ),
            ),
            Foundation::new(
                Suit::Diamonds,
                Entity::new(
                    CARD_WIDTH + 30,
                    foundation_y_pos,
                    CARD_WIDTH,
                    CARD_HEIGHT,
                    Color::WHITE,
                    Color::BLACK,
                    5,
                ),
            ),
            Foundation::new(
                Suit::Clubs,
                Entity::new(
                    (CARD_WIDTH * 2) + 45,
                    foundation_y_pos,
                    CARD_WIDTH,
                    CARD_HEIGHT,
                    Color::WHITE,
                    Color::BLACK,
                    5,
                ),
            ),
            Foundation::new(
                Suit::Spades,
                Entity::new(
                    (CARD_WIDTH * 3) + 60,
                    foundation_y_pos,
                    CARD_WIDTH,
                    CARD_HEIGHT,
                    Color::WHITE,
                    Color::BLACK,
                    5,
                ),
            ),
        ];

        Game {
            deck,
            foundations,
            tableaus,
        }
    }

    pub fn draw(&self, handle: &mut RaylibDrawHandle) {
        self.deck.iter().for_each(|card| card.draw(handle));
        self.tableaus
            .iter()
            .for_each(|tableau| tableau.draw(handle));
        self.foundations
            .iter()
            .for_each(|foundation| foundation.draw(handle));
    }
}
