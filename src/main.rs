use constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use game::Game;
use raylib::prelude::{Color, RaylibDraw};

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

        handle.draw_line(WINDOW_WIDTH / 2, 0, WINDOW_WIDTH / 2, 900, Color::BLUE);
    }
}
