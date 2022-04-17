/*
 * score.rs
 *
 * mallard-chess - Chess engine wrapper utility for fun
 * Copyright (C) 2022 Ammon Smith
 *
 * mallard-chess is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 */

//! Representation of scores for given boards.

use chess::ChessMove;
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ScoredMove {
    pub chess_move: ChessMove,
    pub score: Score,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Score {
    /// The score of this move in centipawns.
    ///
    /// It is from the engine's perspective, with negative
    /// values meaning that the engine is losing.
    Centipawns(i32),

    /// The number of moves until the opponent is checkmated.
    OurMate(u8),

    /// The number of moves until the engine is checkmated.
    TheirMate(u8),
}

impl Score {
    /// The value returned from the server is positive for our mate and negative for their mate.
    pub fn from_mate(value: i8) -> Self {
        let uvalue = value.unsigned_abs();
        match value.signum() {
            1 => Score::OurMate(uvalue),
            -1 => Score::TheirMate(uvalue),
            _ => panic!("Invalid signum value for from_mate(): {}", value),
        }
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Score {
    /// Compares two scores, and determines which is higher.
    ///
    /// That is, scores which are more advantageous for the player have a greater value.
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // Anything involving a mate is automatically better than changes in value.
            (Score::OurMate(_), Score::Centipawns(_)) => Ordering::Greater,
            (Score::Centipawns(_), Score::OurMate(_)) => Ordering::Less,

            // Anything involving us being mated is automatically worse.
            (Score::TheirMate(_), Score::Centipawns(_)) => Ordering::Less,
            (Score::Centipawns(_), Score::TheirMate(_)) => Ordering::Greater,

            // If one move results in us mating, and a different results in the opponent mating,
            // obviously the one with our victory is better.
            (Score::OurMate(_), Score::TheirMate(_)) => Ordering::Greater,
            (Score::TheirMate(_), Score::OurMate(_)) => Ordering::Less,

            // If they're of the same type, then compare appropriately.
            //
            // For centipawns, just see which is larger.
            // For mates, we want smaller numbers of moves for our mates,
            // and greater number of moves for the opponent.
            (Score::Centipawns(x), Score::Centipawns(y)) => x.cmp(y),
            (Score::OurMate(x), Score::OurMate(y)) => y.cmp(x),
            (Score::TheirMate(x), Score::TheirMate(y)) => x.cmp(y),
        }
    }
}
