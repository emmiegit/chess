/*
 * macros.rs
 *
 * mallard-chess - Chess engine wrapper for fun
 * Copyright (C) 2022 Ammon Smith
 *
 * mallard-chess is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 */

macro_rules! log {
    ($file:expr $(,)?) => {
        writeln!(&*$file).expect("Unable to write to log file")
    };

    ($file:expr, $($arg:tt)+) => {
        writeln!(&*$file, $($arg)+).expect("Unable to write to log file")
    };
}
