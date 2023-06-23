use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle};

use crate::{
    card::Card,
    dimensions::Dimensions,
    window::{CARD_HEIGHT, CARD_WIDTH},
};

pub struct Stack {
    pub cards: Vec<Card>,
    pub dimensions: Dimensions,
}

impl Stack {
    pub fn new(cards: Vec<Card>, dimensions: Dimensions) -> Stack {
        Stack { cards, dimensions }
    }

    pub fn draw(&self, handle: &mut RaylibDrawHandle) {
        // Draw Border
        handle.draw_rectangle(
            self.dimensions.x,
            self.dimensions.y,
            self.dimensions.w.unwrap_or(CARD_WIDTH + 10),
            self.dimensions.h.unwrap_or(CARD_HEIGHT + 10),
            Color::BLACK,
        );

        // Draw Inner
        handle.draw_rectangle(
            self.dimensions.x + 5,
            self.dimensions.y + 5,
            self.dimensions.w.unwrap_or(CARD_WIDTH),
            self.dimensions.h.unwrap_or(CARD_HEIGHT),
            Color::WHITE,
        );
    }
}
