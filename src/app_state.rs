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
use std::sync::mpsc::{channel, Receiver, RecvTimeoutError};
use std::thread;
use std::time::{Duration, Instant};

// Possible state of the application at any given time.
pub trait AppState {
    // Runs one timer tick of the application.
    fn tick(&mut self, mgr: &mut StateManager);

    // Interprets a key-press, given the amount of time since the last
    // event.
    fn keypress(&mut self, mgr: &mut StateManager, key: Keycode, time: Duration);
}

// Data structure for storing and updating the current application
// state.
pub struct StateManager {
    // The state the program should transition to at the next loop.
    next_state: StateTransition,

    // Time at which the state next requires an update.
    tick_time: TickTime,
}

enum StateTransition {
    // No change.
    None,

    // Exit the program.
    Exit,

    // Switch to this state.
    To(Box<dyn AppState>),
}

// Time at which the AppState next requires a tick.
enum TickTime {
    // Never tick the AppState.
    None,

    // Tick the AppState at this time.
    Time(Instant),

    // The timer is paused, with this amount left.
    Paused(Duration),
}

// Outputs from the keyboard thread.
pub enum Keycode {
    // Successfully received a key, here it is as a raw u8 byte.
    Key(u8),

    // Failed to receive a key, probably because stdin closed.
    NoKey,
}

impl StateManager {
    // Creates a new StateManager, given the initial state.
    pub fn new(init_state: Box<dyn AppState>) -> Self {
        Self {
            next_state: StateTransition::To(init_state),

            // Schedule an immediate tick for the new state to
            // initialize.
            tick_time: TickTime::Time(Instant::now()),
        }
    }

    // Runs the main program loop.
    pub fn state_loop(mut self) -> Result<()> {
        let kbd = init_kbd_thread();

        let mut state_opt = self.next_state;
        self.next_state = StateTransition::None;

        while let StateTransition::To(ref mut state) = state_opt {
            let start_time = Instant::now();
            let key = if let TickTime::Time(tick_time) = self.tick_time {
                let remaining = tick_time.checked_duration_since(start_time);
                let k = if let Some(t) = remaining {
                    kbd.recv_timeout(t)
                } else {
                    // We're behind schedule; immediately time out.
                    Err(RecvTimeoutError::Timeout)
                };

                if matches!(Instant::now().checked_duration_since(tick_time), Some(_)) {
                    self.tick_time = TickTime::None;
                }

                k
            } else {
                Ok(kbd.recv()?)
            };

            if let Ok(key) = key {
                state.keypress(&mut self, key, start_time.elapsed());
            } else {
                state.tick(&mut self);
            };

            if matches!(self.next_state, StateTransition::To(_) | StateTransition::Exit) {
                state_opt = self.next_state;
                self.next_state = StateTransition::None;
            }
        }

        Ok(())
    }

    // Schedules the program to exit in the next state loop.
    pub fn exit(&mut self) {
        self.next_state = StateTransition::Exit;
    }

    // Sets the program state.
    pub fn set_state(&mut self, new_state: Box<dyn AppState>) {
        self.next_state = StateTransition::To(new_state);

        // Schedule an immediate tick for initialization.
        self.tick_time = TickTime::Time(Instant::now());
    }

    // Schedules a tick for the current state in the given duration.
    pub fn set_tick(&mut self, duration: Duration) {
        // BUG: This loses precision over long periods of time; make
        // it dependent on self.tick_time if that doesn't cause
        // issues.
        self.tick_time = TickTime::Time(Instant::now() + duration);
    }

    // Cancels a scheduled tick.
    pub fn unset_tick(&mut self) {
        self.tick_time = TickTime::None;
    }

    // Pauses the tick timer.
    pub fn pause(&mut self) {
        if let TickTime::Time(time) = self.tick_time {
            let remaining = match time.checked_duration_since(Instant::now()) {
                Some(x) => x,
                None => Duration::new(0, 0),
            };

            self.tick_time = TickTime::Paused(remaining);
        }
    }

    // Resumes a paused tick timer.
    pub fn resume(&mut self) {
        if let TickTime::Paused(remaining) = self.tick_time {
            self.tick_time = TickTime::Time(Instant::now() + remaining);
        } else {
            panic!("Can't resume an unpaused timer");
        }
    }

    // Toggles the tick timer between running and paused.
    pub fn toggle_paused(&mut self) {
        if matches!(self.tick_time, TickTime::Paused(_)) {
            self.resume();
        } else if matches!(self.tick_time, TickTime::Time(_)) {
            self.pause();
        } else {
            panic!("Timer does not exist, so it cannot be toggled");
        }
    }
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
