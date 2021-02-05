// Code for managing the state of the appolication, and the main
// execution loop.
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
use std::io::{stdin, Read};
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::{Duration, Instant};

// Possible state of the application at any given time.
pub trait AppState {
    // Runs one timer tick of the application.
    fn tick(&mut self) -> (StateTransition, TickCommand);

    // Interprets a key-press, given the amount of time since the last
    // event.
    fn keypress(&mut self, key: Keycode, time: Duration) -> (StateTransition, TickCommand);
}

// A transition from one application state to another.
pub enum StateTransition {
    // Keep the current program state.
    NoChange,

    // Quit the program.
    Exit,

    // Set the program state to the given value.
    To(Box<dyn AppState>),
}

// A command for the tick manager.
#[derive(Debug)]
pub enum TickCommand {
    // Leaves the tick manager in its current state.
    None,

    // Signals that a tick should occur for this state in the given
    // amount of time.
    Set(Duration),

    // Sets the tick manager to never run; roughly equivalent to
    // Set(infinity).
    Unset,

    // Pauses the tick manager if it was running; does nothing
    // otherwise.
    Pause,

    // Resumes the tick manager if it was not running; does nothing
    // otherwise. Panics if no duration was previously initialized
    // with Set().
    Resume,

    // Resumes the tick manager if it was paused and pauses it if it
    // was resumed.
    Toggle,
}

// Outputs from the keyboard thread.
pub enum Keycode {
    // Successfully received a key, here it is as a raw u8 byte.
    Key(u8),

    // Failed to receive a key, probably because stdin closed.
    NoKey,
}

// Runs the main program loop, given the initial state.
pub fn state_loop(init_state: Box<dyn AppState>) -> Result<()> {
    let kbd = init_kbd_thread();

    let mut state = init_state;
    let mut tick_time: Option<Duration> = Some(Duration::new(0, 0));
    let mut paused = false;

    let mut exit = false;
    while !exit {
        let start_time = Instant::now();
        let key = if !paused {
            if let Some(tick_time) = tick_time {
                kbd.recv_timeout(tick_time)
            } else {
                Ok(kbd.recv()?)
            }
        } else {
            Ok(kbd.recv()?)
        };

        let (st, tc) = if let Ok(key) = key {
            let tmp = state.keypress(key, start_time.elapsed());
            if let Some(tick_time_unwrapped) = tick_time {
                if !paused {
                    tick_time = match tick_time_unwrapped.checked_sub(start_time.elapsed()) {
                        Some(time) => Some(time),
                        None => Some(Duration::new(0, 0)),
                    };
                }
            }
            tmp
        } else {
            state.tick()
        };
        proc_transition(st, tc, &mut state, &mut tick_time, &mut paused, &mut exit);
    }

    Ok(())
}

// Processes a set of transition commands on the program state.
fn proc_transition(
    st: StateTransition,
    tc: TickCommand,
    state: &mut Box<dyn AppState>,
    tick_time: &mut Option<Duration>,
    paused: &mut bool,
    exit: &mut bool,
) {
    match st {
        StateTransition::NoChange => {}
        StateTransition::Exit => {
            *exit = true;
        }
        StateTransition::To(new_state) => {
            *state = new_state;
        }
    };

    match tc {
        TickCommand::None => {}
        TickCommand::Set(d) => {
            *tick_time = Some(d);
            *paused = false;
        }
        TickCommand::Unset => {
            *tick_time = None;
            *paused = false;
        }
        TickCommand::Pause => {
            *paused = true;
        }
        TickCommand::Resume => {
            *paused = false;
        }
        TickCommand::Toggle => {
            *paused = !*paused;
        }
    };
}

// Sets up a keyboard thread, and returns a receiver for keystrokes;
// None is sent when stdin closes or an input error occurs, and
// further messages should not be read by the caller.
fn init_kbd_thread() -> Receiver<Keycode> {
    let (send, recv) = channel();

    thread::spawn(move || {
        use Keycode::*;
        let mut input = stdin();

        loop {
            let mut buf = vec![0];
            match input.read_exact(&mut buf) {
                Err(_) => {
                    send.send(NoKey).unwrap();
                    return;
                }
                Ok(_) => {
                    send.send(Key(buf[0])).unwrap();
                }
            }
        }
    });

    return recv;
}
