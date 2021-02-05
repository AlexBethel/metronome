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

use colorful::Color;

// ---- Meta ----

// Name of the project.
pub const NAME: &str = "metronome";

// Program version. Try and keep this in sync with Git tags.
pub const VER: &str = "0.1.3";

// Program copyright year(s) and author(s).
pub const COPY_YEARS: &str = "2021";
pub const COPY_AUTHORS: &str = "Alexander Bethel";

// Program license information.
pub const LEGAL: &str = "Licensed under the GNU GPLv3.

This program comes with ABSOLUTELY NO WARRANTY. This is free software,
and you are welcome to redistribute it under certain conditions; see
the included LICENSE file for details.";

// ---- Defaults for user-adjustable options ----

// Default tempo, beats per measure & subdivisions per beat.
pub const DEF_TEMPO: f64 = 120.0;
pub const DEF_BEATS_PER_MEASURE: u32 = 4;
pub const DEF_SUBDIV_PER_BEAT: u32 = 1;

// Default volume of beeps, from 0.0 to 1.0.
pub const DEF_VOLUME: f64 = 0.5;

// ---- Sound options ----

// Length of a beep, in milliseconds.
pub const BEAT_LEN: u64 = 150;

// Pitch of the highest beep the metronome produces.
pub const BEEP_PITCH: f64 = 880.0;

// ---- Controller options ----

// Measure by which volume is adjusted per press of the volume
// increase or decrease button. This is on a scale where 0.0 is silent
// and 1.0 is max volume.
pub const VOL_ADJUST: f64 = 0.1;

// Volumen minimum and maximum.
pub const VOL_MIN: f64 = 0.0;
pub const VOL_MAX: f64 = 1.0;

// Measure by which tempo is adjusted per press of the tempo increase
// or decrease button, in beats per minute.
pub const TEMPO_ADJUST: f64 = 1.0;

// Tempo minimum and maximum.
pub const TEMPO_MIN: f64 = 10.0;
pub const TEMPO_MAX: f64 = 300.0;

// ---- View options ----

// Width of the tempo and volume indicators.
pub const NUM_INDIC_WIDTH: usize = 3;

// Width of the measure progress indicator.
pub const MEAS_INDIC_WIDTH: usize = 40;

// Color scheme.
pub const BRACKET_COLOR: Color = Color::Yellow;
pub const TEMPO_COLOR: Color = Color::LightBlue;
pub const PROGRESS_COLOR: Color = Color::Green;
pub const VOLUME_COLOR: Color = Color::LightRed;
