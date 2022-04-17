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
use chess::{Board, ChessMove, MoveGen};
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
    input_buffer: String,
    pub stockfish: Stockfish,
    pub board: Board,
}

impl Game {
    // Constructor
    pub fn new(config: &Configuration) -> Self {
        Game {
            engine: config.engine_kind.build(),
            input: io::stdin(),
            input_buffer: String::new(),
            stockfish: Stockfish::spawn(config.stockfish_nodes),
            board: Board::default(),
        }
    }

    // Communication
    pub fn recv(&mut self) -> UciMessage {
        self.input_buffer.clear();
        recv_inner!(self, &mut self.input_buffer);
        parse_one(&self.input_buffer)
    }

    pub fn send<D: Display>(&mut self, command: D) {
        println!("{}", command);
    }

    // Methods
    #[inline]
    pub fn moves(&self) -> MoveGen {
        MoveGen::new_legal(&self.board)
    }

    pub fn reset(&mut self) {
        self.board = Board::default();
    }

    #[inline]
    pub fn make_move(&mut self, m: ChessMove) {
        self.board = self.board.make_move_new(m);
    }
}

impl Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Game")
            .field("engine", &self.engine.kind())
            .field("input", &self.input)
            .field("input_buffer", &self.input_buffer)
            .field("stockfish", &self.stockfish)
            .field("board", &self.board)
            .finish()
    }
}
