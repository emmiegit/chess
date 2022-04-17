/*
 * engine/worstfish.rs
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
pub struct WorstfishEngine;

impl Engine for WorstfishEngine {
    fn kind(&self) -> EngineKind {
        EngineKind::Worstfish
    }

    fn name(&self) -> &'static str {
        "Worstfish"
    }

    fn description(&self) -> &'static str {
        "Chooses the move Stockfish dislikes the most."
    }

    fn choose_move(&self, game: &mut Game, player: Color) -> ChessMove {
        todo!()
    }
}
