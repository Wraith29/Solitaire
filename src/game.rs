use raylib::{
    prelude::{Color, RaylibDrawHandle},
    RaylibHandle,
};

use crate::{
    constants::{CARD_HEIGHT, CARD_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH},
    deck::get_shuffled_deck,
    entity::Entity,
    foundation::Foundation,
    state::State,
    stock::{Pile, Stock},
    suit::Suit,
    tableau::Tableau,
};

pub struct Game {
    tableaus: Vec<Tableau>,
    foundations: Vec<Foundation>,
    stock: Stock,
    state: State,
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

    pub fn on_click(&mut self, window: &RaylibHandle) {
        let mouse_pos = window.get_mouse_position();
        self.tableaus.iter().for_each(|tableau| {
            tableau.cards.iter().for_each(|card| {
                if card.entity.contains(mouse_pos) {
                    self.state.selected_card = Some(card.to_owned());
                }
            })
        });

        self.foundations.iter().for_each(|foundation| {
            foundation.cards.iter().for_each(|card| {
                if card.entity.contains(mouse_pos) {
                    self.state.selected_card = Some(card.to_owned());
                }
            });
        });

        self.stock.on_click(window);

        self.stock.waste.cards.iter().for_each(|card| {
            if card.entity.contains(mouse_pos) {
                self.state.selected_card = Some(card.to_owned());
            }
        });

        println!("Selected: {:#?}", self.state.selected_card);
    }

    pub fn on_drag(&mut self, window: &RaylibHandle) {
        let mouse_pos = window.get_mouse_position();
        let Some(mut card) = self.state.selected_card else { return };
        card.entity.x = mouse_pos.x as i32;
        card.entity.y = mouse_pos.y as i32;
        println!("Setting {card:#?} entity as {mouse_pos:?}");
    }

    pub fn on_release(&mut self, _window: &RaylibHandle) {
        self.state.selected_card = None;
    }

    pub fn new() -> Game {
        let deck = get_shuffled_deck();

        let mut total = 0;

        let tableaus = (1..=7)
            .map(|i| {
                let mut cards = deck[total..total + (i as usize)].to_vec();
                let tableau = Tableau::new(i, &mut cards);

                total += i as usize;
                tableau
            })
            .collect();

        let stock = Stock::new(
            Pile::new(
                deck[total..].to_vec(),
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
            Pile::new(
                vec![],
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
            state: State::new(),
        }
    }
}
