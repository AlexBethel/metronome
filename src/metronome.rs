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
use crate::controller::ControllerMsg;
use crate::errors::*;
use crate::sound::{beep, AudioConfig};
use num::traits::Pow;
use std::sync::mpsc::Receiver;
use std::time::Duration;
use std::time::Instant;

// Plays a ticking pattern with the given rhythm loop.
pub fn do_metronome(rhythm: &BeatSpec, controls: Receiver<ControllerMsg>) -> Result<()> {
    let cfg = AudioConfig::new()?;
    let delay = get_delay(rhythm);

    let mut volume = constants::DEF_VOLUME;
    let mut delay_mult = 1.0;
    loop {
        for tick in rhythm.get_ticks() {
            play_event(tick, &cfg, volume);
            for cmd in TimedReceiver::new(&controls, delay.mul_f64(delay_mult)) {
                if proc_cmd(cmd, &mut volume, &mut delay_mult) == CmdResult::Exit {
                    return Ok(());
                }
            }
        }
    }
}

// Possible results of a controller command.
#[derive(PartialEq)]
enum CmdResult {
    // No further action
    None,

    // Exit the program.
    Exit,
}

// Acts upon a command received from the controller. Adjusts the
// volume and delay multipliers according to the user's request.
fn proc_cmd(cmd: ControllerMsg, vol: &mut f64, delay_mult: &mut f64) -> CmdResult {
    match cmd {
        ControllerMsg::Pause => {
            // TODO: Actually block until a new command is received.
            // For now, I'm just emulating roughly what it'll act like
            // by toggling volume on and off.
            *vol = 0.0;
        }
        ControllerMsg::Play => {
            *vol = constants::DEF_VOLUME;
        }
        ControllerMsg::Toggle => {
            // This is such a rough approximation lol
            *vol = constants::DEF_VOLUME - *vol;
        }
        ControllerMsg::AdjustVolume(x) => {
            *vol = *vol + x;
        }
        ControllerMsg::AdjustTempo(x) => {
            let base: f64 = 2.0;
            // Divide, since a delay multiplier (internal) is inverse
            // to the tempo (what the user sees).
            *delay_mult = *delay_mult / base.pow(x);
        }
        ControllerMsg::Quit => {
            return CmdResult::Exit;
        }
    }

    // Bounds check.
    if *vol < 0.0 {
        *vol = 0.0;
    } else if *vol > 1.0 {
        *vol = 1.0;
    }

    CmdResult::None
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

// Iterator over messages received from a channel over a specific
// period of time.
struct TimedReceiver<'a, T> {
    // Start and duration of the time frame.
    start_time: Instant,
    duration: Duration,

    // Source of the messages.
    channel: &'a Receiver<T>,
}

impl<'a, T> TimedReceiver<'a, T> {
    pub fn new(channel: &'a Receiver<T>, duration: Duration) -> Self {
        TimedReceiver {
            start_time: Instant::now(),
            duration,
            channel,
        }
    }
}

impl<'a, T> Iterator for TimedReceiver<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let time = self.start_time.elapsed();
        if time > self.duration {
            // Timer expired, terminate the iterator.
            return None;
        }

        match self.channel.recv_timeout(self.duration - time) {
            Ok(x) => Some(x),
            // FIXME: This doesn't differentiate the other end hanging
            // up from the timer expiring; need to catch that error
            // specifically and propagate it.
            Err(_) => None,
        }
    }
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
