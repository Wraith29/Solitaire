use raylib::prelude::{Color, RaylibDrawHandle};

use crate::{
    constants::{CARD_HEIGHT, CARD_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH},
    deck::get_shuffled_deck,
    entity::Entity,
    foundation::Foundation,
    stock::{Pile, Stock},
    suit::Suit,
    tableau::Tableau,
};

pub struct Game {
    tableaus: Vec<Tableau>,
    foundations: Vec<Foundation>,
    stock: Stock,
}

impl Game {
    pub fn draw(&self, handle: &mut RaylibDrawHandle) {
        self.tableaus
            .iter()
            .for_each(|tableau| tableau.draw(handle));
        self.foundations
            .iter()
            .for_each(|foundation| foundation.draw(handle));
        self.stock.draw(handle);
    }

    pub fn new() -> Game {
        let deck = get_shuffled_deck();

        let mut total = 0;

        let tableaus = (1..=7)
            .map(|i| {
                let cards = deck[total..total + (i as usize)].to_vec();
                let tableau = Tableau::new(i, cards);

                total += i as usize;
                return tableau;
            })
            .collect();

        let stock = Stock::new(
            Pile::new(
                deck[total..].to_vec(),
                Entity::new(
                    WINDOW_WIDTH - ((CARD_WIDTH + 15) * 2),
                    WINDOW_HEIGHT - CARD_HEIGHT - 15,
                    CARD_WIDTH,
                    CARD_HEIGHT,
                    Color::WHITE,
                    Color::BLACK,
                    5,
                ),
            ),
            Pile::new(
                vec![],
                Entity::new(
                    WINDOW_WIDTH - CARD_WIDTH - 15,
                    WINDOW_HEIGHT - CARD_HEIGHT - 15,
                    CARD_WIDTH,
                    CARD_HEIGHT,
                    Color::WHITE,
                    Color::BLACK,
                    5,
                ),
            ),
        );

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
            foundations,
            tableaus,
            stock,
        }
    }
}
