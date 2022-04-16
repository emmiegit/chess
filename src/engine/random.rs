/*
 * engine/random.rs
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
use chess::{Board, MoveGen};

#[derive(Debug, Default)]
pub struct RandomEngine {
    board: Board,
}

impl Engine for RandomEngine {
    fn kind(&self) -> EngineKind {
        EngineKind::Random
    }

    fn name(&self) -> &'static str {
        "Random"
    }

    fn description(&self) -> &'static str {
        "Chooses a random valid move"
    }

    fn reset(&mut self) {
        self.board = Board::default();
    }
}
