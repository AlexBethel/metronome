// Code for the tempo-set functionality.
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
use crate::app_state::{AppState, StateManager};
use crate::beat_spec::BeatSpec;
use crate::constants;
use crate::met_model::MetronomeState;
use crate::set_view::SetView;
use crate::sound::AudioConfig;
use std::time::Duration;

// State of the set mode.
pub struct SetState {
    // Number the user has typed thus far. Currently, we only let the
    // user specify integral tempos.
    tempo: u32,

    // Rhythm to send back to the MetronomeState.
    rhythm: BeatSpec,

    // Audio configuration to send back to the MetronomeState.
    cfg: AudioConfig,

    // Volume to send back to the MetronomeState.
    volume: f64,

    // The visual representation of the SetState.
    view: SetView,
}

impl SetState {
    // Constructs a new SetState given information from the previous
    // MetronomeState. If first_digit is provided, it is used as the
    // first pre-inputted digit.
    pub fn new(rhythm: BeatSpec, cfg: AudioConfig, volume: f64, first_digit: Option<u32>) -> Self {
        let mut view = SetView::new(volume);
        if let Some(x) = first_digit {
            view.set_tempo(x);
        }

        Self {
            tempo: match first_digit {
                None => 0,
                Some(x) => x,
            },
            rhythm,
            cfg,
            volume,
            view,
        }
    }

    // Leaves Set mode and returns to Metronome mode.
    fn exit(&self, mgr: &mut StateManager) {
        mgr.set_state(Box::new(MetronomeState::new(
            &self.rhythm,
            self.cfg.clone(),
            self.volume,
            self.tempo as f64,
        )));
    }
}

impl AppState for SetState {
    fn tick(&mut self, _mgr: &mut StateManager) {
        self.view.draw();
    }

    fn keypress(&mut self, mgr: &mut StateManager, key: Keycode, _time: Duration) {
        match key {
            Keycode::Key(b'\x03') | Keycode::NoKey => {
                // Exit on Control-C or EOF
                mgr.exit();
            }
            Keycode::Key(k) => {
                if (b'0'..=b'9').contains(&k) {
                    let digit = (k - b'0') as u32;
                    self.tempo *= 10;
                    self.tempo += digit;

                    if (self.tempo * 10) > constants::TEMPO_MAX as u32 {
                        // Any more keys typed by the user would
                        // result in an invalid tempo; go ahead and
                        // submit for them.
                        self.exit(mgr)
                    } else {
                        self.view.set_tempo(self.tempo);
                        self.view.draw();
                    }
                } else {
                    self.exit(mgr)
                }
            }
        }
    }
}
