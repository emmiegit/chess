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
use chess::{Board, MoveGen};
use std::fmt::Display;
use std::io::{self, BufRead, Stdin};
use std::process;
use vampirc_uci::{parse_one, UciMessage};

#[derive(Debug)]
pub struct Game {
    pub board: Board,
    pub stockfish: Stockfish,
    input: Stdin,
    input_buffer: String,
}

impl Game {
    // Constructor
    pub fn new(config: &Configuration) -> Self {
        Game {
            board: Board::default(),
            stockfish: Stockfish::spawn(config.stockfish_nodes),
            input: io::stdin(),
            input_buffer: String::new(),
        }
    }

    // Communication
    fn receive(&mut self) -> UciMessage {
        self.input_buffer.clear();
        self.input
            .lock()
            .read_line(&mut self.input_buffer)
            .expect("Unable to read from stdin");

        parse_one(&self.input_buffer)
    }

    fn send<D: Display>(&mut self, command: D) {
        println!("{}", command);
    }

    // Methods

    /// Run only once to initialize the UCI connection.
    pub fn setup(&mut self) {
        match self.receive() {
            UciMessage::Uci => self.send(UciMessage::UciOk),
            UciMessage::Quit => process::exit(0),
            other => {
                eprintln!("Unexpected UCI message on setup: {:#?}", other);
                process::exit(0);
            }
        }
    }

    #[inline]
    pub fn moves(&self) -> MoveGen {
        MoveGen::new_legal(&self.board)
    }

    pub fn make_move(&mut self, engine: &dyn Engine) {
        let chosen_move = engine.choose_move(self);
        self.board = self.board.make_move_new(chosen_move);
    }
}
