# metronome

A metronome controlled by the command line, motivated primarily as an
exercise for learning the Rust programming language. As such, I've
prioritized clean, simple and idiomatic code above advanced and
diverse features. Admittedly, I've very new to Rust, so I won't
pretend I've done a spectacular job of that.

## Installing

First, make sure you have Rust installed -- if `which cargo` doesn't
find a match, run this command to install the Rust toolchain for your
user account:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Now, run `cargo install --path .` to compile and install the program,
or `cargo build` to just build the executable in the `target/debug/`
directory.

## Invocation

`metronome <tempo>[:<beats_per_measure>[:<subdivisions_per_beat>]]`

Starts the metronome at the given tempo (specified in beats per
minute), and with measures defined using the optional parameters
(defaults are 4 beats per measure, one subdivision per beat). For
example, `metronome 140:7:3` plays measures with 7 quarter notes at
140 quarter notes per minute, each of which is subdivided into
triplets.

`metronome -c <cross1>[:<cross2>[...]] <tempo>`

Runs the metronome with several cross rhythms running at once. Here,
the `tempo` value refers to the time for one beat of the first cross
rhythm specified; so, for example, `-c 4:3 150` plays quarter notes at
150 beats per minute, interspersed with three evenly-spaced notes for
every four quarter notes; and `-c 3:4 150` plays "third notes" at 150
beats per minute, interspersed with four evenly-spaced notes for every
three "third notes".

`metronome -s <rhythm> <tempo>`

Runs the metronome with the given custom rhythm. This invocation forms
a superset of the other two invocations, but is less easy to use. See
the section on "Rhythm specification" for more information.

### Other options

* `-h`, `--help`: Prints a help string.
* `-v`, `--version`: Prints the program version.
* `-l`, `--volume <volume>`: Sets the initial volume, out of 100.

## Rhythm specification

Rhythms are specified as strings of commands, where each command
represents the action taken during an evenly-spaced "tick" (not to be
confused with "beats", which are a length of time used in the tempo
calculation). Commands are normally single characters, and are written
without any sort of separator between them; so "12.3" is a string of
four commands: "1", "2", "." and "3".

The valid commands are:
* A single period ".". This represents a rest; the metronome takes no
  action during such a beat.
* Any of the digits 0-9. The metronome briefly plays a tone whose
  frequency is derived from the digit's value, where higher digits are
  lower tones. This effect is generally used to adjust the relative
  emphasis of beats.
* An exclamation mark "!", which modifies another immediately
  following command. This marks the length of a beat, relative to the
  start of the rhythm; so, for example, in "0..!1..", the exclamation
  mark denotes that the time between the beginning of the pattern and
  the "1" (i.e., 3 ticks) is equal to one beat.

## User interface

The metronome can be controlled to some degree via the keyboard. The
key bindings are:
* "p": Pause
* "P": Play
* " " (space): Toggle between playing and paused
* Up, "k", "C-p": Increase volume
* Down, "j", "C-n": Decrease volume
* Right, "l", "C-b": Increase tempo
* Left, "h", "C-f": Decrease tempo
* "q": Quit
* "'": Prompts for a tempo, and sets the metronome's tempo
  accordingly.
* ".": Immediately synchronizes the metronome such that a measure
  begins the instant the key was pressed.
* ",": Enter "tap" mode. Pressing this key repeatedly causes the
  metronome to match the tempo you tap at; press any other key to
  leave this mode and begin ticking again.
