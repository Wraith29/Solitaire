use raylib::prelude::{Color, RaylibDrawHandle};

use crate::{
    card::Card,
    constants::{CARD_HEIGHT, CARD_WIDTH, WINDOW_WIDTH},
    entity::Entity,
};

pub struct Tableau {
    entity: Entity,
    cards: Vec<Card>,
    offset: i32,
}

impl Tableau {
    pub fn new(offset: i32, cards: Vec<Card>) -> Tableau {
        // There should be a 400? pixel margin

        // this accounts for 1 border per tableau, the total size required will be
        // this * 7 + 15
        let individual_tableau_width = CARD_WIDTH + 15;
        let total_tableau_width = (individual_tableau_width * 7) + 15;
        let tableau_available_space = WINDOW_WIDTH;

        let left_margin = (tableau_available_space - total_tableau_width) / 2;
        let width_offset = CARD_WIDTH * offset;
        let margin_offset = 15 * (offset + 1);

        let x_pos = left_margin + width_offset + margin_offset;

        let height = CARD_HEIGHT + (30 * (offset + 1));
        let entity = Entity::new(x_pos, 15, CARD_WIDTH, height, Color::WHITE, Color::BLACK, 5);

        Tableau {
            entity,
            offset,
            cards,
        }
    }

    pub fn draw(&self, handle: &mut RaylibDrawHandle) {
        self.entity.draw(handle);
    }
}
