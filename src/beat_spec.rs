// Definition and usages of beat specifications.
// Copyright (c) 2020 by Alexander Bethel.

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

// Description of precisely what events should occur and when during a
// single measure.
#[derive(Debug)]
pub struct BeatSpec {
    // The set of events to run, and where in the measure each event
    // happens. (Should be sorted by time.)
    events: Vec<TimedEvent>,

    // Length of a measure, in beats.
    measure_len: f64,

    // Tempo, in beats per minute.
    tempo: f64,
}

// An event and the time at which it happens.
#[derive(Debug)]
struct TimedEvent {
    // Time measured in beats.
    time: f64,

    event: Event,
}

// Different types of events that can occur in a measure.
#[derive(Debug)]
enum Event {
    // Default metronome sound; the u32 is the emphasis level of the
    // beat.
    Beep(u32),

    // Could add other types of sounds, messages etc. in the future.
}

impl BeatSpec {
    // Creates a BeatSpec given a number of beats per measure and
    // subdivisions per beat.
    pub fn from_subdiv(tempo: f64, beats: u32, subdiv: u32) -> BeatSpec {
        Self::from_crossbeats(tempo, &vec![beats, beats * subdiv])
    }

    // Creates a BeatSpec given a set of simultaneous cross-rhythms,
    // specified in order of decreasing emphasis.
    pub fn from_crossbeats(tempo: f64, beats: &[u32]) -> BeatSpec {
        // Need at least one cross-rhythm for this constructor to be
        // meaningful at all.
        let beats_default = vec![1];
        let beats = if beats.len() == 0 {
            &beats_default
        } else {
            beats
        };

        let mut evts = vec![];
        for i in 0..beats.len() {
            push_beeps(&mut evts, beats[i] as f64 / beats[0] as f64,
                       beats[i], (i + 1) as u32);
        }

        BeatSpec {
            events: evts,
            measure_len: beats[0] as f64,
            tempo: tempo / beats[0] as f64,
        }
    }

    // Creates a BeatSpec from a rhythm specification string.
    pub fn from_rhythmspec(_tempo: f64, _spec: &str) -> BeatSpec {
        // TODO: Implement.
        BeatSpec {
            events: vec![],
            measure_len: 1.0,
            tempo: 60.0,
        }
    }

    // Plays a single measure with this BeatSpec.
    pub fn play_measure(&self) {
        // TODO: Implement.
    }
}

// Pushes a set of `n_beats' evenly-spaced beeps (separated by `delay'
// beats) to the list of metronome events.
fn push_beeps(evts: &mut Vec<TimedEvent>, delay: f64, n_beats: u32,
              emph: u32) {
    for n in 0..n_beats {
        let time = n as f64 * delay;
        evts.push(TimedEvent {
            time,
            event: Event::Beep(emph),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subdiv_test() {
        let bs = BeatSpec::from_subdiv(60.0, 3, 2);

        assert_eq!(bs.events.len(), 6);
        assert_eq!(bs.measure_len, 3.0);
        assert_eq!(bs.tempo, 60.0);
    }

    #[test]
    fn crossbeat_test() {
        let bs = BeatSpec::from_crossbeats(60.0, &vec![3, 6]);

        assert_eq!(bs.events.len(), 6);
        assert_eq!(bs.measure_len, 3.0);
        assert_eq!(bs.tempo, 60.0);
    }

    #[test]
    fn rspec_test() {
        let bs = BeatSpec::from_rhythmspec(60.0, "02!1212");

        assert_eq!(bs.events.len(), 6);
        assert_eq!(bs.measure_len, 3.0);
        assert_eq!(bs.tempo, 60.0);
    }
}
