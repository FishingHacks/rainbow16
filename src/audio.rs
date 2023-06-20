static waves: [f32; 48] = [
    65.41, 69.3, 73.42, 77.78, 82.41, 87.31, 92.5, 98.0, 103.83, 110.0, 116.54, 123.47, 130.81,
    138.59, 146.83, 155.56, 164.81, 174.61, 185.0, 196.0, 207.65, 220.0, 233.08, 246.94, 261.63,
    277.18, 293.66, 311.13, 329.63, 349.2, 369.9, 392.0, 415.3, 440.0, 466.1, 493.8, 523.2, 554.3,
    587.3, 622.2, 659.2, 698.4, 739.9, 783.9, 830.6, 880.0, 932.3, 987.7,
];

pub enum WaveType {
    SquareWave,
    SineWave,
    SawtoothWave,
    TriangleWave,
    NoiseWave,
    TiltedTriangleWave,
    OrganWave,
}

pub struct AudioItem {
    octave: u8,
    tone: u8,
    volume: u8,
    instrument: WaveType,
}

impl AudioItem {
    pub fn from(octave: u8, tone: u8, volume: u8, instrument: WaveType) -> Option<Self> {
        if octave > 3 || tone > 11 || volume > 7 {
            return None;
        }
        Some(Self {
            instrument,
            octave,
            tone,
            volume,
        })
    }

    pub fn play() {
        
    }
}

impl Default for AudioItem {
    fn default() -> Self {
        Self {
            instrument: WaveType::SquareWave,
            octave: 0,
            tone: 0,
            volume: 0,
        }
    }
}

pub struct Audio {
    audios: [AudioItem; 32],
    speed: u8,
}
