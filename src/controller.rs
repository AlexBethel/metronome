// Interactive controls of the user interface.
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

use crate::constants;
use crate::errors::*;
use std::fmt;
use std::io::{stdin, Read};
use std::os::unix::io::AsRawFd;
use std::sync::mpsc::Sender;
use termios::Termios;

// Messages passed from the controller to the model, indicating user
// requests.
#[derive(Debug)]
pub enum ControllerMsg {
    // Pause the metronome if it is running; do nothing if it is
    // already paused.
    Pause,

    // Play the metronome if it is paused; do nothing if it is already
    // running.
    Play,

    // Toggle the metronome between a playing and paused state.
    Toggle,

    // Increase the volume by the given amount (volume is on a scale
    // from 0.0 to 1.0).
    AdjustVolume(f64),

    // Increase the tempo by the given logarithmic measure, where 1.0
    // doubles the tempo and -1.0 halves it.
    AdjustTempo(f64),

    // Exits the program.
    Quit,
}

// A mapping from a key (represented as a set of characters, [u8]) to
// some functionality.
struct Binding(&'static [u8], &'static dyn Fn(&Sender<ControllerMsg>) -> ());

impl PartialEq for Binding {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl fmt::Debug for Binding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Binding").field(&self.0).finish()
    }
}

// Runs the controller, sending messages through the given channel.
// Returns only on error.
pub fn run_controller(sender: Sender<ControllerMsg>) -> Result<()> {
    let keys = init_keybindings();
    loop {
        // Read input in terms of keystrokes, which can consist of
        // multiple characters (e.g. arrow keys, which are represented
        // as an escape sequence).
        let mut keystroke = vec![];
        loop {
            // println!("{:?}\r", keystroke);
            let mut buf = vec![0];
            stdin().read_exact(&mut buf)?;
            // println!("Just got {}\r", buf[0] as u8);
            keystroke.push(buf[0]);

            match get_binding(&keystroke, &keys) {
                BindingState::Invalid => {
                    // println!("Invalid\r");
                    break;
                }
                BindingState::Start => {
                    // println!("Start\r");
                }
                BindingState::Complete(b) => {
                    // println!("Complete\r");
                    b.1(&sender);
                    break;
                }
            }
        }
    }
}

// Sets up the vector of key mapings used by the program.
fn init_keybindings() -> Vec<Binding> {
    // TODO: Clean this up with a helper function of some sort.
    let mut keys = vec![];
    keys.push(Binding(b"p", &|sender| {
        sender.send(ControllerMsg::Pause).unwrap();
    }));
    keys.push(Binding(b"P", &|sender| {
        sender.send(ControllerMsg::Play).unwrap();
    }));
    keys.push(Binding(b" ", &|sender| {
        sender.send(ControllerMsg::Toggle).unwrap();
    }));

    // Arrow keys
    keys.push(Binding(b"\x1B[A", &|sender| {
        // Up
        sender
            .send(ControllerMsg::AdjustVolume(constants::VOL_ADJUST))
            .unwrap();
    }));
    keys.push(Binding(b"\x1B[B", &|sender| {
        // Down
        sender
            .send(ControllerMsg::AdjustVolume(-constants::VOL_ADJUST))
            .unwrap();
    }));
    keys.push(Binding(b"\x1B[C", &|sender| {
        // Right
        sender
            .send(ControllerMsg::AdjustTempo(constants::TEMPO_ADJUST))
            .unwrap();
    }));
    keys.push(Binding(b"\x1B[D", &|sender| {
        // Left
        sender
            .send(ControllerMsg::AdjustTempo(-constants::TEMPO_ADJUST))
            .unwrap();
    }));

    keys.push(Binding(b"q", &|sender| {
        sender.send(ControllerMsg::Quit).unwrap();
    }));
    keys.push(Binding(b"\x03", &|sender| {
        // Control-C
        sender.send(ControllerMsg::Quit).unwrap();
    }));

    keys
}

// Possible states of the key binding engine.
#[derive(PartialEq, Debug)]
enum BindingState<'a> {
    // The characters in the queue are not a valid key binding, nor
    // are they the first part of valid key binding.
    Invalid,

    // The characters in the queue form the beginning of one or more
    // keybindings, but we don't have a complete key binding yet.
    Start,

    // The characters in the queue are a perfect match for a key
    // binding.
    Complete(&'a Binding),
}

// Calculates the state of the key binding engine, given a set of
// characters that have already been received.
fn get_binding<'a>(queue: &[u8], bindings: &'a [Binding]) -> BindingState<'a> {
    let mut is_prefix = false;
    for b in bindings {
        if b.0 == queue {
            return BindingState::Complete(&b);
        }

        if b.0.starts_with(queue) {
            is_prefix = true;
        }
    }

    match is_prefix {
        true => BindingState::Start,
        false => BindingState::Invalid,
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multichar_binding_test() {
        let bindings = init_keybindings();

        // Type in the left arrow key, character by character.
        let mut left = vec![];
        assert_eq!(get_binding(&left, &bindings), BindingState::Start);
        left.push('\x1B' as u8);
        assert_eq!(get_binding(&left, &bindings), BindingState::Start);
        left.push(91 as u8);
        assert_eq!(get_binding(&left, &bindings), BindingState::Start);
        left.push('X' as u8);
        assert_eq!(get_binding(&left, &bindings), BindingState::Invalid);
        left.pop();
        left.push('D' as u8);
        match get_binding(&left, &bindings) {
            BindingState::Complete(_) => (),
            _ => panic!("Didn't recognize binding"),
        };
    }
}
