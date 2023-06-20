use sdl2::audio::AudioCallback;
use std::f32::consts::PI;
use std::time::Instant;

use crate::get_s_val;
use crate::sound::Wave;

macro_rules! WAVE {
    ($name: ident, $fn: ident) => {
        pub struct $name {
            phase_inc: f32,
            phase: f32,
            volume: f32,
            now: Instant,
            freq: f32,
        }

        impl $name {
            pub fn new(volume: f32, phase_inc: f32, freq: f32) -> Self {
                Self {
                    phase: 0.0,
                    phase_inc,
                    volume,
                    freq,
                    now: Instant::now(),
                }
            }

            pub fn as_wave(self) -> Wave {
                Wave::$name(self)
            }
        }

        impl AudioCallback for $name {
            fn callback(self: &mut Self, out: &mut [f32]) {
                for x in out.iter_mut() {
                    $fn(
                        self.phase,
                        x,
                        self.volume,
                        self.now.elapsed().as_millis() as u64,
                        self.freq,
                    );
                    self.phase = (self.phase + self.phase_inc) % 1.0
                }
            }

            type Channel = f32;
        }
    };
}

fn square_wave(phase: f32, x: &mut f32, volume: f32, _: u64, _: f32) {
    *x = if phase >= 0.5 { volume } else { -volume };
}
WAVE!(SquareWave, square_wave);

fn sine_wave(phase: f32, x: &mut f32, volume: f32, _: u64, _: f32) {
    *x = phase.sin() * volume * PI;
}
WAVE!(SineWave, sine_wave);

fn sawtooth_wave(phase: f32, x: &mut f32, volume: f32, elapsed_time: u64, freq: f32) {
    *x = (phase * 2.0 - 1.0) * volume
}
WAVE!(SawtoothWave, sawtooth_wave);

fn triangle_wave(phase: f32, x: &mut f32, volume: f32, elapsed_time: u64, freq: f32) {
    *x = (if phase < 0.5 {
        phase * 4.0 - 1.0
    } else {
        (1.0 - phase) * 4.0
    }) * volume
}
WAVE!(TriangleWave, triangle_wave);

fn tilted_triangle_wave(phase: f32, x: &mut f32, volume: f32, elapsed_time: u64, freq: f32) {
    let phase = phase + (phase - 0.5);
    *x = (if phase < 0.5 {
        phase * 4.0 - 1.0
    } else {
        (1.0 - phase) * 4.0
    }) * volume
}
WAVE!(TiltedTriangleWave, tilted_triangle_wave);

fn noise_wave(phase: f32, x: &mut f32, volume: f32, elapsed_time: u64, freq: f32) {
    // do this better
    *x = (rand::random::<f32>() % 1.0) * volume * phase.sin();
}
WAVE!(NoiseWave, noise_wave);

static HARMONIC_COUNT: u8 = 5;

fn organ_wave(phase: f32, x: &mut f32, volume: f32, elapsed_time: u64, freq: f32) {
    let mut value = 0.0;
    let mut ampl = volume;

    for i in 1..=HARMONIC_COUNT {
        value += ampl * (i as f32) * (2.0 * PI * (i as f32) * phase).sin();
        ampl *= 0.5;
    }

    *x = value;
}
WAVE!(OrganWave, noise_wave);
