/*
 * engine/stockfish.rs
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

#[derive(Debug)]
pub struct StockfishEngine;

impl Engine for StockfishEngine {
    fn kind(&self) -> EngineKind {
        EngineKind::Stockfish
    }

    fn name(&self) -> &'static str {
        "Stockfish (regular)"
    }

    fn description(&self) -> &'static str {
        "Boring engine. Simply returns whatever Stockfish thinks is the best move."
    }

    fn choose_move(&self, game: &mut Game, player: Color) -> ChessMove {
        todo!()
    }
}
