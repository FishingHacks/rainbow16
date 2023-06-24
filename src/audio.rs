use std::time::{SystemTime, UNIX_EPOCH};

use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired, AudioStatus};

use crate::{
    frequencies::FREQUENCIES,
    get_s_val,
    memory::{sfx, MemorySection},
    singleton::NewSingleton,
    utils::{from_hex, to_hex},
    waves::wave,
    SDL_CONTEXT,
};

static mut __VOL: u8 = 100;
static mut __IS_MUTED: bool = false;

pub fn is_muted() -> bool {
    unsafe { __IS_MUTED }
}

pub fn set_muted(new: Option<bool>) {
    unsafe {
        __IS_MUTED = new.unwrap_or(!is_muted());
    }
}

pub fn get_volume() -> u8 {
    unsafe {
        if is_muted() {
            0
        } else {
            __VOL
        }
    }
}

pub fn set_volume(mut new: u8) {
    if new > 100 {
        new = 100;
    }
    unsafe { __VOL = new }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum WaveType {
    #[default]
    SquareWave,
    SineWave,
    SawtoothWave,
    TriangleWave,
    TiltedSawtoothWave,
    NoiseWave,
    OrganWave,
}

impl WaveType {
    pub fn from_u8(i: u8) -> Self {
        match i {
            1 => Self::SineWave,
            2 => Self::SawtoothWave,
            3 => Self::TriangleWave,
            4 => Self::TiltedSawtoothWave,
            5 => Self::NoiseWave,
            6 => Self::OrganWave,
            0 | _ => Self::SquareWave,
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct AudioItem {
    pub wave_type: WaveType,
    pub sound: u8,
    pub volume: u8,
}

#[derive(Copy, Clone)]
pub struct Audio {
    pub speed: u8,
    pub items: [AudioItem; 32],
}

impl AudioItem {
    fn write_to_memory(&self, memory: &MemorySection, offset: u32) {
        memory.set_at_addr(offset, self.wave_type as u8);
        memory.set_at_addr(offset + 1, self.sound);
        memory.set_at_addr(offset + 2, self.volume);
    }

    fn read_from_memory(&mut self, memory: &MemorySection, offset: u32) {
        self.wave_type = WaveType::from_u8(memory.get_at_addr_d(offset));
        self.sound = memory.get_at_addr_d(offset + 1);
        self.volume = memory.get_at_addr_d(offset + 2);
    }

    fn from_memory(memory: &MemorySection, offset: u32) -> Self {
        let mut new = Self {
            sound: 0,
            volume: 0,
            wave_type: WaveType::SquareWave,
        };
        new.wave_type = WaveType::from_u8(memory.get_at_addr_d(offset));
        new.sound = memory.get_at_addr_d(offset + 1);
        new.volume = memory.get_at_addr_d(offset + 2);
        new
    }

    const fn new() -> Self {
        AudioItem {
            wave_type: WaveType::SquareWave,
            sound: 0,
            volume: 0,
        }
    }
}

impl Audio {
    pub fn write_to_memory(&self, memory: &MemorySection, mut offset: u32) {
        memory.set_at_addr(offset, self.speed);
        offset += 1;
        for audio_item in self.items.iter() {
            audio_item.write_to_memory(memory, offset);
            offset += 3;
        }
    }

    fn read_from_memory(&mut self, memory: &MemorySection, mut offset: u32) {
        self.speed = memory.get_at_addr_d(offset);
        offset += 1;
        for item in self.items.iter_mut() {
            item.read_from_memory(memory, offset);
            offset += 3;
        }
    }

    pub const fn new() -> Self {
        Self {
            speed: 1,
            items: [AudioItem::new(); 32],
        }
    }

    fn from_memory(memory: &MemorySection, mut offset: u32) -> Self {
        let mut new = Self::new();
        new.speed = memory.get_at_addr_d(offset);
        offset += 1;
        for item in new.items.iter_mut() {
            item.read_from_memory(memory, offset);
            offset += 3;
        }
        new
    }

    pub fn to_string(&self) -> String {
        let mut vec: Vec<u8> = Vec::new();
        vec.push(self.speed);
        for audio_item in self.items.iter() {
            vec.push(audio_item.sound);
            vec.push(audio_item.volume);
            vec.push(audio_item.wave_type as u8);
        }

        let mut str = String::with_capacity(vec.len() * 2);
        for v in vec {
            str.push_str(&to_hex(v));
        }
        str
    }

    pub fn from_string(str: String) -> Self {
        let mut vec: Vec<u8> = Vec::with_capacity(str.len() / 2);

        for i in 0..97 {
            vec.push(from_hex(&str, i * 2));
        }

        let mut new = Self::new();
        new.speed = vec[0];
        let mut off = 1;
        for item in new.items.iter_mut() {
            item.sound = vec[off];
            item.volume = vec[off+1];
            item.wave_type = WaveType::from_u8(vec[off+2]);
            off += 3;
        }
        
        new
    }
}

static DESIRED_SPEC: AudioSpecDesired = AudioSpecDesired {
    channels: Some(1),
    freq: Some(44100),
    samples: None,
};

struct AudioHandler {
    freq: f32,
    phase: f32,
}

impl AudioCallback for AudioHandler {
    type Channel = f32;

    fn callback(&mut self, x: &mut [Self::Channel]) {
        for x in x.iter_mut() {
            if let Some(mut item) = get_current_audio_item() {
                if item.sound >= FREQUENCIES.len() as u8 {
                    item.sound = FREQUENCIES.len() as u8 - 1;
                }
                let inc = FREQUENCIES[item.sound as usize] / self.freq;
                wave(
                    item.wave_type,
                    self.phase,
                    x,
                    get_volume() as f32 * 0.0025 * (item.volume as f32 / 5.0),
                    inc,
                );
                self.phase = (self.phase + inc) % 1.0;
            } else {
                self.phase = 0.0;
                *x = 0.0;
            }
        }
    }
}

static mut DEVICE_SFX: NewSingleton<AudioDevice<AudioHandler>> = NewSingleton::new(|| {
    if let Some(ctx) = get_s_val!(SDL_CONTEXT) {
        Some(
            ctx.audio()
                .expect("Failed to get the audio subsystem")
                .open_playback(None, &DESIRED_SPEC, |spec| AudioHandler {
                    freq: spec.freq as f32,
                    phase: 0.0,
                })
                .expect("Failed to create audio device!"),
        )
    } else {
        None
    }
});

pub fn tick_audio() {
    if let Some(v) = get_s_val!(DEVICE_SFX) {
        if v.status() == AudioStatus::Paused {
            v.resume();
        }
    }
}

pub fn get_current_audio_item() -> Option<AudioItem> {
    let mem = get_s_val!(sfx);
    if mem.get_at_addr_d(102) < 1 {
        return None;
    }
    let current_audio = Audio::from_memory(mem, 0);
    if current_audio.speed < 1 {
        for i in 0..=102 {
            mem.set_at_addr_u32(i, 0);
        }
        return None;
    }
    let start_time = mem.get_at_addr_u32_d(98);
    let length_ms = current_audio.speed as u32 * 320; // 32 sounds, each sound taking 10*speed ms
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards ???")
        .as_millis() as u32;
    let sound_idx = (now - start_time) / // amount of time elapsed since start
    (current_audio.speed as u32
        * 10); // amount of time in ms it takes for a sound to finish
    if start_time + length_ms < now || sound_idx >= current_audio.items.len() as u32 {
        for i in 0..=102 {
            mem.set_at_addr_u32(i, 0);
        }
        return None;
    }
    Some(current_audio.items[sound_idx as usize])
}
