/*
 * stockfish.rs
 *
 * mallard-chess - Chess engine wrapper for fun
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
use chess::{Board, BoardStatus, MoveGen};
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::{self, Child, ChildStdin, ChildStdout, Command, Stdio};
use std::rc::Rc;
use std::thread;
use std::time::Duration;
use vampirc_uci::{parse_one, UciFen, UciInfoAttribute, UciMessage, UciSearchControl};

#[derive(Debug)]
pub struct Stockfish {
    process: Child,
    input: BufReader<ChildStdout>,
    output: ChildStdin,
    output_buffer: String,
    nodes_to_search: Option<u64>,
    log_file: Rc<File>,
}

impl Stockfish {
    // Constructor
    pub fn spawn(nodes_to_search: Option<u64>, log_file: Rc<File>) -> Self {
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
            log_file,
        }
    }

    // Communication
    fn receive(&mut self) -> UciMessage {
        self.output_buffer.clear();
        self.input
            .read_line(&mut self.output_buffer)
            .expect("Unable to read from stockfish");

        parse_one(&self.output_buffer)
    }

    fn send<D: Display>(&mut self, command: D) {
        writeln!(self.output, "{}", command).expect("Unable to write to stockfish");
        self.output.flush().expect("Unable to flush stockfish pipe");
    }

    // Methods

    /// Evaluates a board.
    ///
    /// It determines the score of a board by having Stockfish return
    /// its preferred move, as well as its value change based on it.
    pub fn evaluate_position(&mut self, board: &Board) -> ScoredMove {
        log!(
            self.log_file,
            "Asking Stockfish to evaluate position (hash {})",
            board.get_hash(),
        );

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
            match self.receive() {
                // Finished evaluating
                UciMessage::BestMove { best_move, .. } => {
                    log!(
                        self.log_file,
                        "Stockfish finished, found best move: {:?}",
                        best_move,
                    );

                    chess_move = best_move;
                    break;
                }

                // Record scores as we receive them
                // The last score before BestMove is the evaluation
                UciMessage::Info(attributes) => {
                    for attribute in &attributes {
                        log!(self.log_file, "Stockfish sent information: {:?}", attribute);

                        match attribute {
                            // Providing a material difference in centipawns
                            UciInfoAttribute::Score {
                                cp: Some(centipawns),
                                ..
                            } => score = Some(Score::Centipawns(*centipawns)),

                            // Found a mate in X moves
                            UciInfoAttribute::Score {
                                mate: Some(moves), ..
                            } => score = Some(Score::from_mate(*moves)),

                            // Ignore other info lines
                            _ => (),
                        }
                    }

                    // Copy info message to UI
                    self.send(UciMessage::Info(attributes));
                }

                // Terminal messages
                UciMessage::Quit => process::exit(0),

                // Ignore unknown or unexpected messages
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

    /// Evaluates all possible moves from the board position.
    ///
    /// Using `evaluate_position()`, it sees the Stockfish improvement
    /// score for each legal move in this position, and then returns
    /// all the moves and their calculated scores in a list.
    pub fn evaluate_possible_moves_unsorted(&mut self, board: &Board) -> Vec<ScoredMove> {
        log!(
            self.log_file,
            "Asking Stockfish to evaluate all possible moves for board (hash {})",
            board.get_hash(),
        );

        let mut possible_board = Board::default();

        MoveGen::new_legal(board)
            .map(|chess_move| {
                board.make_move(chess_move, &mut possible_board);
                let score = match possible_board.status() {
                    BoardStatus::Ongoing => {
                        // We are attempting to score / recommend speculative moves,
                        // but evaluate_position() checks moves beyond that, that is,
                        // for the opposite player. So we need to ensure we store the
                        // possible move at the iterator level, not the one from
                        // evaluate_position().
                        //
                        // Similarly, we negate the score from this position because
                        // it was calculated from the opponent's perspective.
                        -self.evaluate_position(&possible_board).score
                    }

                    // Game is finished, return immediate score
                    BoardStatus::Checkmate => Score::OurMate(0),
                    BoardStatus::Stalemate => Score::Stalemate(0),
                };

                ScoredMove { chess_move, score }
            })
            .collect()
    }

    /// Evaluates all possible moves from the board position, sorted by increasing score value.
    ///
    /// See `evaluate_possible_moves_unsorted()`.
    pub fn evaluate_possible_moves(&mut self, board: &Board) -> Vec<ScoredMove> {
        let mut scored_moves = self.evaluate_possible_moves_unsorted(board);
        scored_moves.sort_by_key(|scored_move| scored_move.score);
        scored_moves
    }
}

impl Drop for Stockfish {
    fn drop(&mut self) {
        // Tell stockfish to gracefully quit
        self.send(UciMessage::Quit);

        // Check if it's exited after a bit
        thread::sleep(Duration::from_millis(10));
        match self.process.try_wait() {
            Ok(Some(status)) if status.success() => {
                log!(self.log_file, "Stockfish exited successfully");
            }
            Ok(Some(_)) => log!(self.log_file, "Stockfish exited with errors"),
            Err(error) => log!(self.log_file, "Stockfish has an unknown status: {}", error),
            Ok(None) => {
                log!(self.log_file, "Stockfish has not yet exited, killing");

                // We don't care if this succeeds or not, just send the signal.
                // We're done with it and are trying to clean up.
                let _ = self.process.kill();
            }
        }
    }
}
