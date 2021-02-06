// Display for the Set mode.
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
use crate::met_view::fmt_metronome_like;
use std::fmt::Display;
use std::io::{stdout, Write};
// use std::time::Duration;

pub struct SetView {
    // The volume from 0 to 1.
    volume: f64,

    // The tempo typed by the user.
    tempo: u32,
}

impl SetView {
    // Creates a new SetView with a blank tempo.
    pub fn new(volume: f64) -> Self {
        Self { volume, tempo: 0 }
    }

    // Sets the tempo of this object.
    pub fn set_tempo(&mut self, tempo: u32) {
        self.tempo = tempo;
    }

    // Visual indicator for the tempo. For Set mode, this is the tempo
    // written as an integer, right-aligned in the field with dots as
    // padding.
    fn tempo_indicator(&self) -> String {
        let tempo_string = match self.tempo {
            0 => "".into(),
            _ => format!("{}", self.tempo),
        };
        let dots_len = constants::NUM_INDIC_WIDTH - tempo_string.len();
        ".".repeat(dots_len) + &tempo_string
    }

    // Visual indicator for the volume level.
    fn volume_indicator(&self) -> String {
        format!(
            "{:1$}%",
            (self.volume * 100.0) as u32,
            constants::NUM_INDIC_WIDTH,
        )
    }

    // Redraws the view.
    pub fn draw(&self) {
        // Reset to the left edge of the screen, so as to draw over
        // whatever view was there before.
        print!("\r");

        print!("{}", self);

        // Seek to the right edge of the tempo indicator. Right now
        // I'm using a Linux-specific control code here; TODO: find a
        // more general way to do this.
        print!("\r\x1B[{}C", 1 + constants::NUM_INDIC_WIDTH);

        stdout().flush().unwrap();
    }
}

impl Display for SetView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        fmt_metronome_like(
            f,
            Some(&self.tempo_indicator()),
            None,
            Some(&self.volume_indicator()),
        )
    }
}
