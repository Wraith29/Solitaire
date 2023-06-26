mod card;
mod constants;
mod deck;
mod entity;
mod foundation;
mod game;
mod number;
mod stock;
mod suit;
mod tableau;

use constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use game::Game;
use raylib::prelude::{Color, RaylibDraw, MouseButton};

fn main() {
    let (mut window, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Solitaire")
        .build();

    let mut game = Game::new();

    while !window.window_should_close() {
        if window.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            game.on_click(&window);
        }

        if window.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            game.on_drag(&window);
        }

        let mut handle = window.begin_drawing(&thread);
        handle.clear_background(Color::WHITE);

        game.draw(&mut handle);
    }
}
