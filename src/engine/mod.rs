/*
 * engine/mod.rs
 *
 * mallard-chess - Chess engine wrapper utility for fun
 * Copyright (C) 2022 Ammon Smith
 *
 * mallard-chess is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 */

mod prelude {
    pub use super::{Engine, EngineKind};
}

mod passthrough;
mod random;
mod worstfish;

pub use self::passthrough::StockfishEngine;
pub use self::random::RandomEngine;
pub use self::worstfish::WorstfishEngine;

use std::convert::TryFrom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub trait Engine {
    /// Returns the `EngineKind` associated with the engine.
    fn kind(&self) -> EngineKind;

    /// Returns a constant string describing the name of the engine.
    fn name(&self) -> &'static str;

    /// Returns a short constant string describing the behavior of the engine.
    fn description(&self) -> &'static str;

    /// Resets the engine, preparing for a new game.
    /// The state should be identical to constructing a new instance of the engine.
    fn reset(&mut self);
}

#[derive(EnumIter, Debug, Copy, Clone, PartialEq, Eq)]
pub enum EngineKind {
    Random,
    Stockfish,
    Worstfish,
}

impl EngineKind {
    pub fn print_variants() {
        eprintln!("Possible values:");

        for variant in EngineKind::iter() {
            eprintln!("- {:?}", variant);
        }
    }

    pub fn build(self) -> Box<dyn Engine> {
        match self {
            EngineKind::Random => Box::new(RandomEngine::new()),
            EngineKind::Stockfish => Box::new(StockfishEngine::new()),
            EngineKind::Worstfish => Box::new(WorstfishEngine::new()),
        }
    }
}

impl<'a> TryFrom<&'a str> for EngineKind {
    type Error = &'a str;

    fn try_from(name: &'a str) -> Result<EngineKind, &'a str> {
        const VALUES: [(&str, EngineKind); 7] = [
            ("rand", EngineKind::Random),
            ("random", EngineKind::Random),
            ("boring", EngineKind::Stockfish),
            ("dummy", EngineKind::Stockfish),
            ("passthrough", EngineKind::Stockfish),
            ("stockfish", EngineKind::Stockfish),
            ("worstfish", EngineKind::Worstfish),
        ];

        for (value, mode) in VALUES {
            if name.eq_ignore_ascii_case(value) {
                return Ok(mode);
            }
        }

        Err(name)
    }
}
