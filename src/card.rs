use raylib::prelude::{RaylibDraw, RaylibDrawHandle};

use crate::{constants::FONT_SIZE, entity::Entity, number::Number, suit::Suit};

#[derive(Clone, Copy)]
pub struct Card {
    pub entity: Entity,
    pub number: Number,
    pub suit: Suit,
}

impl Card {
    pub fn name(&self) -> String {
        let mut name = String::new();
        name.push_str(&self.suit.to_string());
        name.push_str(&self.number.to_string());
        return name;
    }

    pub fn draw(&self, handle: &mut RaylibDrawHandle) {
        self.entity.draw(handle);

        handle.draw_text(
            &self.name(),
            self.entity.x + 7,
            self.entity.y + 7,
            FONT_SIZE,
            self.suit.colour(),
        );
    }
}
