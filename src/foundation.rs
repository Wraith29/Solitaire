use raylib::prelude::{RaylibDraw, RaylibDrawHandle};

use crate::{constants::FONT_SIZE, entity::Entity, suit::Suit};

pub struct Foundation {
    suit: Suit,
    entity: Entity,
}

impl Foundation {
    pub fn new(suit: Suit, entity: Entity) -> Foundation {
        Foundation { suit, entity }
    }

    pub fn draw(&self, handle: &mut RaylibDrawHandle) {
        self.entity.draw(handle);

        handle.draw_text(
            &self.suit.to_string(),
            self.entity.x + 7,
            self.entity.y + 7,
            FONT_SIZE,
            self.suit.colour(),
        );
    }
}
