use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle};

use crate::{
    dimensions::Dimensions,
    suit::Suit,
    window::{CARD_HEIGHT, CARD_WIDTH, FONT_SIZE},
};

pub struct Foundation {
    suit: Suit,
    dimensions: Dimensions,
}

impl Foundation {
    pub fn new(suit: Suit, dimensions: Dimensions) -> Foundation {
        Foundation { suit, dimensions }
    }

    pub fn draw(&self, handle: &mut RaylibDrawHandle) {
        // Border
        handle.draw_rectangle(
            self.dimensions.x,
            self.dimensions.y,
            CARD_WIDTH,
            CARD_HEIGHT,
            Color::BLACK,
        );

        // Fill
        handle.draw_rectangle(
            self.dimensions.x + 5,
            self.dimensions.y + 5,
            CARD_WIDTH - 10,
            CARD_HEIGHT - 10,
            Color::WHITE,
        );

        handle.draw_text(
            &self.suit.to_string(),
            self.dimensions.x + 7,
            self.dimensions.y + 7,
            FONT_SIZE,
            self.suit.get_colour(),
        )
    }
}
