/*
 * engine/passthrough.rs
 *
 * mallard-chess - Chess engine wrapper utility for fun
 * Copyright (C) 2022 Ammon Smith
 *
 * mallard-chess is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 */

use super::prelude::*;
use crate::stockfish::Stockfish;

#[derive(Debug)]
pub struct StockfishEngine {
    stockfish: Stockfish,
}

impl StockfishEngine {
    pub fn new() -> Self {
        StockfishEngine {
            stockfish: Stockfish::spawn(),
        }
    }
}

impl Engine for StockfishEngine {
    fn kind(&self) -> EngineKind {
        EngineKind::Stockfish
    }

    fn name(&self) -> &'static str {
        "Stockfish (pass-through)"
    }

    fn description(&self) -> &'static str {
        "Dummy pass-through engine. Simply pipes Stockfish's moves as-is."
    }

    fn reset(&mut self) {
        self.stockfish.reset();
    }
}
