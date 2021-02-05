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
use crate::constants;
use crate::errors::*;
use error_chain::bail;
use getopts::Options;

// Summary of the user's desired configuration for the program.
pub struct Config {
    // Specification of the rhythm to beat.
    pub rhythm: BeatSpec,

    // The initial tempo to beat at.
    pub tempo: f64,

    // The initial volume.
    pub volume: f64,
}

// Possible outcomes from parsing a configuration.
pub enum ConfigResult {
    // Successfully parsed the config.
    Run(Config),

    // The config was well-formed, but it implied that the main
    // program shouldn't be run. This is the case with options like
    // "--help" that do more meta things than program configuration
    // options.
    DontRun,
}

// Command-line usage switch.
enum CmdSwitch {
    // A switch taking an argument.
    Option {
        short_name: &'static str,
        long_name: &'static str,
        description: &'static str,
        example: &'static str,

        action: &'static dyn Fn(&str, &mut Config, &Options) -> Result<Option<ConfigResult>>,
    },

    // A switch that does not have an argument.
    Flag {
        short_name: &'static str,
        long_name: &'static str,
        description: &'static str,

        action: &'static dyn Fn(&mut Config, &Options) -> Result<Option<ConfigResult>>,
    },
}

impl Config {
    // Creates a new Config from the program command-line arguments.
    pub fn new(args: &[&str]) -> Result<ConfigResult> {
        let args = &args[1..];

        let opts = compile_opts(SWITCHES);

        let matches = opts.parse(args)?;
        let mut cfg = parse_free_args(&matches, &opts)?;

        for switch in SWITCHES {
            let short_name = match switch {
                CmdSwitch::Option { short_name, .. } => short_name,
                CmdSwitch::Flag { short_name, .. } => short_name,
            };

            if matches.opt_present(short_name) {
                let res = match switch {
                    CmdSwitch::Option { action, .. } => {
                        action(&matches.opt_str(short_name).unwrap(), &mut cfg, &opts)
                    }
                    CmdSwitch::Flag { action, .. } => action(&mut cfg, &opts),
                }?;

                if let Some(v) = res {
                    return Ok(v);
                }
            }
        }

        return Ok(ConfigResult::Run(cfg));
    }
}

// Parses all the free arguments to the program. Returns a default
// BeatSpec object, which might be further modified or varied upon by
// the option arguments.
fn parse_free_args(matches: &getopts::Matches, opts: &Options) -> Result<Config> {
    return match matches.free.len() {
        0 => Ok(Config {
            rhythm: BeatSpec::from_subdiv(
                constants::DEF_BEATS_PER_MEASURE,
                constants::DEF_SUBDIV_PER_BEAT,
            ),
            tempo: constants::DEF_TEMPO,
            volume: constants::DEF_VOLUME,
        }),
        1 => parse_free_arg(&matches.free[0]),
        _ => {
            print_help(opts);
            bail!("Too many operands");
        }
    };
}

// Parses the free argument to the program (which takes the form
// "<tempo>[:<beats_per_measure>[:<subdivisions_per_beat>]]").
// Returns its corresponding BeatSpec.
fn parse_free_arg(arg: &str) -> Result<Config> {
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
    let volume = constants::DEF_VOLUME;

    Ok(Config {
        rhythm: BeatSpec::from_subdiv(beats_per_measure, subdivisions_per_beat),
        tempo,
        volume,
    })
}

// Compiles a set of options in our format to the getopt::Options
// format.
fn compile_opts(switches: &[CmdSwitch]) -> Options {
    let mut opts = Options::new();
    for opt in switches {
        match opt {
            CmdSwitch::Option {
                short_name,
                long_name,
                description,
                example,
                action: _,
            } => {
                opts.optopt(short_name, long_name, description, example);
            }
            CmdSwitch::Flag {
                short_name,
                long_name,
                description,
                action: _,
            } => {
                opts.optflag(short_name, long_name, description);
            }
        }
    }

    opts
}

// The switches the program checks for.
const SWITCHES: &[CmdSwitch] = &[
    CmdSwitch::Option {
        short_name: "c",
        long_name: "crossbeat",
        description: "Specifies a polyrhythm of several simultaneous crossbeats.",
        example: "<cross1>[:<cross2>[...]]",

        action: &opt_crossbeat,
    },
    CmdSwitch::Option {
        short_name: "s",
        long_name: "rhythm",
        description: "Directly specifies a rhythm to beat out.",
        example: "<rhythm>",

        action: &opt_rhythm,
    },
    CmdSwitch::Option {
        short_name: "l",
        long_name: "volume",
        description: "Sets the initial volume, out of 100.",
        example: "<volume>",

        action: &opt_volume,
    },
    CmdSwitch::Flag {
        short_name: "h",
        long_name: "help",
        description: "Prints this help message.",

        action: &flag_help,
    },
    CmdSwitch::Flag {
        short_name: "v",
        long_name: "version",
        description: "Prints the program version and legal info.",

        action: &flag_version,
    },
];

fn opt_crossbeat(arg: &str, config: &mut Config, _opts: &Options) -> Result<Option<ConfigResult>> {
    config.rhythm = parse_cross_rhythms(arg)?;
    Ok(None)
}

fn opt_rhythm(arg: &str, config: &mut Config, _opts: &Options) -> Result<Option<ConfigResult>> {
    config.rhythm = parse_rhythm_string(arg)?;
    Ok(None)
}

fn opt_volume(arg: &str, config: &mut Config, _opts: &Options) -> Result<Option<ConfigResult>> {
    config.volume = arg.parse::<f64>()? / 100.0;
    Ok(None)
}

fn flag_help(_config: &mut Config, opts: &Options) -> Result<Option<ConfigResult>> {
    print_help(opts);
    Ok(Some(ConfigResult::DontRun))
}

fn flag_version(_config: &mut Config, _opts: &Options) -> Result<Option<ConfigResult>> {
    print_version();
    Ok(Some(ConfigResult::DontRun))
}

// Parses and applies a cross-rhythm string. Returns a modified
// version of the supplied BeatSpec object.
fn parse_cross_rhythms(cross_str: &str) -> Result<BeatSpec> {
    let mut beats = vec![];
    let beats_str = cross_str.split(':');
    for beat in beats_str {
        beats.push(beat.parse()?);
    }

    Ok(BeatSpec::from_crossbeats(&beats))
}

// Parses and applies a rhythm specification string. Returns a
// modified version of the supplied BeatSpec object.
fn parse_rhythm_string(rhythm_str: &str) -> Result<BeatSpec> {
    BeatSpec::from_rhythmspec(rhythm_str)
}

// Prints the program's usage string.
fn print_help(opts: &Options) {
    let brief = format!(
        "Usage: {} [<options> ...] [<tempo>[:<beats>[:<subdiv>]]]",
        constants::NAME
    );
    print!("{}", opts.usage(&brief));
}

// Prints the program's version, as well as legal information.
fn print_version() {
    println!("{} version {}", constants::NAME, constants::VER);
    println!(
        "Copyright (c) {} by {}. All rights reserved.",
        constants::COPY_YEARS,
        constants::COPY_AUTHORS
    );
    println!("{}", constants::LEGAL);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_test() {
        let default_test = match Config::new(&vec!["foo"]).unwrap() {
            ConfigResult::Run(x) => x,
            ConfigResult::DontRun => panic!("Got DontRun"),
        };
        assert_eq!(default_test.tempo, constants::DEF_TEMPO);
        assert_eq!(
            default_test.rhythm.get_beat_len(),
            constants::DEF_SUBDIV_PER_BEAT
        );
        assert_eq!(
            default_test.rhythm.get_ticks().len(),
            (constants::DEF_SUBDIV_PER_BEAT * constants::DEF_BEATS_PER_MEASURE) as usize
        );

        // Test that --help and --version don't start the metronome.
        match Config::new(&vec!["foo", "--help"]).unwrap() {
            ConfigResult::Run(_) => panic!("--help runs metronome"),
            ConfigResult::DontRun => {}
        }
        match Config::new(&vec!["foo", "--version"]).unwrap() {
            ConfigResult::Run(_) => panic!("--version runs metronome"),
            ConfigResult::DontRun => {}
        }

        // Check crossbeats and rhythm specifications.
        let ctest = match Config::new(&vec!["foo", "-c", "2:3"]).unwrap() {
            ConfigResult::Run(x) => x,
            ConfigResult::DontRun => panic!("Got DontRun"),
        };
        assert_eq!(ctest.tempo, constants::DEF_TEMPO);
        assert_eq!(ctest.rhythm.get_beat_len(), 3);
        assert_eq!(ctest.rhythm.get_ticks().len(), (2 * 3) as usize);

        let stest = match Config::new(&vec!["foo", "-s", "01!2"]).unwrap() {
            ConfigResult::Run(x) => x,
            ConfigResult::DontRun => panic!("Got DontRun"),
        };
        assert_eq!(stest.tempo, constants::DEF_TEMPO);
        assert_eq!(stest.rhythm.get_beat_len(), 2);
        assert_eq!(stest.rhythm.get_ticks().len(), 3);
    }

    #[test]
    fn free_arg_test() {
        // Should default to being in 4, with no beat subdivision.
        let test_1 = parse_free_arg("72").unwrap();
        assert_eq!(test_1.tempo, 72.0);
        assert_eq!(test_1.rhythm.get_beat_len(), constants::DEF_SUBDIV_PER_BEAT);
        assert_eq!(
            test_1.rhythm.get_ticks().len(),
            (constants::DEF_BEATS_PER_MEASURE * constants::DEF_SUBDIV_PER_BEAT) as usize
        );

        let test_2 = parse_free_arg("72:5:3").unwrap();
        assert_eq!(test_2.tempo, 72.0);
        assert_eq!(test_2.rhythm.get_beat_len(), 3);
        assert_eq!(test_2.rhythm.get_ticks().len(), 5 * 3);

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
        // Use 3 primes to make the math simpler.
        let valid_test = parse_cross_rhythms("3:5:17").unwrap();
        assert_eq!(valid_test.get_beat_len(), 5 * 17);
        assert_eq!(valid_test.get_ticks().len(), 3 * 5 * 17);

        let invalid_test = parse_cross_rhythms("3:x:17");
        if let Ok(_) = invalid_test {
            panic!("Valid result from invalid input");
        }
    }
}
