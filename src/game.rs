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
use std::fmt::{self, Debug, Display};
use std::io::{self, BufRead, Stdin};
use vampirc_uci::{parse_one, UciMessage};

macro_rules! recv_inner {
    ($self:expr, $buffer:expr $(,)?) => {
        $self
            .input
            .lock()
            .read_line($buffer)
            .expect("Unable to read from stdin")
    };
}

pub struct Game {
    engine: Box<dyn Engine>,
    input: Stdin,
    buffer: String,
}

impl Game {
    pub fn new(config: &Configuration) -> Self {
        Game {
            engine: config.engine_kind.build(),
            input: io::stdin(),
            buffer: String::new(),
        }
    }

    pub fn recv_raw(&mut self) -> String {
        let mut buffer = String::new();
        recv_inner!(self, &mut buffer);
        buffer
    }

    pub fn recv(&mut self) -> UciMessage {
        self.buffer.clear();
        recv_inner!(self, &mut self.buffer);
        parse_one(&self.buffer)
    }

    pub fn send<D: Display>(&mut self, command: D) {
        println!("{}", command);
    }

    pub fn process(&mut self, command: &UciMessage) {
        todo!()
    }
}

impl Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Game")
            .field("engine", &self.engine.kind())
            .field("input", &self.input)
            .field("buffer", &self.buffer)
            .finish()
    }
}
