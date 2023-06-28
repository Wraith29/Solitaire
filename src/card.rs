use raylib::{
    prelude::{Color, RaylibDraw, RaylibDrawHandle},
    RaylibHandle,
};

use crate::{constants::FONT_SIZE, entity::Entity, number::Number, suit::Suit};

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub entity: Entity,
    pub number: Number,
    pub suit: Suit,
    pub hidden: bool,
}

impl Card {
    pub fn name(&self) -> String {
        let mut name = String::new();
        name.push_str(&self.suit.to_string());
        name.push_str(&self.number.to_string());
        name
    }

    pub fn draw(&self, handle: &mut RaylibDrawHandle) {
        if !self.hidden {
            self.entity.draw(handle);
            handle.draw_text(
                &self.name(),
                self.entity.x + 7,
                self.entity.y + 7,
                FONT_SIZE,
                self.suit.colour(),
            );
        } else {
            handle.draw_rectangle(
                self.entity.x,
                self.entity.y,
                self.entity.width,
                self.entity.height,
                Color::BLACK,
            );
        }
    }

    pub fn can_stack(&self, other: &Card) -> bool {
        if self.suit.is_red() && other.suit.is_red() {
            false
        } else if self.suit.is_black() && other.suit.is_black() {
            return false;
        } else {
            if other.number.next() == self.number {
                println!("Hello");
                return true;
            }
            return false;
        }
    }

    pub fn on_drag(&self, _window: &RaylibHandle) {
        if self.hidden {
        }
    }

    pub fn on_click(&mut self, window: &RaylibHandle) {
        if self.hidden {
            return;
        }

        let mouse_pos = window.get_mouse_position();

        if self.entity.contains(mouse_pos) {
            self.entity.x = mouse_pos.x as i32 - self.entity.width / 2;
            self.entity.y = mouse_pos.y as i32 - self.entity.height / 2;
        }
    }
}
