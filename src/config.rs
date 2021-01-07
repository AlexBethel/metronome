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

use crate::constants;
use crate::beat_spec::BeatSpec;
use crate::errors::*;
use error_chain::bail;
use getopts::Options;

// Summary of the user's desired configuration for the program.
struct Config {
    // Specification of the rhythm to beat.
    pub rhythm: BeatSpec,
}

// Possible outcomes from parsing a configuration.
enum ConfigResult {
    // Successfully parsed the config.
    Run(Config),

    // The config was well-formed, but it implied that the main
    // program shouldn't be run. This is the case with options like
    // "--help" that do more meta things than program configuration
    // options.
    DontRun,
}

impl Config {
    // Creates a new Config from the program command-line arguments.
    pub fn new(args: &[&str]) -> Result<ConfigResult> {
        let args = &args[1..];

        let mut opts = Options::new();
        opts.optopt(
            "c", "crossbeats",
            "Specifies a polyrhythm of several simultaneous crossbeats.",
            "<cross1>[:<cross2>[...]]");
        opts.optopt(
            "s", "rhythm",
            "Directly specifies a rhythm to beat out.",
            "<rhythm>");
        opts.optflag(
            "h", "help",
            "Prints this help message.");
        opts.optflag(
            "v", "version",
            "Prints the program version and legal info.");

        let matches = opts.parse(args)?;
        if matches.opt_present("h") {
            print_help();
            return Ok(ConfigResult::DontRun);
        }
        if matches.opt_present("v") {
            print_version();
            return Ok(ConfigResult::DontRun);
        }

        let mut beat_spec = parse_free_args(&matches)?;
        if matches.opt_present("c") {
            beat_spec = parse_cross_rhythms(
                beat_spec, &matches.opt_str("c").unwrap())?;
        }
        if matches.opt_present("s") {
            beat_spec = parse_rhythm_string(
                beat_spec, &matches.opt_str("s").unwrap())?;
        }

        return Ok(ConfigResult::Run(Config {
            rhythm: beat_spec,
        }));
    }
}

// Parses all the free arguments to the program. Returns a default
// BeatSpec object, which might be further modified or varied upon by
// the option arguments.
fn parse_free_args(matches: &getopts::Matches) -> Result<BeatSpec> {
    return match matches.free.len() {
        0 => Ok(BeatSpec::from_subdiv(120.0, 4, 1)),
        1 => parse_free_arg(&matches.free[0]),
        _ => {
            print_help();
            bail!("Too many operands");
        }
    }
}

// Parses the free argument to the program (which takes the form
// "<tempo>[:<beats_per_measure>[:<subdivisions_per_beat>]]").
// Returns its corresponding BeatSpec.
fn parse_free_arg(arg: &str) -> Result<BeatSpec> {
    let mut nums = arg.split(':');
    let tempo = nums.next();
    let beats_per_measure = nums.next();
    let subdivisions_per_beat = nums.next();
    if let Some(_) = nums.next() {
        bail!("Unexpected ':' in free arg");
    }

    let tempo: f64 = match tempo {
        Some(x) => x.parse()?,
        None => constants::DEF_TEMPO,
    };
    let beats_per_measure: u32 = match beats_per_measure {
        Some(x) => x.parse()?,
        None => constants::DEF_BEATS_PER_MEASURE,
    };
    let subdivisions_per_beat: u32 = match subdivisions_per_beat {
        Some(x) => x.parse()?,
        None => constants::DEF_SUBDIV_PER_BEAT,
    };

    Ok(BeatSpec::from_subdiv(
        tempo, beats_per_measure, subdivisions_per_beat
    ))
}

// Parses and applies a cross-rhythm string. Returns a modified
// version of the supplied BeatSpec object.
fn parse_cross_rhythms(beat_spec: BeatSpec, cross_str: &str)
                       -> Result<BeatSpec> {
    let mut beats = vec![];
    let beats_str = cross_str.split(':');
    for beat in beats_str {
        beats.push(beat.parse()?);
    }

    Ok(BeatSpec::from_crossbeats(beat_spec.get_tempo(), &beats))
}

// Parses and applies a rhythm specification string. Returns a
// modified version of the supplied BeatSpec object.
fn parse_rhythm_string(beat_spec: BeatSpec, rhythm_str: &str)
                       -> Result<BeatSpec> {
    BeatSpec::from_rhythmspec(beat_spec.get_tempo(), rhythm_str)
}

// Prints the program's usage string.
fn print_help() {
    println!("TODO: Implement usage string");
    // Need accesss to the Options object from here.
}

// Prints the program's version, as well as legal information.
fn print_version() {
    println!("{} version {}",
             constants::NAME, constants::VER);
    println!("Copyright (c) {} by {}. All rights reserved.",
             constants::COPY_YEARS, constants::COPY_AUTHORS);
    println!("{}", constants::LEGAL);
}
