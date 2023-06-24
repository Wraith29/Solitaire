use raylib::prelude::{Color, RaylibDrawHandle};

use crate::{
    card::Card,
    constants::{CARD_HEIGHT, CARD_WIDTH, TABLEAU_WIDTH, WINDOW_WIDTH},
    entity::Entity,
};

pub struct Tableau {
    entity: Entity,
    cards: Vec<Card>,
    offset: i32,
}

impl Tableau {
    pub fn new(offset: i32, cards: Vec<Card>) -> Tableau {
        /*
        7 Tableaus
        Tableau Width: CARD_WIDTH
        Tableau Margin: 15

        | = Margin
        T = Tableau

        |T|T|T|T|T|T|T|

        total_tableau_width = 7 * TABLEAU_WIDTH + 8 * TABLEAU_MARGIN

        window_width - total_tableau_width
        */
        // 15 is the margin between tableaus
        let total_tableau_width = (TABLEAU_WIDTH * 7) + (15 * 8);
        let outer_margin_per_side = (WINDOW_WIDTH - total_tableau_width) / 2;

        // outer margin per side is the baseline x coord offset
        // the 15 is for the left hand margin of the card
        // the tableau_width + 15 is accounting for each tableau and it's margin
        // that comes before this one
        let x_coord = outer_margin_per_side + ((TABLEAU_WIDTH + 15) * offset);

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
                let mut card_copy = card.clone();

                card_copy.entity.x = x_coord;
                card_copy.entity.y = 15 * (index as i32 + 2);

                card_copy
            })
            .collect();

        Tableau {
            entity,
            offset,
            cards: updated_cards,
        }
    }

    pub fn draw(&self, handle: &mut RaylibDrawHandle) {
        self.entity.draw(handle);
        self.cards.iter().for_each(|card| card.draw(handle));
    }
}
