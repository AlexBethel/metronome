// Interactive controls of the Metronome user interface.
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
use std::fmt;

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

    // Increase the tempo by the given number of beats per measure.
    AdjustTempo(f64),

    // Exits the program.
    Quit,
}

pub struct ControllerState {
    // The mapping from key events to commands.
    mapping: Vec<Binding>,

    // Partial key combination entered. This is only used for parsing
    // multi-byte escape codes at the moment, and should not be used
    // for actual series of keystrokes (a la Emacs).
    partial: Vec<u8>,
}

impl ControllerState {
    // Creates a new ControllerState.
    pub fn new() -> ControllerState {
        ControllerState {
            mapping: init_keybindings(),
            partial: vec![],
        }
    }

    // Sends a byte received from the keyboard to the controller,
    // which processes it according to the keymap and may or may not
    // produce a message directing what to do.
    pub fn send(&mut self, key: u8) -> Option<ControllerMsg> {
        self.partial.push(key);
        match get_binding(&self.partial, &self.mapping) {
            BindingState::Invalid => {
                println!("Invalid\r");
                self.partial = vec![];
                None
            }
            BindingState::Start => None,
            BindingState::Complete(b) => {
                self.partial = vec![];
                b.1()
            }
        }
    }
}

// A mapping from a key (represented as a set of characters, [u8]) to
// some functionality.
struct Binding(&'static [u8], &'static dyn Fn() -> Option<ControllerMsg>);

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

// Sets up the vector of key mapings used by the program.
fn init_keybindings() -> Vec<Binding> {
    // TODO: Clean this up with a helper function of some sort.
    let mut keys = vec![];
    keys.push(Binding(b"p", &|| Some(ControllerMsg::Pause)));
    keys.push(Binding(b"P", &|| Some(ControllerMsg::Play)));
    keys.push(Binding(b" ", &|| Some(ControllerMsg::Toggle)));

    // Arrow keys
    keys.push(Binding(b"\x1B[A", &|| {
        // Up
        Some(ControllerMsg::AdjustVolume(constants::VOL_ADJUST))
    }));
    keys.push(Binding(b"\x1B[B", &|| {
        // Down
        Some(ControllerMsg::AdjustVolume(-constants::VOL_ADJUST))
    }));
    keys.push(Binding(b"\x1B[C", &|| {
        // Right
        Some(ControllerMsg::AdjustTempo(constants::TEMPO_ADJUST))
    }));
    keys.push(Binding(b"\x1B[D", &|| {
        // Left
        Some(ControllerMsg::AdjustTempo(-constants::TEMPO_ADJUST))
    }));

    keys.push(Binding(b"q", &|| Some(ControllerMsg::Quit)));
    keys.push(Binding(b"\x03", &|| {
        // Control-C
        Some(ControllerMsg::Quit)
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
