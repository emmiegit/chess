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
//! Stockfish determines, with modifications set via
//! command-line settings.

// TODO
