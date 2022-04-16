/*
 * mode/mod.rs
 *
 * mallard-chess - Chess engine wrapper utility for fun
 * Copyright (C) 2022 Ammon Smith
 *
 * mallard-chess is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 */

use std::convert::TryFrom;

#[derive(Debug, Copy, Clone)]
pub enum GameMode {
    Stockfish,
    Worstfish,
}

impl<'a> TryFrom<&'a str> for GameMode {
    type Error = &'a str;

    fn try_from(name: &'a str) -> Result<GameMode, &'a str> {
        const VALUES: [(&str, GameMode); 3] = [
            ("boring", GameMode::Stockfish),
            ("stockfish", GameMode::Stockfish),
            ("worstfish", GameMode::Worstfish),
        ];

        for (value, mode) in VALUES {
            if name.eq_ignore_ascii_case(value) {
                return Ok(mode);
            }
        }

        Err(name)
    }
}
