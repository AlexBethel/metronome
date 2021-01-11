// Code for emitting dynamically-generated tones.
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
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, StreamConfig};
use std::ops::Deref;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

// Since AudioConfigInternal is not Clone (because Device is not
// Clone), we use reference counting to ensure its data can be passed
// between threads.
#[derive(Clone)]
pub struct AudioConfig {
    // Note that this struct implements Deref, so you can write
    // "audio_config.device" rather than needing to spell out the
    // literal path "(*audio_config.cfg).device".
    cfg: Arc<AudioConfigInternal>,
}

impl AudioConfig {
    pub fn new() -> Result<Self> {
        Ok(AudioConfig {
            cfg: Arc::new(AudioConfigInternal::new()?),
        })
    }
}

impl Deref for AudioConfig {
    type Target = Arc<AudioConfigInternal>;

    fn deref(&self) -> &Arc<AudioConfigInternal> {
        &self.cfg
    }
}

// Context used to play audio. In general, one of these should be
// prepared at the start of the program, and it should live for the
// entire duration of the program.
pub struct AudioConfigInternal {
    device: Device,
    stream_config: StreamConfig,
}

impl AudioConfigInternal {
    pub fn new() -> Result<Self> {
        let host = cpal::default_host();
        let device = match host.default_output_device() {
            Some(dev) => dev,
            None => {
                return Err(ErrorKind::AudioConfig("No audio device found".to_string()).into());
            }
        };

        let mut supported_cfg_range = device.supported_output_configs()?;

        let stream_config = match supported_cfg_range.next() {
            Some(cfg) => cfg.with_max_sample_rate().config(),
            None => {
                return Err(
                    ErrorKind::AudioConfig("No supported configurations".to_string()).into(),
                );
            }
        };

        Ok(Self {
            device,
            stream_config,
        })
    }
}

// Plays a beep at the given frequency, for the given length of time.
// The sound is played in another thread, so this function does not
// block.
pub fn beep(frequency: f64, length: Duration, cfg: &AudioConfig) {
    let cfg = cfg.clone();
    thread::spawn(move || {
        let omega = frequency * std::f64::consts::TAU / cfg.stream_config.sample_rate.0 as f64;
        let mut theta: f64 = 0.0;
        let stream = cfg.device.build_output_stream(
            &cfg.stream_config,
            move |data: &mut [f32], _info: &cpal::OutputCallbackInfo| {
                for el in data {
                    *el = theta.sin() as f32;
                    theta += omega;
                }
            },
            move |_err| {
                panic!("Stream error");
            },
        );

        thread::sleep(length);
        drop(stream);
    });
}
