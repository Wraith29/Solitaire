use raylib::prelude::{RaylibDrawHandle, Vector2};

use crate::{
    deck::get_random_deck,
    dimensions::Dimensions,
    foundation::Foundation,
    stack::Stack,
    suit::Suit,
    tableau::Tableau,
    window::{CARD_HEIGHT, CARD_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH},
};

const FOUNDATION_Y: i32 = (WINDOW_HEIGHT - CARD_HEIGHT) - 15;

fn clicked(dimensions: Dimensions, mouse: Vector2) -> bool {
    true
}

pub struct Game {
    tableaus: Vec<Tableau>,
    stock: Stack,
    waste: Stack,
    foundations: Vec<Foundation>,
}

impl Game {
    pub fn new() -> Game {
        let deck = get_random_deck();

        let mut tableaus = Vec::new();
        let mut total = 0;
        (1..=7).for_each(|i| {
            let mut tableau = Tableau::new(i);
            (0..i).for_each(|j| {
                tableau.push(deck[(i + j) as usize]);
            });
            tableaus.push(tableau);
            total += i;
        });

        // X = WINDOW_WIDTH - (CARD_WIDTH + 30) // 30 = buffer
        let stock_dim = Dimensions::new(
            WINDOW_WIDTH - (CARD_WIDTH + 30),
            WINDOW_HEIGHT - (CARD_HEIGHT + 30),
            Some(CARD_WIDTH),
            Some(CARD_HEIGHT),
        );

        let stock = Stack::new(
            deck[total as usize..]
                .to_vec()
                .iter()
                .map(|&card| {
                    let mut card_copy = card.clone();
                    card_copy.dimensions = stock_dim;
                    card_copy
                })
                .collect(),
            stock_dim,
        );

        let waste = Stack::new(vec![], stock_dim);

        Game {
            tableaus,
            stock,
            waste,
            foundations: vec![
                Foundation::new(Suit::Hearts, Dimensions::new(15, FOUNDATION_Y, None, None)),
                Foundation::new(
                    Suit::Diamonds,
                    Dimensions::new(CARD_WIDTH + 30, FOUNDATION_Y, None, None),
                ),
                Foundation::new(
                    Suit::Clubs,
                    Dimensions::new(CARD_WIDTH * 2 + 45, FOUNDATION_Y, None, None),
                ),
                Foundation::new(
                    Suit::Spades,
                    Dimensions::new(CARD_WIDTH * 3 + 60, FOUNDATION_Y, None, None),
                ),
            ],
        }
    }

    pub fn draw(&self, handle: &mut RaylibDrawHandle) {
        self.tableaus
            .iter()
            .for_each(|tableau| tableau.draw(handle));
        self.stock.draw(handle);
        self.waste.draw(handle);
        self.foundations
            .iter()
            .for_each(|foundation| foundation.draw(handle));
    }

    pub fn on_click(&self, handle: &mut RaylibDrawHandle) {
        let pos = handle.get_mouse_position();
        if clicked(self.stock.dimensions, pos) {}
    }
}
