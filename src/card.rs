use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle};

use crate::{
    dimensions::Dimensions,
    number::Number,
    suit::Suit,
    window::{CARD_HEIGHT, CARD_WIDTH, FONT_SIZE},
};

#[derive(Clone, Copy)]
pub struct Card {
    pub number: Number,
    pub dimensions: Dimensions,
    pub suit: Suit,
    pub flipped: bool,
}

impl Card {
    pub fn new(number: Number, suit: Suit, dimensions: Dimensions) -> Card {
        Card {
            number,
            suit,
            dimensions,
            flipped: true,
        }
    }

    fn get_text(&self) -> String {
        let mut res = String::new();

        res.push_str(&self.suit.to_string());
        res.push_str(&self.number.to_string());

        res
    }

    pub fn draw(&self, handle: &mut RaylibDrawHandle) {
        // Draw Border
        handle.draw_rectangle(
            self.dimensions.x,
            self.dimensions.y,
            CARD_WIDTH,
            CARD_HEIGHT,
            Color::BLACK,
        );

        // Draw Inner
        handle.draw_rectangle(
            self.dimensions.x + 5,
            self.dimensions.y + 5,
            CARD_WIDTH - 10,
            CARD_HEIGHT - 10,
            if self.flipped {
                Color::BLACK
            } else {
                Color::WHITE
            },
        );

        // Write Text
        if !self.flipped {
            handle.draw_text(
                &self.get_text(),
                self.dimensions.x + 7,
                self.dimensions.y + 7,
                FONT_SIZE,
                self.suit.get_colour(),
            );
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.suit == other.suit && self.number == other.number
    }
}
