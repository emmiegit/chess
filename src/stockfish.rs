/*
 * stockfish.rs
 *
 * mallard-chess - Chess engine wrapper utility for fun
 * Copyright (C) 2022 Ammon Smith
 *
 * mallard-chess is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 */

//! Module for communicating with Stockfish.
//!
//! Stockfish is a very solid chess engine which we are
//! using for our various "modes" of chess engine operation.
//!
//! This application is essentially "piping through" what
//! Stockfish determines, with modifications depending on the mode.

use std::fmt::Display;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use vampirc_uci::{parse_one, UciMessage};

macro_rules! recv_inner {
    ($self:expr, $buffer:expr $(,)?) => {
        $self
            .input
            .read_line($buffer)
            .expect("Unable to read from stockfish")
    };
}

#[derive(Debug)]
pub struct Stockfish {
    process: Child,
    input: BufReader<ChildStdout>,
    output: ChildStdin,
    buffer: String,
}

impl Stockfish {
    // Constructor
    pub fn spawn() -> Self {
        let mut process = Command::new("stockfish")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("Unable to start stockfish");

        let stdin = process.stdin.take().expect("No stdin opened");
        let stdout = process.stdout.take().expect("No stdout opened");

        Stockfish {
            process,
            input: BufReader::new(stdout),
            output: stdin,
            buffer: String::new(),
        }
    }

    // Communication
    pub fn recv_raw(&mut self) -> String {
        let mut buffer = String::new();
        recv_inner!(self, &mut buffer);
        buffer
    }

    pub fn recv(&mut self) -> UciMessage {
        self.buffer.clear();
        recv_inner!(self, &mut self.buffer);
        parse_one(&self.buffer)
    }

    pub fn send<D: Display>(&mut self, command: D) {
        writeln!(self.output, "{}", command).expect("Unable to write to stockfish");
        self.output.flush().expect("Unable to flush stockfish pipe");
    }

    // Methods
    // TODO evaluate fn
}
