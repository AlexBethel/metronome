// Compile-time constants.
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

// ---- Meta ----

// Name of the project.
pub const NAME: &str = "metronome";

// Program version. (Will bump to 0.1.0 when calling main() actually
// does something.) Try and keep this in sync with Git tags.
pub const VER: &str = "0.0.0";

// Program copyright year(s) and author(s).
pub const COPY_YEARS: &str = "2021";
pub const COPY_AUTHORS: &str = "Alexander Bethel";

// Program license information.
pub const LEGAL: &str = "Licensed under the GNU GPLv3.0.";

// ---- Default options ----

// Default tempo, beats per measure & subdivisions per beat.
pub const DEF_TEMPO: f64 = 120.0;
pub const DEF_BEATS_PER_MEASURE: u32 = 4;
pub const DEF_SUBDIV_PER_BEAT: u32 = 1;

// ---- Sound options ----

// Length of a beep, in milliseconds.
pub const BEAT_LEN: u64 = 150;

// Pitch of the highest beep the metronome produces.
pub const BEEP_PITCH: f64 = 880.0;
