use std::time::Instant;

use sdl2::{
    audio::{AudioCallback, AudioDevice, AudioSpecDesired},
    AudioSubsystem,
};

use crate::{
    get_s_val,
    waves::{SawtoothWave, SineWave, SquareWave, TriangleWave, NoiseWave, TiltedTriangleWave, OrganWave},
    SDL_CONTEXT,
};

pub fn update_sounds() {
    unsafe {
        if let Some(sound) = &SOUND {
            if sound.now.elapsed().as_millis() as u64 >= sound.duration {
                if let Some(sound) = SOUND.take() {
                    sound.device.close_and_get_callback();
                }
            }
        }
    }
}

fn get_sdl_audio_context() -> Option<AudioSubsystem> {
    if let Some(ctx) = get_s_val!(SDL_CONTEXT) {
        ctx.audio().ok()
    } else {
        None
    }
}

pub fn stop_sound() {
    unsafe {
        if let Some(sound) = SOUND.take() {
            sound.device.close_and_get_callback();
        }
    }
}

static DESIRED_SPEC: AudioSpecDesired = AudioSpecDesired {
    freq: Some(44100),
    channels: Some(1), // mono
    samples: None,     // default sample size
};

pub enum Wave {
    SquareWave(SquareWave),
    SineWave(SineWave),
    SawtoothWave(SawtoothWave),
    TriangleWave(TriangleWave),
    NoiseWave(NoiseWave),
    TiltedTriangleWave(TiltedTriangleWave),
    OrganWave(OrganWave),
}

impl AudioCallback for Wave {
    fn callback(&mut self, out: &mut [Self::Channel]) {
        match self {
            Self::SquareWave(wave) => wave.callback(out),
            Self::SineWave(wave) => wave.callback(out),
            Self::SawtoothWave(wave) => wave.callback(out),
            Self::TriangleWave(wave) => wave.callback(out),
            Self::TiltedTriangleWave(wave) => wave.callback(out),
            Self::NoiseWave(wave) => wave.callback(out),
            Self::OrganWave(wave) => wave.callback(out),
        }
    }

    type Channel = f32;
}

struct PlayingDevice {
    device: AudioDevice<Wave>,
    duration: u64,
    now: Instant,
}

unsafe impl Sync for PlayingDevice {}

static mut SOUND: Option<PlayingDevice> = None;

macro_rules! play_wave_function {
    ($type: ident, $name: tt) => {
        pub fn $name(freq: f32, volume: f32, ms: u64) {
            if unsafe { SOUND.is_some() } {
                return;
            }
            if let Some(ctx) = get_sdl_audio_context() {
                let device = ctx.open_playback(None, &DESIRED_SPEC, |spec| {
                    $type::new(
                        volume * (get_volume() as f32 / 100.0),
                        freq / spec.freq as f32,
                        freq,
                    )
                    .as_wave()
                });
                if let Ok(device) = device {
                    device.resume();
                    unsafe {
                        SOUND = Some(PlayingDevice {
                            device,
                            duration: ms,
                            now: Instant::now(),
                        });
                    }
                }
            }
        }
    };
}

static mut VOLUME: u8 = 100;
static mut MUTED: bool = false;

pub fn is_muted() -> bool {
    unsafe { MUTED }
}

pub fn toggle_muted(value: Option<bool>) {
    unsafe {
        MUTED = value.unwrap_or(!MUTED);
    }
}

pub fn get_volume() -> u8 {
    unsafe {
        if MUTED {
            0
        } else {
            VOLUME
        }
    }
}

pub fn set_volume(mut vol: u8) {
    if vol > 100 {
        vol = 100;
    }
    unsafe { VOLUME = vol }
}

play_wave_function!(SquareWave, squarewave);
play_wave_function!(SineWave, sinewave);
play_wave_function!(SawtoothWave, sawtoothwave);
play_wave_function!(TriangleWave, trianglewave);
play_wave_function!(TiltedTriangleWave, tiltedtrianglewave);
play_wave_function!(NoiseWave, noisewave);
play_wave_function!(OrganWave, organwave);