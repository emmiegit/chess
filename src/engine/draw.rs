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
        todo!()
    }
}
