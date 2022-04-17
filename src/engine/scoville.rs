/*
 * engine/scoville.rs
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
use super::{RandomEngine, StockfishEngine};
use rand::prelude::*;

#[derive(Debug)]
pub struct ScovilleEngine(f32);

impl ScovilleEngine {
    pub fn new(percent: f32) -> Self {
        assert!(
            percent.is_normal(),
            "Percentage for Scoville engine not a normal number",
        );
        assert!(
            percent >= 0.0 && percent <= 100.0,
            "Percentage for Scoville engine not between 0-100%",
        );

        ScovilleEngine(percent / 100.0)
    }
}

impl Engine for ScovilleEngine {
    fn kind(&self) -> EngineKind {
        EngineKind::Scoville
    }

    fn name(&self) -> &'static str {
        "Scoville"
    }

    fn description(&self) -> &'static str {
        "Plays Stockfish X% of the time, with remaining turns diluted with random moves"
    }

    fn choose_move(&self, game: &mut Game) -> ChessMove {
        let mut rng = thread_rng();
        if rng.gen::<f32>() < self.0 {
            StockfishEngine.choose_move(game)
        } else {
            RandomEngine.choose_move(game)
        }
    }
}
