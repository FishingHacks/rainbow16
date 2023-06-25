#![allow(dead_code)]
use crate::{get_s_val, memory::keymemory};

pub fn cycle(mut value: u8, min: u8, max: u8, up: bool) -> u8 {
    if up {
        if value < max - 1 {
            value += 1;
        } else {
            value = min;
        }
    } else {
        if value > min {
            value -= 1
        } else {
            value = max - 1;
        }
    }

    value
}

pub fn is_ctrl_pressed() -> bool {
    get_s_val!(keymemory).get_at_addr_d(0) & 1 > 0
}

pub fn is_alt_pressed() -> bool {
    (get_s_val!(keymemory).get_at_addr_d(0) >> 1) & 1 > 0
}

pub fn is_shift_pressed() -> bool {
    (get_s_val!(keymemory).get_at_addr_d(0) >> 2) & 1 > 0
}

pub fn is_caps_pressed() -> bool {
    (get_s_val!(keymemory).get_at_addr_d(0) >> 3) & 1 > 0
}

pub fn is_altgr_pressed() -> bool {
    (get_s_val!(keymemory).get_at_addr_d(0) >> 4) & 1 > 0
}

#[inline(always)]
pub fn is_mode_pressed() -> bool {
    is_altgr_pressed()
}

pub fn to_hex(num: u8) -> String {
    let mut str = String::with_capacity(2);
    str.push(__to_hex(num & 0xf));
    str.push(__to_hex(num >> 4 & 0xf));

    str
}

pub fn __to_hex(num: u8) -> char {
    (48 + num) as char
}

pub fn from_hex(str: &String, offset: usize) -> u8 {
    let c1 = str.as_bytes()[offset] as char;
    let c2 = str.as_bytes()[offset + 1] as char;

    __from_hex(c1) | (__from_hex(c2) << 4)
}

pub fn __from_hex(char: char) -> u8 {
    char as u8 - 48
}
