/*
 * engine/mod.rs
 *
 * mallard-chess - Chess engine wrapper for fun
 * Copyright (C) 2022 Ammon Smith
 *
 * mallard-chess is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 */

mod prelude {
    pub use super::{Engine, EngineKind};
    pub use crate::game::Game;
    pub use chess::{ChessMove, Color};
    pub use std::io::Write;
}

mod draw;
mod mediocre;
mod pacifist;
mod random;
mod scoville;
mod stockfish;
mod worstfish;

pub use self::draw::DrawfishEngine;
pub use self::mediocre::MediocrefishEngine;
pub use self::pacifist::PacifistEngine;
pub use self::random::RandomEngine;
pub use self::scoville::ScovilleEngine;
pub use self::stockfish::StockfishEngine;
pub use self::worstfish::WorstfishEngine;

use self::prelude::*;
use crate::config::Configuration;
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

    /// Given this game, determine what move the engine would like to play.
    fn choose_move(&self, game: &mut Game) -> ChessMove;
}

#[derive(EnumIter, Debug, Copy, Clone, PartialEq, Eq)]
pub enum EngineKind {
    Random,
    Pacifist,
    Stockfish,
    Mediocrefish,
    Drawfish,
    Worstfish,
    Scoville,
}

impl EngineKind {
    pub fn print_variants() {
        eprintln!("Possible values:");

        for variant in EngineKind::iter() {
            eprintln!("- {:?}", variant);
        }
    }

    pub fn build(self, config: &Configuration) -> Box<dyn Engine> {
        match self {
            EngineKind::Random => Box::new(RandomEngine),
            EngineKind::Pacifist => Box::new(PacifistEngine),
            EngineKind::Stockfish => Box::new(StockfishEngine),
            EngineKind::Mediocrefish => Box::new(MediocrefishEngine),
            EngineKind::Drawfish => Box::new(DrawfishEngine),
            EngineKind::Worstfish => Box::new(WorstfishEngine),
            EngineKind::Scoville => Box::new(ScovilleEngine::new(config.scoville_percent)),
        }
    }
}

impl<'a> TryFrom<&'a str> for EngineKind {
    type Error = &'a str;

    fn try_from(name: &'a str) -> Result<EngineKind, &'a str> {
        const VALUES: [(&str, EngineKind); 20] = [
            ("rand", EngineKind::Random),
            ("random", EngineKind::Random),
            ("pacifist", EngineKind::Pacifist),
            ("pacifism", EngineKind::Pacifist),
            ("hippie", EngineKind::Pacifist),
            ("boring", EngineKind::Stockfish),
            ("dummy", EngineKind::Stockfish),
            ("passthrough", EngineKind::Stockfish),
            ("stockfish", EngineKind::Stockfish),
            ("meh", EngineKind::Mediocrefish),
            ("mediocre", EngineKind::Mediocrefish),
            ("mediocrefish", EngineKind::Mediocrefish),
            ("draw", EngineKind::Drawfish),
            ("drawfish", EngineKind::Drawfish),
            ("stale", EngineKind::Drawfish),
            ("stalemate", EngineKind::Drawfish),
            ("worst", EngineKind::Worstfish),
            ("worstfish", EngineKind::Worstfish),
            ("scoville", EngineKind::Scoville),
            ("mix", EngineKind::Scoville),
        ];

        for (value, mode) in VALUES {
            if name.eq_ignore_ascii_case(value) {
                return Ok(mode);
            }
        }

        Err(name)
    }
}
