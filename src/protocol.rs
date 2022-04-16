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

use std::io::{self, BufRead, Result, Stdin, Stdout, Write};

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

    fn read_raw(&mut self) -> Result<String> {
        let mut buffer = String::new();
        let mut guard = self.input.lock();
        guard.read_line(&mut buffer)?;
        Ok(buffer)
    }

    fn write_raw(&mut self, command: &str) -> Result<()> {
        assert!(
            command.ends_with('\n'),
            "Raw command does not end with newline",
        );

        let mut guard = self.output.lock();
        guard.write_all(command.as_bytes())?;
        guard.flush()?;
        Ok(())
    }

    pub fn read(&mut self) -> ReceivedCommand {
        let command = self.read_raw().expect("Read error");
        let parts = command.split_ascii_whitespace();
    }

    pub fn write(&mut self, command: &SentCommand) {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum ReceivedCommand {
    SynchronizeXboard,
    SynchronizeProtocol(i32),
    // TODO
}

#[derive(Debug, Clone)]
pub enum SentCommand {
    // TODO
}
