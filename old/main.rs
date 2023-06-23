// #![allow(unused_variables)]
use game::Game;
use raylib::prelude::{Color, MouseButton, RaylibDraw};
use window::{WINDOW_HEIGHT, WINDOW_WIDTH};

mod card;
mod deck;
mod dimensions;
mod foundation;
mod game;
mod number;
mod stack;
mod suit;
mod tableau;
mod window;

fn main() {
    let (mut window, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Solitaire")
        .build();
    let game = Game::new();

    while !window.window_should_close() {
        let mut handle = window.begin_drawing(&thread);
        handle.clear_background(Color::WHITE);

        game.draw(&mut handle);

        if handle.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            game.on_click(&mut handle);
        }
    }
}
