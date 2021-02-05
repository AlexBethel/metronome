// Code for running the tap functionality.
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

use crate::app_state::Keycode;
use crate::app_state::{AppState, StateTransition, TickCommand};
use crate::beat_spec::BeatSpec;
use crate::constants;
use crate::met_model::MetronomeState;
use crate::sound::AudioConfig;
use crate::tap_view::TapView;
use std::time::{Duration, Instant};

// State of the tap mode.
pub struct TapState {
    // Times at which each tap occurred.
    times: Vec<Instant>,

    // The rhythm to create when we go back to Metronome mode.
    rhythm: BeatSpec,

    // Configuration for producing tick sounds.
    cfg: AudioConfig,

    // The volume at which to produce tick sounds.
    volume: f64,

    // The on-screen representation of the TapState.
    view: TapView,
}

impl TapState {
    // Constructs a new TapState given the previous MetronomeState.
    pub fn new(rhythm: BeatSpec, cfg: AudioConfig, volume: f64) -> Self {
        Self {
            // The first tap occurs the moment this state is invoked.
            times: vec![Instant::now()],
            rhythm,
            cfg,
            volume,
            view: TapView::new(volume),
        }
    }

    // Calculates the current tempo in beats per minute from the
    // received tap events. Returns None if there is not enough
    // information to solve the problem.
    fn calc_tempo(&self) -> Option<f64> {
        if self.times.len() < 2 {
            return None;
        }

        // To get the average time between consecutive events, we only
        // actually have to look at the first and last events, and
        // know the number of events between them; the times of the
        // other events cancel out in the equation.
        let first_time = self.times[0];
        let last_time = self.times[self.times.len() - 1];

        let time_delta = last_time - first_time;
        let n_events = self.times.len() as u32;

        // There are `n_events' actual tick events, but that equates
        // to only `n_events - 1' intervals between tick events
        // (fencepost problem).
        let time_per_event = time_delta / (n_events - 1);
        let minute = Duration::from_secs(60);

        // Convert to beats per minute.
        Some((minute.as_nanos() / time_per_event.as_nanos()) as f64)
    }

    // Leaves Tap mode and returns to Metronome mode.
    fn exit(&self) -> (StateTransition, TickCommand) {
        (
            StateTransition::To(Box::new(MetronomeState::new(
                &self.rhythm,
                self.cfg.clone(),
                self.volume,
                match self.calc_tempo() {
                    None => constants::DEF_TEMPO,
                    Some(x) => x,
                },
            ))),
            TickCommand::Set(Duration::from_secs(0)),
        )
    }
}

impl AppState for TapState {
    fn tick(&mut self) -> (StateTransition, TickCommand) {
        self.view.draw();
        (StateTransition::NoChange, TickCommand::Unset)
    }

    fn keypress(&mut self, key: Keycode, _time: Duration) -> (StateTransition, TickCommand) {
        // Tap controller is simple enough that it doesn't get its own
        // file. (It's self-contained in this function here.)
        match key {
            Keycode::Key(b',') => {
                self.times.push(Instant::now());
                (StateTransition::NoChange, TickCommand::Unset)
            }
            Keycode::Key(b'\x03') => {
                // Exit on Control-C
                (StateTransition::Exit, TickCommand::Unset)
            }
            _ => self.exit(),
        }
    }
}
