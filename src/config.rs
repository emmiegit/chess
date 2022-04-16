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

use crate::engine::EngineChoice;
use clap::{Arg, Command};
use std::convert::TryFrom;
use std::fs::File;
use std::process;
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct Configuration {
    engine: EngineChoice,
    log_file: Option<File>,
}

impl Configuration {
    pub fn load() -> Self {
        let matches = Command::new("Mallard Chess")
            .author(env!("CARGO_PKG_AUTHORS"))
            .version(env!("CARGO_PKG_VERSION"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .arg(
                Arg::new("log-output")
                    .short('o')
                    .long("output")
                    .long("log-output")
                    .takes_value(true)
                    .value_name("PATH")
                    .help("Path to optionally share program logging during execution."),
            )
            .arg(
                Arg::new("engine")
                    .required(true)
                    .takes_value(true)
                    .value_name("NAME")
                    .help("What internal engine to play using."),
            )
            .get_matches();

        let engine = {
            let value = matches
                .value_of("engine")
                .expect("Missing required argument");

            match EngineChoice::try_from(value) {
                Ok(game_mode) => game_mode,
                Err(_) => {
                    eprintln!("Unknown game engine: {}", value);
                    eprintln!("Possible values:");

                    for variant in EngineChoice::iter() {
                        eprintln!("- {:?}", variant);
                    }

                    process::exit(1);
                }
            }
        };

        let log_file = matches
            .value_of_os("log-output")
            .map(|path| match File::open(path) {
                Ok(file) => file,
                Err(error) => {
                    eprintln!("Unable to open log output path: {}", error);
                    process::exit(1);
                }
            });

        Configuration { engine, log_file }
    }
}
