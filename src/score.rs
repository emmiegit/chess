/*
 * score.rs
 *
 * mallard-chess - Chess engine wrapper for fun
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
use std::ops::Neg;

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

    /// The number of moves until a drawn game.
    Stalemate(u8),
}

impl Score {
    /// The value returned from the server is positive for our mate and negative for their mate.
    pub fn from_mate(value: i8) -> Self {
        let uvalue = value.unsigned_abs();
        match value.signum() {
            1 => Score::OurMate(uvalue),
            -1 => Score::TheirMate(uvalue),

            // 0 means mate this turn. Since we're evaluating possible moves for "us",
            //   this means we lose
            0 => Score::TheirMate(0),

            // Only returns -1, 0, +1
            _ => unreachable!("Invalid signum value"),
        }
    }
}

impl Neg for Score {
    type Output = Self;

    /// Negates this score.
    ///
    /// That is, interpret this score from the opposite player's perspective.
    /// So any gains become losses, our checkmates become their checkmates, etc.
    fn neg(self) -> Self {
        match self {
            Score::Centipawns(value) => Score::Centipawns(-value),
            Score::OurMate(moves) => Score::TheirMate(moves),
            Score::TheirMate(moves) => Score::OurMate(moves),

            // Stalemates are the same for both players, so keep the same
            Score::Stalemate(moves) => Score::Stalemate(moves),
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

            // We prefer to win rather than draw.
            (Score::OurMate(_), Score::Stalemate(_)) => Ordering::Greater,
            (Score::Stalemate(_), Score::OurMate(_)) => Ordering::Less,

            // Similarly, we prefer to draw rather than lose.
            (Score::TheirMate(_), Score::Stalemate(_)) => Ordering::Less,
            (Score::Stalemate(_), Score::TheirMate(_)) => Ordering::Greater,

            // Any change in material is better than a draw.
            (Score::Centipawns(_), Score::Stalemate(_)) => Ordering::Greater,
            (Score::Stalemate(_), Score::Centipawns(_)) => Ordering::Less,

            // If they're of the same type, then compare appropriately.
            //
            // For centipawns, just see which is larger.
            // For mates, we want smaller numbers of moves for our mates,
            // and greater number of moves for the opponent.
            (Score::Centipawns(x), Score::Centipawns(y)) => x.cmp(y),
            (Score::OurMate(x), Score::OurMate(y)) => y.cmp(x),
            (Score::TheirMate(x), Score::TheirMate(y)) => x.cmp(y),
            (Score::Stalemate(x), Score::Stalemate(y)) => x.cmp(y),
        }
    }
}

#[test]
fn sorting() {
    // Actual
    let mut scores = vec![
        Score::Stalemate(5),
        Score::OurMate(0),
        Score::Stalemate(2),
        Score::Centipawns(500),
        Score::Stalemate(0),
        Score::Centipawns(-100),
        Score::Centipawns(50),
        Score::OurMate(5),
        Score::Centipawns(-30),
        Score::OurMate(2),
        Score::TheirMate(2),
        Score::Centipawns(200),
        Score::TheirMate(0),
        Score::TheirMate(5),
    ];
    scores.sort();

    // Expected
    let sorted = vec![
        Score::TheirMate(0),
        Score::TheirMate(2),
        Score::TheirMate(5),
        Score::Stalemate(0),
        Score::Stalemate(2),
        Score::Stalemate(5),
        Score::Centipawns(-100),
        Score::Centipawns(-30),
        Score::Centipawns(50),
        Score::Centipawns(200),
        Score::Centipawns(500),
        Score::OurMate(5),
        Score::OurMate(2),
        Score::OurMate(0),
    ];

    assert_eq!(scores, sorted, "Sorted score list did not match expected");
}
