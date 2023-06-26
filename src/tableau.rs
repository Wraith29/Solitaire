use raylib::prelude::{Color, RaylibDrawHandle};

use crate::{
    card::Card,
    constants::{CARD_HEIGHT, CARD_WIDTH, TABLEAU_WIDTH, WINDOW_WIDTH},
    entity::Entity,
};

pub struct Tableau {
    entity: Entity,
    pub cards: Vec<Card>,
}

impl Tableau {
    pub fn new(offset: i32, cards: Vec<Card>) -> Tableau {
        let total_tableau_width = (TABLEAU_WIDTH * 7) + (15 * 8);
        let outer_margin_per_side = (WINDOW_WIDTH - total_tableau_width) / 2;

        let x_coord = outer_margin_per_side + (15 * offset) + (TABLEAU_WIDTH * (offset - 1));

        let height = CARD_HEIGHT + 15 + (15 * offset);
        let entity = Entity::new(
            x_coord,
            15,
            CARD_WIDTH,
            height,
            Color::WHITE,
            Color::BLACK,
            5,
        );

        let updated_cards = cards
            .iter()
            .enumerate()
            .map(|(index, card)| {
                let mut card_copy = *card;

                card_copy.entity.x = x_coord;
                card_copy.entity.y = 15 * (index as i32 + 2);

                if index as i32 + 1 == offset {
                    card_copy.hidden = false;
                }

                card_copy
            })
            .collect();

        Tableau {
            entity,
            cards: updated_cards,
        }
    }

    pub fn draw(&self, handle: &mut RaylibDrawHandle) {
        self.entity.draw(handle);
        self.cards.iter().for_each(|card| card.draw(handle));
    }
}
