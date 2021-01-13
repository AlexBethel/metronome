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
use std::io::{stdin, Read};
use std::sync::mpsc::Sender;

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

// Runs the controller, sending messages through the given channel.
// Returns only on error.
pub fn run_controller(sender: Sender<ControllerMsg>) -> Result<()> {
    loop {
        let mut buf = vec![0];
        stdin().read_exact(&mut buf)?;

        proc_keystroke(buf[0], &sender)?;
    }
}

// Performs an action as indicated by the keystroke.
fn proc_keystroke(key: u8, chan: &Sender<ControllerMsg>) -> Result<()> {
    // TODO: Make this work with UTF-8 characters, not bytes.
    match match key as char {
        'p' => chan.send(ControllerMsg::Pause),
        'P' => chan.send(ControllerMsg::Play),
        ' ' => chan.send(ControllerMsg::Toggle),

        // TODO: Re-work these into arrow keys (which require multiple
        // characters).
        'k' => chan.send(ControllerMsg::AdjustVolume(constants::VOL_ADJUST)),
        'j' => chan.send(ControllerMsg::AdjustVolume(-constants::VOL_ADJUST)),
        'l' => chan.send(ControllerMsg::AdjustTempo(constants::TEMPO_ADJUST)),
        'h' => chan.send(ControllerMsg::AdjustTempo(-constants::TEMPO_ADJUST)),

        'q' => chan.send(ControllerMsg::Quit),

        _ => Ok(()),
    } {
        Ok(()) => Ok(()),
        Err(_) => Err("Error sending message".into()),
    }
}
