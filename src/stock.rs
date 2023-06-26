use raylib::{prelude::RaylibDrawHandle, RaylibHandle};

use crate::{card::Card, entity::Entity};

pub struct Pile {
    cards: Vec<Card>,
    entity: Entity,
}

impl Pile {
    pub fn new(cards: Vec<Card>, entity: Entity) -> Pile {
        let updated_cards = cards
            .iter()
            .map(|card| {
                let mut clone = *card;
                clone.entity = entity;

                clone
            })
            .collect();

        Pile {
            cards: updated_cards,
            entity,
        }
    }

    pub fn draw(&self, handle: &mut RaylibDrawHandle) {
        self.entity.draw(handle);

        self.cards.iter().for_each(|card| card.draw(handle));
    }
}

pub struct Stock {
    stock: Pile,
    waste: Pile,
}

impl Stock {
    pub fn new(stock: Pile, waste: Pile) -> Stock {
        Stock { stock, waste }
    }

    pub fn draw(&self, handle: &mut RaylibDrawHandle) {
        self.stock.draw(handle);
        self.waste.draw(handle);
    }

    pub fn on_click(&mut self, window: &RaylibHandle) {
        let mouse_pos = window.get_mouse_position();

        if self.stock.entity.contains(mouse_pos) {
            if self.stock.cards.len() > 0 {
                // TODO: Remove Unwrap at some point
                let mut card = self.stock.cards.pop().unwrap();
                card.flipped = false;
                card.entity = self.waste.entity;

                self.waste.cards.push(card);
            } else {
                self.stock.cards = self
                    .waste
                    .cards
                    .to_owned()
                    .iter()
                    .map(|card| {
                        let mut card_copy = *card;
                        card_copy.entity = self.stock.entity;
                        card_copy.flipped = true;
                        card_copy
                    })
                    .collect();

                self.stock.cards.reverse();
                self.waste.cards = vec![];
            }
        }

        if self.waste.entity.contains(mouse_pos) {
            println!("Waste Clicked :)");
        }
    }
}
