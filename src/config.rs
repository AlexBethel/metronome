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
        0 => Ok(BeatSpec::from_subdiv(
            constants::DEF_TEMPO,
            constants::DEF_BEATS_PER_MEASURE,
            constants::DEF_SUBDIV_PER_BEAT)),
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

    let tempo = match tempo {
        Some(x) => x.parse()?,
        None => constants::DEF_TEMPO,
    };
    let beats_per_measure = match beats_per_measure {
        Some(x) => x.parse()?,
        None => constants::DEF_BEATS_PER_MEASURE,
    };
    let subdivisions_per_beat = match subdivisions_per_beat {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_test() {
        let default_test = match Config::new(
            &vec!["foo"]).unwrap() {
            ConfigResult::Run(x) => x,
            ConfigResult::DontRun => panic!("Got DontRun"),
        };
        assert_eq!(default_test.rhythm.get_tempo(), constants::DEF_TEMPO);
        assert_eq!(default_test.rhythm.get_beat_len(),
                   constants::DEF_SUBDIV_PER_BEAT);
        assert_eq!(default_test.rhythm.get_ticks().len(),
                   constants::DEF_SUBDIV_PER_BEAT as usize
                   * constants::DEF_BEATS_PER_MEASURE as usize);

        // Test that --help and --version don't start the metronome.
        match Config::new(&vec!["foo", "--help"]).unwrap() {
            ConfigResult::Run(_) => panic!("--help runs metronome"),
            ConfigResult::DontRun => { },
        }
        match Config::new(&vec!["foo", "--version"]).unwrap() {
            ConfigResult::Run(_) => panic!("--version runs metronome"),
            ConfigResult::DontRun => { },
        }

        // Check crossbeats and rhythm specifications.
        let ctest = match Config::new(&vec!["foo", "-c", "2:3"]).unwrap() {
            ConfigResult::Run(x) => x,
            ConfigResult::DontRun => panic!("Got DontRun"),
        };
        assert_eq!(ctest.rhythm.get_tempo(), constants::DEF_TEMPO);
        assert_eq!(ctest.rhythm.get_beat_len(), 3);
        assert_eq!(ctest.rhythm.get_ticks().len(), (2 * 3) as usize);

        let stest = match Config::new(&vec!["foo", "-s", "01!2"]).unwrap() {
            ConfigResult::Run(x) => x,
            ConfigResult::DontRun => panic!("Got DontRun"),
        };
        assert_eq!(stest.rhythm.get_tempo(), constants::DEF_TEMPO);
        assert_eq!(stest.rhythm.get_beat_len(), 2);
        assert_eq!(stest.rhythm.get_ticks().len(), 3);
    }

    #[test]
    fn free_arg_test() {
        // Should default to being in 4, with no beat subdivision.
        let test_1 = parse_free_arg("72").unwrap();
        assert_eq!(test_1.get_tempo(), 72.0);
        assert_eq!(test_1.get_beat_len(), 1);
        assert_eq!(test_1.get_ticks().len(), 4);

        let test_2 = parse_free_arg("72:5:3").unwrap();
        assert_eq!(test_2.get_tempo(), 72.0);
        assert_eq!(test_2.get_beat_len(), 3);
        assert_eq!(test_2.get_ticks().len(), 5 * 3);

        // Extra parameters and invalid numbers should both throw
        // syntax errors.
        let test_invalid = parse_free_arg("72:x:3");
        if let Ok(_) = test_invalid {
            panic!("Valid result from invalid input");
        }

        let test_invalid = parse_free_arg("72:5:3:4");
        if let Ok(_) = test_invalid {
            panic!("Succeeded with too many parameters");
        }
    }

    #[test]
    fn cross_rhythm_parse_test() {
        // Need a template to get the tempo from.
        let tmp = BeatSpec::from_rhythmspec(72.0, "0").unwrap();

        // Use 3 primes to make the math simpler.
        let valid_test = parse_cross_rhythms(tmp, "3:5:17").unwrap();
        assert_eq!(valid_test.get_tempo(), 72.0);
        assert_eq!(valid_test.get_beat_len(), 5 * 17);
        assert_eq!(valid_test.get_ticks().len(), 3 * 5 * 17);

        let invalid_test = parse_cross_rhythms(valid_test, "3:x:17");
        if let Ok(_) = invalid_test {
            panic!("Valid result from invalid input");
        }
    }
}
