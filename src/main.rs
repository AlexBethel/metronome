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
pub mod app_state;
pub mod beat_spec;
pub mod config;
pub mod constants;
pub mod met_controller;
pub mod met_model;
pub mod met_view;
pub mod sound;
pub mod termios_handler;

use app_state::state_loop;
use config::Config;
use met_model::MetronomeState;
use std::env;
use termios_handler::TermiosHandler;

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
            RecvError(::std::sync::mpsc::RecvError);
            RecvTimeoutError(::std::sync::mpsc::RecvTimeoutError);
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
        let _termios = TermiosHandler::set_stdin_raw()?;
        let init_state = MetronomeState::new(&cfg.rhythm)?;

        let s = state_loop(Box::new(init_state));
        return s;
    }

    Ok(())
}
