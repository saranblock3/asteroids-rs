mod ship;
mod direction;
mod game;
mod point;
mod command;

use crate::game::Game;
use std::io::stdout;

// test

fn main() {
    Game::new(stdout(), 10, 10).run();
}
