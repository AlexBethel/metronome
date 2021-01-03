// Command-line option parsing, and translation to the more useful
// Config data structure.
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

use crate::beat_spec::BeatSpec;

// Summary of the user's desired configuration for the program.
struct Config {
    // The tempo at which the metronome clicks, in beats per minute.
    tempo: f64,

    // Specification of the rhythm to beat.
    rhythm: BeatSpec,
}
