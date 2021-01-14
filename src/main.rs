// Main program entry point for the program.
// Copyright (c) 2021 by Alexander Bethel.

// This file is part of Metronome.

// Metronome is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published
// by the Free Software Foundation, either version 3 of the License,
// or (at your option) any later version.

// Metronome is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Metronome. If not, see <https://www.gnu.org/licenses/>.

extern crate getopts;
extern crate termios;
pub mod beat_spec;
pub mod config;
pub mod constants;
pub mod controller;
pub mod metronome;
pub mod sound;

use config::Config;
use controller::run_controller;
use metronome::do_metronome;
use std::env;
use std::io::{stderr, Write};
use std::sync::mpsc::channel;
use std::thread;
use controller::{init_termios, cleanup_termios};

use error_chain::{error_chain, quick_main};
mod errors {
    use super::*;
    error_chain! {
        foreign_links {
            Options(::getopts::Fail);
            ParseFloatError(::std::num::ParseFloatError);
            ParseIntError(::std::num::ParseIntError);
            SupportedStreamConfigsError(::cpal::SupportedStreamConfigsError);
            IOError(::std::io::Error);
        }

        errors {
            AudioConfig(e: String) {
                description("Error configuring audio device"),
                display("Error configuring audio device: {}", e),
            }
        }
    }
}
use errors::*;

quick_main!(run);
fn run() -> Result<()> {
    let args_vec: Vec<String> = env::args().collect();
    let mut args_ref: Vec<&str> = vec![];
    for arg in args_vec.iter() {
        args_ref.push(&arg);
    }

    let cfg = Config::new(&args_ref)?;
    if let config::ConfigResult::Run(cfg) = cfg {
        let termios = init_termios()?;
        let (send, recv) = channel();
        thread::spawn(move || {
            if let Err(e) = run_controller(send) {
                cleanup_termios(&termios).unwrap();
                write!(&mut stderr(), "{}", e).unwrap();
                std::process::exit(1);
            }
        });
        do_metronome(&cfg.rhythm, recv)?;
        cleanup_termios(&termios).unwrap();
    }

    Ok(())
}
