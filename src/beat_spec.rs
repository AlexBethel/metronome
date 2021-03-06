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

use crate::errors::*;
use error_chain::bail;
use std::convert::TryInto;

// Description of precisely what events should occur and when during a
// single measure.
#[derive(Debug, Clone)]
pub struct BeatSpec {
    // The set of events to run during each tick in a measure.
    ticks: Vec<Event>,

    // Length of a beat, in ticks.
    beat_len: u32,
}

// Different types of events that can occur in a measure.
#[derive(Debug, PartialEq, Clone)]
pub enum Event {
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
    pub fn from_subdiv(beats: u32, subdiv: u32) -> BeatSpec {
        Self::from_crossbeats(&vec![beats, beats * subdiv])
    }

    // Creates a BeatSpec given a set of simultaneous cross-rhythms,
    // specified in order of decreasing emphasis.
    pub fn from_crossbeats(beats: &[u32]) -> BeatSpec {
        // Add an implicit crossbeat of 1, so we get a high-pitched
        // beep at the start of each measure.
        let beats = &{
            let mut tmp = vec![1];
            tmp.extend_from_slice(beats);
            tmp
        };

        let n_ticks = lcm(&beats);
        let mut ticks = vec![];
        ticks.reserve(n_ticks.try_into().unwrap());

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
            beat_len: n_ticks / beats[1],
        }
    }

    // Creates a BeatSpec from a rhythm specification string.
    pub fn from_rhythmspec(spec: &str) -> Result<BeatSpec> {
        let mut ticks = vec![];
        ticks.reserve(spec.len());
        let mut beat_len = 1;

        let mut n = 0;
        for c in spec.chars() {
            match c {
                '0'..='9' => {
                    ticks.push(Event::Beep(c as u32 - '0' as u32));
                }
                '.' => {
                    ticks.push(Event::Rest);
                }
                '!' => {
                    beat_len = n;
                }
                _ => {
                    bail!(String::from("Unknown rhythm spec command ") + &String::from(c));
                }
            }
            n += 1;
        }

        Ok(BeatSpec { ticks, beat_len })
    }

    // Constructs a BeatSpec with the same content as this one, but
    // subdivided to be divisible by the given integer. E.g., given a
    // 4/4 measure of quarter notes and the parameter 8, this returns
    // an 8/8 measure with rests on the off-beats, which sounds
    // exactly the same as the 4/4 measure.
    pub fn make_divisible(&self, value: u32) -> BeatSpec {
        let n_ticks = self.beat_len as u32;
        let factor = value / euclid(n_ticks, value);

        BeatSpec {
            ticks: {
                let mut v = Vec::new();
                v.reserve((n_ticks * factor) as usize);
                for b in self.ticks.iter() {
                    v.push(b.clone());
                    for _ in 1..factor {
                        v.push(Event::Rest);
                    }
                }
                v
            },
            beat_len: self.beat_len * factor,
        }
    }

    // Accessor functions
    pub fn get_ticks(&self) -> &[Event] {
        &self.ticks
    }

    pub fn get_beat_len(&self) -> u32 {
        self.beat_len
    }
}

// Returns the lowest common multiple of the set of integers.
fn lcm(nums: &[u32]) -> u32 {
    let mut lcm = 1;
    for n in nums {
        lcm *= n / euclid(lcm, *n);
    }

    lcm
}

// Returns the greatest common divisor of two integers (Euclidean
// algorithm).
fn euclid(a: u32, b: u32) -> u32 {
    let (mut a, mut b) = (a, b);
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
        let bs = BeatSpec::from_subdiv(3, 2);

        assert_eq!(bs.ticks.len(), 6);
        assert_eq!(bs.beat_len, 2);
    }

    #[test]
    fn crossbeat_test() {
        let bs = BeatSpec::from_crossbeats(&vec![3, 6]);

        assert_eq!(bs.ticks.len(), 6);
        assert_eq!(bs.beat_len, 2);

        assert_eq!(bs.ticks[0], Event::Beep(0));
        assert_eq!(bs.ticks[1], Event::Beep(2));
        assert_eq!(bs.ticks[2], Event::Beep(1));
        assert_eq!(bs.ticks[3], Event::Beep(2));
        assert_eq!(bs.ticks[4], Event::Beep(1));
        assert_eq!(bs.ticks[5], Event::Beep(2));
    }

    #[test]
    fn rspec_test() {
        let bs = BeatSpec::from_rhythmspec("02!1212").unwrap();

        assert_eq!(bs.ticks.len(), 6);
        assert_eq!(bs.beat_len, 2);
    }

    #[test]
    fn lcm_test() {
        assert_eq!(euclid(12, 12), 12);
        assert_eq!(euclid(12, 13), 1);
        assert_eq!(euclid(12, 14), 2);
        assert_eq!(euclid(12, 15), 3);
        assert_eq!(euclid(12, 16), 4);

        assert_eq!(
            lcm(&vec![12, 12, 13, 14, 15, 16]),
            12 * 12 * 13 * 14 * 15 * 16 / 12 / 1 / 2 / 3 / 4
        );
    }
}
