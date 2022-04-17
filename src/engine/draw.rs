/*
 * engine/draw.rs
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
use crate::score::{Score, ScoredMove};

#[derive(Debug)]
pub struct DrawfishEngine;

impl Engine for DrawfishEngine {
    fn kind(&self) -> EngineKind {
        EngineKind::Drawfish
    }

    fn name(&self) -> &'static str {
        "Drawfish"
    }

    fn description(&self) -> &'static str {
        "Attempts to force a stalemate"
    }

    fn choose_move(&self, game: &mut Game) -> ChessMove {
        let mut moves = game
            .stockfish
            .evaluate_possible_moves(&game.board)
            .into_iter()
            .map(|ScoredMove { chess_move, score }| {
                const BIG_VALUE: i32 = 1_000_000_000;

                // If a stalemate is possible, then take it.
                // If not, choose lowest-rated positive move.
                // Checkmates of any kind are strongly disincentivized.
                let new_score = match score {
                    // Invert material changes.
                    // If a gain is huge, we want to make it look like a bad choice.
                    // If a gain is minor, we want to make it look more favorable.
                    Score::Centipawns(value) if value > 0 => BIG_VALUE / value,

                    // For losses, keep the value as-is.
                    // We want to prefer minor losses in material to major ones.
                    Score::Centipawns(value) => value,

                    // For stalemates or checkmates, give a massive fixed score.
                    //
                    // We subtract by moves in stalemate since we slightly prefer
                    // moves that get us to stalemate faster.
                    Score::Stalemate(moves) => BIG_VALUE - i32::from(moves),
                    Score::OurMate(_) | Score::TheirMate(_) => -BIG_VALUE,
                };

                (chess_move, new_score)
            })
            .collect::<Vec<_>>();

        // Sort moves by recalculated score
        moves.sort_by_key(|&(_, score)| score);

        log!(
            game.log_file,
            "Scored possible moves for drawfish: {:?}",
            moves,
        );

        // Choose the best-scoring move.
        // Because it sorts from least to greatest, this should be the last one.
        moves.last().expect("No valid moves").0
    }
}
