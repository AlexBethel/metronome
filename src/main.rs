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
pub mod controller;
pub mod metronome;
pub mod sound;
pub mod view;

use app_state::state_loop;
use config::Config;
use metronome::MetronomeState;
use std::env;
use std::io::stdin;
use std::os::unix::io::AsRawFd;
use termios::Termios;

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
        let termios = init_termios()?;
        let init_state = MetronomeState::new(&cfg.rhythm)?;

        let s = state_loop(Box::new(init_state));
        cleanup_termios(&termios).unwrap();
        return s;
    }

    Ok(())
}

// Sets the terminal to raw mode, as is necessary for reading key
// bindings from a terminal in real time. Returns the original termios
// state.
pub fn init_termios() -> Result<Termios> {
    // TODO: Windows compatibility -- Termios doesn't work on Windows.
    let stdin_fd = stdin().as_raw_fd();
    let mut t = termios::Termios::from_fd(stdin_fd).unwrap();
    let orig_termios = t.clone();

    termios::cfmakeraw(&mut t);
    termios::tcsetattr(stdin_fd, termios::TCSANOW, &t)?;
    Ok(orig_termios)
}

// Resets the terminal from raw mode to the given initial state.
pub fn cleanup_termios(orig: &Termios) -> Result<()> {
    let stdin_fd = stdin().as_raw_fd();
    termios::tcsetattr(stdin_fd, termios::TCSANOW, orig)?;

    Ok(())
}
