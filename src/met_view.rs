// Visual part of the user interface.
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
use std::fmt::Display;
use std::io::{stdout, Write};

// Direction of movement for the metronome indicator.
enum Direction {
    Left,
    Right,
}

// State of the view module; this represents exactly which numbers and
// indicators are visible on the screen.
pub struct ViewState {
    // Current progress through a measure, on a scale from 0 to 1.
    progress: f64,

    // The tempo in bpm.
    tempo: f64,

    // The volume from 0 to 1.
    volume: f64,

    // The number of beats per measure.
    beats_per_measure: f64,
}

impl ViewState {
    pub fn new(beats_per_measure: f64) -> Self {
        Self {
            progress: 0.0,
            tempo: constants::DEF_TEMPO,
            volume: constants::DEF_VOLUME,
            beats_per_measure,
        }
    }

    // Sets the progress through the measure.
    pub fn set_progress(&mut self, progress: f64) {
        self.progress = progress;
    }

    // Sets the tempo in beats per minute.
    pub fn set_tempo(&mut self, tempo: f64) {
        self.tempo = tempo;
    }

    // Sets the volume from 0 to 1.
    pub fn set_volume(&mut self, volume: f64) {
        self.volume = volume;
    }

    // Visual indicator string for the tempo marking.
    fn tempo_indicator(&self) -> String {
        format!("{:1$}", self.tempo as u32, constants::NUM_INDIC_WIDTH)
    }

    // Visual indicator for the progress through the measure. In this
    // implementation, we use an asterisk that bounces back and forth
    // across the fixed-width indicator.
    fn progress_indicator(&self) -> String {
        let mut indicator = String::new();
        indicator.reserve(constants::MEAS_INDIC_WIDTH as usize);

        let total_spaces = constants::MEAS_INDIC_WIDTH - 1;
        let leading_spaces = (total_spaces as f64 * self.progress_indicator_pos()) as usize;
        let trailing_spaces = total_spaces - leading_spaces;

        indicator.push_str(&" ".repeat(leading_spaces));
        indicator.push('*');
        indicator.push_str(&" ".repeat(trailing_spaces));

        indicator
    }

    // Calculates the position of the progress indicator, where 0 is
    // the left side of the indicator and 1 is the right side.
    fn progress_indicator_pos(&self) -> f64 {
        let beat_fl = self.beats_per_measure * self.progress;
        let n_beats = beat_fl as u32;
        let dir = if n_beats % 2 == 0 {
            Direction::Right
        } else {
            Direction::Left
        };

        let beat_progress = beat_fl % 1.0;
        match dir {
            Direction::Left => 1.0 - beat_progress,
            Direction::Right => beat_progress,
        }
    }

    // Visual indicator for the volume level.
    fn volume_indicator(&self) -> String {
        format!(
            "{:1$}%",
            (self.volume * 100.0) as u32,
            constants::NUM_INDIC_WIDTH
        )
    }

    // Draws the ViewState on the screen.
    pub fn draw(&self) {
        // Reset to the left edge of the screen, so as to draw over
        // whatever ViewState was there before.
        print!("\r");

        print!("{}", self);

        stdout().flush().unwrap();
    }
}

impl Display for ViewState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "[{}] [{}] ({})",
            self.tempo_indicator(),
            self.progress_indicator(),
            self.volume_indicator()
        )
    }
}
