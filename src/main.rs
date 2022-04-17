/*
 * main.rs
 *
 * mallard-chess - Chess engine wrapper utility for fun
 * Copyright (C) 2022 Ammon Smith
 *
 * mallard-chess is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 */

mod config;
mod engine;
mod game;
mod score;
mod stockfish;

use self::config::Configuration;
use self::game::Game;

fn main() {
    let config = Configuration::load();
    let mut game = Game::new(&config);
    game.setup();

    todo!();
}
