// Simple Termios state controller.
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

use crate::errors::*;
use std::io::stdin;
use std::os::unix::io::{AsRawFd, RawFd};
use termios::Termios;

// TODO: Add stub functionality for MS Windows, which doesn't support
// Termios.

// Data structure representing a temporary change in the Termios
// state. When this gets created, the state change is initialized;
// when it is dropped, that state change is reverted.
pub struct TermiosHandler {
    // Terminal file descriptor this Termios controls.
    fd: RawFd,

    // The original Termios, from before we changed the state.
    orig: Termios,
}

impl TermiosHandler {
    // Sets stdin to raw mode.
    pub fn set_stdin_raw() -> Result<TermiosHandler> {
        let stdin_fd = stdin().as_raw_fd();
        let mut t = Termios::from_fd(stdin_fd).unwrap();
        let orig_termios = t.clone();

        termios::cfmakeraw(&mut t);
        termios::tcsetattr(stdin_fd, termios::TCSANOW, &t)?;
        Ok(TermiosHandler {
            fd: stdin_fd,
            orig: orig_termios,
        })
    }
}

impl Drop for TermiosHandler {
    // Restore the termios to its prior state when this structure goes
    // out of scope.
    fn drop(&mut self) {
        termios::tcsetattr(self.fd, termios::TCSANOW, &self.orig).unwrap();
    }
}
