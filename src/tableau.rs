use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle};

use crate::{
    card::Card,
    dimensions::Dimensions,
    window::{CARD_HEIGHT, TABLEAU_WIDTH, WINDOW_WIDTH},
};

pub struct Tableau {
    dimensions: Dimensions,
    cards: Vec<Card>,
    offset: i32,
}

impl Tableau {
    pub fn new(offset: i32) -> Tableau {
        // Offset somewhere between 0 .. 6 (1-7)
        // Needs to calculate position on screen based off of that
        let col_count = 7;

        // Width of tableaus total
        let min_width = ((TABLEAU_WIDTH + 15) * col_count) + 30;
        let left_offset = (WINDOW_WIDTH - min_width) / 2;
        let card_offset = left_offset + ((TABLEAU_WIDTH + 15) * (offset - 1)) + 15;

        let dimensions = Dimensions::new(card_offset, 0, Some(TABLEAU_WIDTH), None);
        Tableau {
            cards: vec![],
            dimensions,
            offset,
        }
    }

    pub fn push(&mut self, card: Card) {
        let mut card_copy = card.clone();
        card_copy.dimensions.x = self.dimensions.x + 15;
        card_copy.dimensions.y = 20 + (20 * self.cards.len() as i32);

        if (self.cards.len() + 1) as i32 == self.offset {
            card_copy.flipped = false;
        }

        self.cards.push(card_copy);
    }

    pub fn draw(&self, handle: &mut RaylibDrawHandle) {
        // + 40 for 20 offset either wat
        let height = CARD_HEIGHT + 40 + (self.cards.len() * 20) as i32;

        handle.draw_rectangle(
            self.dimensions.x,
            self.dimensions.y,
            TABLEAU_WIDTH,
            height,
            Color::BLACK,
        );
        handle.draw_rectangle(
            self.dimensions.x + 5,
            self.dimensions.y + 5,
            TABLEAU_WIDTH - 10,
            height - 10,
            Color::WHITE,
        );

        self.cards.iter().for_each(|card| card.draw(handle));
    }
}
