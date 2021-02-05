// Display for the tap mode.
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

pub struct TapView {
    // The volume from 0 to 1.
    volume: f64,
}

impl TapView {
    pub fn new(volume: f64) -> Self {
        Self { volume }
    }

    // Sets the volume level, on a scale from 0 to 1.
    pub fn set_volume(&mut self, volume: f64) {
        self.volume = volume;
    }

    // Visual indicator for the volume level.
    fn volume_indicator(&self) -> String {
        format!(
            "{:1$}%",
            (self.volume * 100.0) as u32,
            constants::NUM_INDIC_WIDTH,
        )
    }

    // Draws the TapView on the screen.
    pub fn draw(&self) {
        // Reset to the left edge of the screen, so as to draw over
        // whatever view was there before.
        print!("\r");

        print!("{}", self);

        stdout().flush().unwrap();
    }
}

impl Display for TapView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        fmt_metronome_like(f, Some("TAP"), None, Some(&self.volume_indicator()))
    }
}
