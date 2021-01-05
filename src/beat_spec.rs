// Definition and usages of beat specifications.
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

use error_chain::{ bail, error_chain };

mod errors {
    use super::*;
    error_chain! {}
}
pub use errors::*;

// Description of precisely what events should occur and when during a
// single measure.
#[derive(Debug)]
pub struct BeatSpec {
    // The set of events to run during each tick in a measure.
    ticks: Vec<Event>,

    // Length of a beat, in ticks.
    beat_len: u32,

    // Tempo, in beats per minute.
    tempo: f64,
}

// Different types of events that can occur in a measure.
#[derive(Debug, PartialEq)]
enum Event {
    // Do nothing during this tick.
    Rest,

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
    pub fn from_crossbeats(tempo: f64, beats_: &[u32]) -> BeatSpec {
        // Add an implicit crossbeat of 1, so we get a high-pitched
        // beep at the start of each measure.
        let mut beats = vec![1];
        beats.extend_from_slice(beats_);

        let n_ticks = lcm(&beats);
        let mut ticks = vec![];
        for tick in 0..n_ticks {
            let mut ev = Event::Rest;
            for n in 0..beats.len() {
                let beat = beats[n];
                assert_eq!(n_ticks % beat, 0);
                if tick % (n_ticks / beat) == 0 {
                    ev = Event::Beep(n as u32);
                    break;
                }
            }

            ticks.push(ev);
        }

        BeatSpec {
            ticks,
            beat_len: n_ticks / beats_[0],
            tempo,
        }
    }

    // Creates a BeatSpec from a rhythm specification string.
    pub fn from_rhythmspec(tempo: f64, spec: &str)
                           -> Result<BeatSpec> {
        let mut ticks = vec![];
        let mut beat_len = 1;

        let mut n = 0;
        for c in spec.chars() {
            match c {
                '0'..='9' => {
                    ticks.push(Event::Beep(c as u32 - '0' as u32));
                },
                '.' => {
                    ticks.push(Event::Rest);
                },
                '!' => {
                    beat_len = n;
                },
                _ => {
                    bail!(String::from("Unknown rhythm spec command ")
                          + &String::from(c));
                }
            }
            n += 1;
        }

        Ok(BeatSpec {
            ticks,
            beat_len,
            tempo,
        })
    }

    // Plays a single measure with this BeatSpec.
    pub fn play_measure(&self) {
        // TODO: Implement.
    }
}

// Returns the lowest common multiple of the set of integers.
fn lcm(nums: &[u32]) -> u32 {
    let mut lcm = 1;
    for n in nums {
        lcm = lcm * (*n) / euclid(lcm, *n);
    }

    lcm
}

// Returns the greatest common divisor of two integers (Euclidean
// algorithm).
fn euclid(a: u32, b: u32) -> u32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }

    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subdiv_test() {
        let bs = BeatSpec::from_subdiv(60.0, 3, 2);

        assert_eq!(bs.ticks.len(), 6);
        assert_eq!(bs.beat_len, 2);
        assert_eq!(bs.tempo, 60.0);
    }

    #[test]
    fn crossbeat_test() {
        let bs = BeatSpec::from_crossbeats(60.0, &vec![3, 6]);

        assert_eq!(bs.ticks.len(), 6);
        assert_eq!(bs.beat_len, 2);
        assert_eq!(bs.tempo, 60.0);

        assert_eq!(bs.ticks[0], Event::Beep(0));
        assert_eq!(bs.ticks[1], Event::Beep(2));
        assert_eq!(bs.ticks[2], Event::Beep(1));
        assert_eq!(bs.ticks[3], Event::Beep(2));
        assert_eq!(bs.ticks[4], Event::Beep(1));
        assert_eq!(bs.ticks[5], Event::Beep(2));
    }

    #[test]
    fn rspec_test() {
        let bs = BeatSpec::from_rhythmspec(60.0, "02!1212").unwrap();

        assert_eq!(bs.ticks.len(), 6);
        assert_eq!(bs.beat_len, 2);
        assert_eq!(bs.tempo, 60.0);
    }

    #[test]
    fn lcm_test() {
        assert_eq!(euclid(12, 12), 12);
        assert_eq!(euclid(12, 13), 1);
        assert_eq!(euclid(12, 14), 2);
        assert_eq!(euclid(12, 15), 3);
        assert_eq!(euclid(12, 16), 4);

        assert_eq!(lcm(&vec![12, 12, 13, 14, 15, 16]),
                   12 * 12 * 13 * 14 * 15 * 16
                   / 12 / 1 / 2 / 3 / 4);
    }
}
