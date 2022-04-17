/*
 * engine/pacifist.rs
 *
 * mallard-chess - Chess engine wrapper for fun
 * Copyright (C) 2022 Ammon Smith
 *
 * mallard-chess is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 */

use super::prelude::*;
use chess::{Board, BoardStatus, MoveGen};
use std::io::Write;

#[derive(Debug)]
pub struct PacifistEngine;

impl Engine for PacifistEngine {
    fn kind(&self) -> EngineKind {
        EngineKind::Pacifist
    }

    fn name(&self) -> &'static str {
        "Pacifist"
    }

    fn description(&self) -> &'static str {
        "Simple algorithm that avoids checkmate, check, and capture."
    }

    fn choose_move(&self, game: &mut Game) -> ChessMove {
        // Gather a list of all moves, with scores reflecting our priorities.
        //
        // From worst to best:
        // * Checkmate
        // * Capture
        // * Check
        // * Anything else

        // Gather the list of all moves
        let mut possible_board = Board::default();

        let mut moves = MoveGen::new_legal(&game.board)
            .map(|chess_move| {
                game.board.make_move(chess_move, &mut possible_board);

                let score = match possible_board.status() {
                    BoardStatus::Ongoing => score_move(&game.board, &possible_board),
                    BoardStatus::Checkmate => -10, // Doesn't like checkmates of any kind
                    BoardStatus::Stalemate => 10, // Prefers stalemates because nobody is "conquering" the other
                };

                (chess_move, score)
            })
            .collect::<Vec<_>>();

        // Sort moves by score
        //
        // This sorts from lowest to highest, so the most pacifist move
        // is the last item in this list.
        moves.sort_by_key(|&(_, score)| score);
        log!(game.log_file, "Scored possible moves for pacifism: {:?}", moves);

        // Extract chess move
        moves.last().expect("No legal moves").0
    }
}

fn score_move(current_board: &Board, proposed_board: &Board) -> i32 {
    let color = !current_board.side_to_move(); // Get color of opponent
    let current_pieces_count = current_board.color_combined(color).popcnt();
    let proposed_pieces_count = proposed_board.color_combined(color).popcnt();

    // See if the move results in a capture
    if proposed_pieces_count < current_pieces_count {
        return -5;
    }

    // See if the board results in check
    if proposed_board.checkers().popcnt() > 0 {
        return -1;
    }

    // Default score
    0
}
