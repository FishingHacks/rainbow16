use crate::audio::WaveType;
use std::f32::consts::PI;

fn square_wave(phase: f32, x: &mut f32, volume: f32) {
    *x = if phase >= 0.5 { volume } else { -volume };
}

fn sine_wave(phase: f32, x: &mut f32, volume: f32) {
    *x = phase.sin() * volume * PI;
}

fn sawtooth_wave(phase: f32, x: &mut f32, volume: f32) {
    *x = (phase * 2.0 - 1.0) * volume
}

fn triangle_wave(phase: f32, x: &mut f32, volume: f32) {
    *x = (if phase < 0.5 {
        phase * 4.0 - 1.0
    } else {
        (1.0 - phase) * 4.0
    }) * volume
}

fn tilted_sawtooth_wave(phase: f32, x: &mut f32, volume: f32) {
    let phase = phase + (phase - 0.5);
    *x = (if phase < 0.5 {
        phase * 4.0 - 1.0
    } else {
        (1.0 - phase) * 4.0
    }) * volume
}

static mut LAST_NOISE: f32 = 0.0;
fn noise_wave(phase: f32, x: &mut f32, volume: f32, phase_inc: f32) {
    if phase < phase_inc {
        unsafe {
            LAST_NOISE = rand::random::<f32>() * 4.0 - 2.0;
        }
    }
    *x = unsafe { LAST_NOISE * volume };
}

static HARMONIC_COUNT: u8 = 5;

fn organ_wave(phase: f32, x: &mut f32, volume: f32) {
    let mut value = 0.0;
    let mut ampl = volume;

    for i in 1..=HARMONIC_COUNT {
        value += ampl * (i as f32) * (2.0 * PI * (i as f32) * phase).sin();
        ampl *= 0.5;
    }

    *x = value;
}

pub fn wave(typ: WaveType, phase: f32, x: &mut f32, volume: f32, phase_inc: f32) {
    match typ {
        WaveType::NoiseWave => noise_wave(phase, x, volume, phase_inc),
        WaveType::OrganWave => organ_wave(phase, x, volume),
        WaveType::SawtoothWave => sawtooth_wave(phase, x, volume),
        WaveType::SineWave => sine_wave(phase, x, volume),
        WaveType::SquareWave => square_wave(phase, x, volume),
        WaveType::TiltedSawtoothWave => tilted_sawtooth_wave(phase, x, volume),
        WaveType::TriangleWave => triangle_wave(phase, x, volume),
    }
}
