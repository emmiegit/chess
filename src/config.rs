/*
 * options.rs
 *
 * mallard-chess - Chess engine wrapper utility for fun
 * Copyright (C) 2022 Ammon Smith
 *
 * mallard-chess is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 */

use crate::engine::EngineKind;
use clap::{Arg, Command};
use std::convert::TryFrom;
use std::process;

#[derive(Debug)]
pub struct Configuration {
    pub engine_kind: EngineKind,
    pub stockfish_nodes: Option<u64>,
}

impl Configuration {
    pub fn load() -> Self {
        let matches = Command::new("Mallard Chess")
            .author(env!("CARGO_PKG_AUTHORS"))
            .version(env!("CARGO_PKG_VERSION"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .arg(
                Arg::new("stockfish-nodes")
                    .short('N')
                    .long("nodes")
                    .takes_value(true)
                    .value_name("NODES")
                    .default_value("unlimited")
                    .help("Number of nodes for Stockfish to explore in its evaluation"),
            )
            .arg(
                Arg::new("engine")
                    .required(true)
                    .takes_value(true)
                    .value_name("NAME")
                    .help("What internal engine to play using"),
            )
            .get_matches();

        let stockfish_nodes = {
            let value = matches
                .value_of("stockfish-nodes")
                .expect("Missing default argument");

            if value == "-" || value == "unlimited" {
                None
            } else {
                match value.parse() {
                    Ok(nodes) => Some(nodes),
                    Err(error) => {
                        eprintln!("Invalid Stockfish node depth: {}", error);
                        process::exit(1);
                    }
                }
            }
        };

        let engine_kind = {
            let value = matches
                .value_of("engine")
                .expect("Missing required argument");

            match EngineKind::try_from(value) {
                Ok(game_mode) => game_mode,
                Err(_) => {
                    eprintln!("Unknown game engine: {}", value);
                    EngineKind::print_variants();
                    process::exit(1);
                }
            }
        };

        Configuration {
            engine_kind,
            stockfish_nodes,
        }
    }
}
