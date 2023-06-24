#![allow(dead_code)]

use std::fmt::Display;

use crate::{get_s_val, singleton::Singleton, HEIGHT, WIDTH};

static mut __MEM: Singleton<Vec<u8>> = Singleton::new(|| <Vec<u8>>::new());

pub fn getmem() -> &'static mut Vec<u8> {
    unsafe { __MEM.get() }
}

static mut CURRENT_PTR: u32 = 0;

pub struct MemorySection {
    start: u32,
    length: u32,
    name: String,
}

impl MemorySection {
    pub fn new<T: Into<String>>(length: u32, name: T) -> Self {
        let section = MemorySection {
            length,
            start: unsafe { CURRENT_PTR },
            name: name.into(),
        };
        unsafe {
            CURRENT_PTR += length;
        }
        let mem = getmem();
        for _ in 0..length {
            mem.push(0);
        }

        section
    }

    pub fn get_at_addr(&self, address: u32) -> Option<u8> {
        let mem = getmem();
        if address >= self.length || self.start + address >= mem.len() as u32 {
            None
        } else {
            Some(mem[(self.start + address) as usize])
        }
    }

    pub fn get_at_addr_d(&self, address: u32) -> u8 {
        self.get_at_addr(address).unwrap_or(0)
    }

    pub fn get_at_addr_u32_d(&self, address: u32) -> u32 {
        self.get_at_addr_u32(address).unwrap_or(0)
    }

    pub fn get_at_addr_u32(&self, address: u32) -> Option<u32> {
        let mut num: u32 = 0;
        for i in 0..4 {
            if let Some(v) = self.get_at_addr(address + i) {
                num |= (v as u32) << i * 8;
            } else {
                return None;
            }
        }

        Some(num)
    }

    pub fn set_at_addr_u32(&self, address: u32, byte: u32) {
        self.set_at_addr(address, (byte & 0xff) as u8);
        self.set_at_addr(address + 1, ((byte >> 8) & 0xff) as u8);
        self.set_at_addr(address + 2, ((byte >> 16) & 0xff) as u8);
        self.set_at_addr(address + 3, ((byte >> 24) & 0xff) as u8);
    }

    pub fn set_at_addr(&self, address: u32, byte: u8) {
        let mem = getmem();
        if address < self.length && self.start + address < mem.len() as u32 {
            mem[(self.start + address) as usize] = byte;
        }
    }
}

impl Display for MemorySection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "MemorySection {} ({} bytes, starts at {:#x})",
            self.name, self.length, self.start
        ))
    }
}

// 0x0-0xf: Color Translations
// 0x10: Palette
// 0x11-0x12: Color Translations (transparency)
// 0x13-end: Display
#[allow(non_upper_case_globals)]
pub static mut displaymemory: Singleton<MemorySection> = Singleton::new(|| {
    let section = MemorySection::new(WIDTH * HEIGHT + 19, "Display Memory");
    for i in 0..16 {
        section.set_at_addr(i, i as u8);
    }
    section.set_at_addr(16, 0); // palette
    section.set_at_addr(17, 0); // color translations transparency
    section.set_at_addr(18, 0);
    section
});
/*
Layout:
0x0: Acceleration Keys
-> 1. bit: CtrlKey
-> 2. bit: AltKey
-> 3. bit: ShiftKey
-> 4. bit: CapsLock
-> 5. bit: AltGr (Mode) Key
-> _. bit: Unassigned
0x1-0x25: KeyCode indices
0x26-0x2d: #Ticks since button got pressed + 1
-> 1. byte: Up
-> 2. byte: Down
-> 3. byte: Left
-> 4. byte: Right
-> 5. byte: A
-> 6. byte: B
-> 7. byte: X
-> 8. byte: Y
0x2e: mouse buttons
-> 1. bit: Left
-> 2. bit: Right
-> 3. bit: Middle
0x2f-0x32: mouse wheel dy (subtract i32::max)
0x33-0x36: mouse pos x
0x37-0x3a: mouse pos y
0x3b: Set to 1 to activate the mouse cursor
 */
#[allow(non_upper_case_globals)]
pub static mut keymemory: Singleton<MemorySection> =
    Singleton::new(|| MemorySection::new(60, "Key Memory"));

// a 4-byte char
#[allow(non_upper_case_globals)]
pub static mut charpress: Singleton<MemorySection> =
    Singleton::new(|| MemorySection::new(4, "Charpress Memory"));

// see: crate::audio::AudioItem
// 98-101 4 bytes: starttime in ms: u32
// 102: bool on wether or not a sound is playing
#[allow(non_upper_case_globals)]
pub static mut sfx: Singleton<MemorySection> =
    Singleton::new(|| MemorySection::new(103, "SFX Memory"));

pub fn peek(address: usize) -> u8 {
    let mem = getmem();
    if address >= mem.len() {
        0
    } else {
        mem[address]
    }
}

pub fn init_memory_sections() {
    let sections = vec![
        get_s_val!(displaymemory),
        get_s_val!(keymemory),
        get_s_val!(charpress),
        get_s_val!(sfx),
    ];

    for s in sections {
        println!("{s}");
    }
}

pub fn poke(address: usize, value: u8) {
    let mem = getmem();
    if address < mem.len() {
        mem[address] = value;
    } else {
        println!("too big!");
    }
}
