use raylib::prelude::RaylibDrawHandle;

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
}
