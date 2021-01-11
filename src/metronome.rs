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

use crate::beat_spec::{BeatSpec, Event};
use crate::constants;
use crate::errors::*;
use crate::sound::{beep, AudioConfig};
use std::thread::sleep;
use std::time::Duration;

// Plays a ticking pattern with the given rhythm loop.
pub fn do_metronome(rhythm: &BeatSpec) -> Result<()> {
    let cfg = AudioConfig::new()?;
    let delay = get_delay(rhythm);
    loop {
        for tick in rhythm.get_ticks() {
            play_event(tick, &cfg);
            sleep(delay);
        }
    }
}

// Plays a single BeatSpec event.
fn play_event(evt: &Event, cfg: &AudioConfig) {
    match *evt {
        Event::Rest => {}
        Event::Beep(emph) => beep(
            constants::BEEP_PITCH / (emph + 1) as f64,
            Duration::from_millis(constants::BEAT_LEN),
            cfg,
        ),
    }
}

// Gets the time delay between two ticks of the given BeatSpec.
fn get_delay(bs: &BeatSpec) -> Duration {
    let beat_time = 60.0 / bs.get_tempo();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, ConfigResult};

    #[test]
    fn delay_test() {
        let cfg = match Config::new(&vec!["foo", "72:4:3"]).unwrap() {
            ConfigResult::Run(x) => x,
            ConfigResult::DontRun => {
                panic!("Got DontRun");
            }
        };

        assert_eq!(get_delay(&cfg.rhythm), seconds(60.0 / 72.0 / 3.0));
    }

    #[test]
    fn seconds_test() {
        assert_eq!(seconds(5.5), Duration::from_millis(5500));
    }
}
