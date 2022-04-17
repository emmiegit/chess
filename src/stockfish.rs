/*
 * stockfish.rs
 *
 * mallard-chess - Chess engine wrapper utility for fun
 * Copyright (C) 2022 Ammon Smith
 *
 * mallard-chess is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 */

//! Module for communicating with Stockfish.
//!
//! Stockfish is a very solid chess engine which we are
//! using for our various "modes" of chess engine operation.
//!
//! This application is essentially "piping through" what
//! Stockfish determines, with modifications depending on the mode.

use crate::score::{Score, ScoredMove};
use chess::{Board, MoveGen};
use std::fmt::Display;
use std::io::{BufRead, BufReader, Write};
use std::process::{self, Child, ChildStdin, ChildStdout, Command, Stdio};
use vampirc_uci::{parse_one, UciFen, UciInfoAttribute, UciMessage, UciSearchControl};

macro_rules! recv_inner {
    ($self:expr, $buffer:expr $(,)?) => {
        $self
            .input
            .read_line($buffer)
            .expect("Unable to read from stockfish")
    };
}

#[derive(Debug)]
pub struct Stockfish {
    process: Child,
    input: BufReader<ChildStdout>,
    output: ChildStdin,
    output_buffer: String,
    nodes_to_search: Option<u64>,
}

impl Stockfish {
    // Constructor
    pub fn spawn(nodes_to_search: Option<u64>) -> Self {
        let mut process = Command::new("stockfish")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("Unable to start stockfish");

        let stdin = process.stdin.take().expect("No stdin opened");
        let stdout = process.stdout.take().expect("No stdout opened");

        Stockfish {
            process,
            input: BufReader::new(stdout),
            output: stdin,
            output_buffer: String::new(),
            nodes_to_search,
        }
    }

    // Communication
    pub fn recv_raw(&mut self) -> String {
        let mut buffer = String::new();
        recv_inner!(self, &mut buffer);
        buffer
    }

    pub fn recv(&mut self) -> UciMessage {
        self.output_buffer.clear();
        recv_inner!(self, &mut self.output_buffer);
        parse_one(&self.output_buffer)
    }

    pub fn send<D: Display>(&mut self, command: D) {
        writeln!(self.output, "{}", command).expect("Unable to write to stockfish");
        self.output.flush().expect("Unable to flush stockfish pipe");
    }

    // Methods
    pub fn evaluate_position(&mut self, board: &Board) -> ScoredMove {
        self.send(UciMessage::Position {
            startpos: false,
            fen: Some(UciFen(board.to_string())),
            moves: Vec::new(),
        });

        self.send(UciMessage::Go {
            time_control: None,
            search_control: Some(UciSearchControl {
                search_moves: Vec::new(),
                mate: None,
                depth: None,
                nodes: self.nodes_to_search,
            }),
        });

        let chess_move;
        let mut score = None;

        loop {
            let message = self.recv();
            match message {
                // Finished evaluating
                UciMessage::BestMove { best_move, .. } => {
                    chess_move = best_move.into();
                    break;
                }

                // Record scores as we receive them
                // The last score before BestMove is the evaluation
                UciMessage::Info(attributes) => {
                    for attribute in attributes {
                        match attribute {
                            // Providing a material difference in centipawns
                            UciInfoAttribute::Score {
                                cp: Some(centipawns),
                                ..
                            } => score = Some(Score::Centipawns(centipawns)),

                            // Found a mate in X moves
                            UciInfoAttribute::Score {
                                mate: Some(moves), ..
                            } => score = Some(Score::from_mate(moves)),

                            // Ignore other info lines
                            _ => (),
                        }
                    }
                }

                // Terminal
                UciMessage::Quit => process::exit(0),

                // Ignore other messages
                _ => (),
            }
        }

        // If no score value has been set, then Stockfish isn't behaving properly.
        let score =
            score.expect("Stockfish didn't return score information before deciding a move");

        // Return result
        //
        // This is the best move it found, and the score of this move,
        // which rates our current position.
        ScoredMove { chess_move, score }
    }

    pub fn evaluate_possible_moves(&mut self, board: &Board) -> () {
        todo!()
        // XXX MoveGen::new_legal(board)
    }
}
