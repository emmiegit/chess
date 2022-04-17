/*
 * engine/mediocrefish.rs
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

#[derive(Debug)]
pub struct MediocrefishEngine;

impl Engine for MediocrefishEngine {
    fn kind(&self) -> EngineKind {
        EngineKind::Mediocrefish
    }

    fn name(&self) -> &'static str {
        "Mediocrefish"
    }

    fn description(&self) -> &'static str {
        "Chooses the median of moves as ordered by score"
    }

    fn choose_move(&self, game: &mut Game) -> ChessMove {
        // Choose the move in the middle, sorted by score.
        let moves = game.stockfish.evaluate_possible_moves(&game.board);
        moves[moves.len() / 2].chess_move
    }
}
