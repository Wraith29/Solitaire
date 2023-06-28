use raylib::prelude::{RaylibDraw, RaylibDrawHandle};

use crate::{card::Card, constants::FONT_SIZE, entity::Entity, suit::Suit};

pub struct Foundation {
    suit: Suit,
    entity: Entity,
    pub cards: Vec<Card>,
}

impl Foundation {
    pub fn new(suit: Suit, entity: Entity) -> Foundation {
        Foundation {
            suit,
            entity,
            cards: vec![],
        }
    }

    pub fn add_card(&mut self, card: &mut Card) {
        card.entity.x = self.entity.x;
        card.entity.y = self.entity.y;

        self.cards.push(card.to_owned());

        self.cards.sort_by(|a, b| a.number.cmp(&b.number));
    }

    pub fn draw(&self, handle: &mut RaylibDrawHandle) {
        self.entity.draw(handle);

        if self.cards.len() >= 1 {
            // TODO: Remove Unwrap
            handle.draw_text(
                &self.cards.first().unwrap().name(),
                self.entity.x + 7,
                self.entity.y + 7,
                FONT_SIZE,
                self.suit.colour(),
            )
        } else {
            handle.draw_text(
                &self.suit.to_string(),
                self.entity.x + 7,
                self.entity.y + 7,
                FONT_SIZE,
                self.suit.colour(),
            );
        }
    }
}
