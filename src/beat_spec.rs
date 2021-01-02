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
struct TimedEvent {
    // Time measured in beats.
    time: f64,

    event: Event,
}

// Different types of events that can occur in a measure.
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
    pub fn from_crossbeats(_tempo: f64, _beats: &[u32]) -> BeatSpec {
        // TOOD: Implement.
        BeatSpec {
            events: vec![],
            measure_len: 1.0,
            tempo: 60.0,
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
