/*
 * game.rs
 *
 * mallard-chess - Chess engine wrapper utility for fun
 * Copyright (C) 2022 Ammon Smith
 *
 * mallard-chess is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 */

use crate::config::Configuration;
use crate::engine::Engine;
use crate::stockfish::Stockfish;
use std::fmt::{self, Debug};
use std::io::{self, BufRead, Stdin};

pub struct Game {
    engine: Box<dyn Engine>,
    stockfish: Stockfish,
    input: Stdin,
}

impl Game {
    pub fn new(config: &Configuration) -> Self {
        Game {
            engine: config.engine_kind.build(),
            stockfish: Stockfish::spawn(),
            input: io::stdin(),
        }
    }

    pub fn main_loop(&mut self) {
        todo!()
    }

    fn read_raw(&mut self) -> String {
        let mut buffer = String::new();
        self.input
            .lock()
            .read_line(&mut buffer)
            .expect("Unable to read from stdin");

        buffer
    }

    fn write_raw(&mut self, command: &str) {
        println!("{}", command);
    }
}

impl Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Game")
            .field("engine", &self.engine.kind())
            .field("stockfish", &self.stockfish)
            .field("input", &self.input)
            .finish()
    }
}
