// Code for running the actual metronome, after its configuration has
// been loaded.
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

use crate::app_state::{AppState, Keycode, StateManager};
use crate::beat_spec::{BeatSpec, Event};
use crate::constants;
use crate::met_controller::{ControllerMsg, ControllerState};
use crate::met_view::MetronomeView;
use crate::set_model::SetState;
use crate::sound::{beep, AudioConfig};
use crate::tap_model::TapState;
use std::time::Duration;

// State of the metronome at any given time.
pub struct MetronomeState {
    // The rhythm the metronome is beating out.
    rhythm: BeatSpec,

    // The index of the next tick to be played by the metronome.
    tick_number: usize,

    // The audio device configuration.
    cfg: AudioConfig,

    // The current volume and tempo settings.
    volume: f64,
    tempo: f64,

    // State of the view and controller subsystems.
    view: MetronomeView,
    controller: ControllerState,
}

impl MetronomeState {
    pub fn new(rhythm: &BeatSpec, cfg: AudioConfig, volume: f64, tempo: f64) -> MetronomeState {
        MetronomeState {
            rhythm: rhythm.clone(),
            tick_number: 0,
            cfg,
            volume,
            tempo,
            view: MetronomeView::new(
                rhythm.get_ticks().len() as f64 / rhythm.get_beat_len() as f64,
            ),
            controller: ControllerState::new(),
        }
    }
}

impl AppState for MetronomeState {
    fn tick(&mut self, mgr: &mut StateManager) {
        let ticks = &self.rhythm.get_ticks();
        let tick = &ticks[self.tick_number];
        play_event(tick, &self.cfg, self.volume);

        self.view
            .set_progress(self.tick_number as f64 / ticks.len() as f64);
        self.view.set_tempo(self.tempo);
        self.view.set_volume(self.volume);
        self.view.draw();

        self.tick_number = (self.tick_number + 1) % ticks.len();

        mgr.set_tick(get_delay(&self.rhythm, self.tempo));
    }

    fn keypress(&mut self, mgr: &mut StateManager, key: Keycode, _time: Duration) {
        let cmd = if let Keycode::Key(key) = key {
            self.controller.send(key)
        } else {
            // stdin closed, quit the program.
            mgr.exit();
            return;
        };

        if let Some(cmd) = cmd {
            match cmd {
                ControllerMsg::Pause => mgr.pause(),
                ControllerMsg::Play => mgr.resume(),
                ControllerMsg::Toggle => mgr.toggle_paused(),
                ControllerMsg::AdjustVolume(x) => {
                    self.volume += x;
                    if self.volume < 0.0 {
                        self.volume = 0.0;
                    } else if self.volume > 1.0 {
                        self.volume = 1.0;
                    }

                    self.view.set_volume(self.volume);
                    self.view.draw();
                }
                ControllerMsg::AdjustTempo(x) => {
                    self.tempo += x;
                    if self.tempo < constants::TEMPO_MIN {
                        self.tempo = constants::TEMPO_MIN;
                    } else if self.tempo > constants::TEMPO_MAX {
                        self.tempo = constants::TEMPO_MAX;
                    }

                    self.view.set_tempo(self.tempo);
                    self.view.draw();
                }
                ControllerMsg::Sync => {
                    self.tick_number = 0;
                    mgr.set_tick(Duration::new(0, 0));
                }
                ControllerMsg::TapMode => {
                    mgr.set_state(Box::new(TapState::new(
                        self.rhythm.clone(),
                        self.cfg.clone(),
                        self.volume,
                    )));
                }
                ControllerMsg::SetMode(first_digit) => {
                    mgr.set_state(Box::new(SetState::new(
                        self.rhythm.clone(),
                        self.cfg.clone(),
                        self.volume,
                        first_digit,
                    )));
                }
                ControllerMsg::Quit => mgr.exit(),
            };
        }
    }
}

// Plays a single BeatSpec event with the given configuration and
// volume.
fn play_event(evt: &Event, cfg: &AudioConfig, vol: f64) {
    match *evt {
        Event::Rest => {}
        Event::Beep(emph) => beep(
            constants::BEEP_PITCH / (emph + 1) as f64,
            Duration::from_millis(constants::BEAT_LEN),
            cfg,
            vol,
        ),
    }
}

// Gets the time delay between two ticks of the given BeatSpec.
fn get_delay(bs: &BeatSpec, tempo: f64) -> Duration {
    let beat_time = 60.0 / tempo;
    let tick_time = beat_time / bs.get_beat_len() as f64;

    seconds(tick_time)
}

// Creates a Duration from the given number of seconds.
fn seconds(secs: f64) -> Duration {
    let s = secs as u64;
    let remainder = secs - s as f64;
    let ns = (remainder * 1_000_000_000.0) as u64;

    Duration::from_secs(s) + Duration::from_nanos(ns)
}
