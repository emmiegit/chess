/*
 * main.rs
 *
 * mallard-chess - Chess engine wrapper utility for fun
 * Copyright (C) 2022 Ammon Smith
 *
 * mallard-chess is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 */

mod config;
mod mode;
mod protocol;
mod stockfish;

use self::config::Configuration;

fn main() {
    let config = Configuration::load();

    todo!();
}
