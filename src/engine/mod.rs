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
    pub use super::Engine;
}

mod passthrough;
mod worstfish;

pub use self::passthrough::StockfishEngine;
pub use self::worstfish::WorstfishEngine;

use std::convert::TryFrom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub trait Engine {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
}

#[derive(EnumIter, Debug, Copy, Clone)]
pub enum EngineChoice {
    Stockfish,
    Worstfish,
}

impl EngineChoice {
    pub fn print_variants() {
        eprintln!("Possible values:");

        for variant in EngineChoice::iter() {
            eprintln!("- {:?}", variant);
        }
    }
}

impl<'a> TryFrom<&'a str> for EngineChoice {
    type Error = &'a str;

    fn try_from(name: &'a str) -> Result<EngineChoice, &'a str> {
        const VALUES: [(&str, EngineChoice); 5] = [
            ("boring", EngineChoice::Stockfish),
            ("dummy", EngineChoice::Stockfish),
            ("passthrough", EngineChoice::Stockfish),
            ("stockfish", EngineChoice::Stockfish),
            ("worstfish", EngineChoice::Worstfish),
        ];

        for (value, mode) in VALUES {
            if name.eq_ignore_ascii_case(value) {
                return Ok(mode);
            }
        }

        Err(name)
    }
}
