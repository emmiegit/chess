/*
 * protocol.rs
 *
 * mallard-chess - Chess engine wrapper utility for fun
 * Copyright (C) 2022 Ammon Smith
 *
 * mallard-chess is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 */

//! Module for communicating using the Chess Engine Communication Protocol.
//! Needed for talking to xboard and other such programs.
//!
//! It uses stdin for receiving data and stdout for sending data, so all
//! "logging" throughout this application uses stderr.
//!
//! See description: https://home.hccnet.nl/h.g.muller/engine-intf.html

use std::io::{self, Stdin, Stdout};

#[derive(Debug)]
pub struct Communicator {
    input: Stdin,
    output: Stdout,
}

impl Communicator {
    #[inline]
    pub fn new() -> Self {
        Communicator {
            input: io::stdin(),
            output: io::stdout(),
        }
    }
}
