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
#[derive(Debug, Clone, Copy)]
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

    // Synchronizes the metronome, so a downbeat occurs the instant
    // this message is received.
    Sync,

    // Enters tap mode.
    TapMode,

    // Enters set mode (for setting the tempo). Optionally includes a
    // first digit to input.
    SetMode(Option<u32>),

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
                self.partial = vec![];
                None
            }
            BindingState::Start => None,
            BindingState::Complete(b) => {
                self.partial = vec![];
                Some(b.1)
            }
        }
    }
}

// A mapping from a key (vector of input codes) to some functionality.
struct Binding(Vec<u8>, ControllerMsg);

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
    use ControllerMsg::*;
    let mut keys = vec![
        Binding(b"p".to_vec(), Pause),
        Binding(b"P".to_vec(), Play),
        Binding(b" ".to_vec(), Toggle),
        Binding(b".".to_vec(), Sync),
        Binding(b",".to_vec(), TapMode),
        Binding(b"'".to_vec(), SetMode(None)),
        Binding(b"q".to_vec(), Quit),
        // Control-C
        Binding(b"\x03".to_vec(), Quit),
        // ---- Arrow keys ----
        // Up
        Binding(b"\x1B[A".to_vec(), AdjustVolume(constants::VOL_ADJUST)),
        // Down
        Binding(b"\x1B[B".to_vec(), AdjustVolume(-constants::VOL_ADJUST)),
        // Right
        Binding(b"\x1B[C".to_vec(), AdjustTempo(constants::TEMPO_ADJUST)),
        // Left
        Binding(b"\x1B[D".to_vec(), AdjustTempo(-constants::TEMPO_ADJUST)),
        // ---- Vim-like directional keys ----
        Binding(b"k".to_vec(), AdjustVolume(constants::VOL_ADJUST)),
        Binding(b"j".to_vec(), AdjustVolume(-constants::VOL_ADJUST)),
        Binding(b"l".to_vec(), AdjustTempo(constants::TEMPO_ADJUST)),
        Binding(b"h".to_vec(), AdjustTempo(-constants::TEMPO_ADJUST)),
        // ---- Emacs-like directional keys ----
        // C-p
        Binding(b"\x10".to_vec(), AdjustVolume(constants::VOL_ADJUST)),
        // C-n
        Binding(b"\x0E".to_vec(), AdjustVolume(-constants::VOL_ADJUST)),
        // C-f
        Binding(b"\x06".to_vec(), AdjustTempo(constants::TEMPO_ADJUST)),
        // C-b
        Binding(b"\x02".to_vec(), AdjustTempo(-constants::TEMPO_ADJUST)),
    ];

    // Add number keys for directly typing tempo.
    for c in b'0'..=b'9' {
        keys.push(Binding(vec![c], SetMode(Some((c - b'0') as u32))));
    }

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
