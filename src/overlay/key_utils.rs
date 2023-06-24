use sdl2::keyboard::Keycode;

use crate::{get_s_val, memory::charpress};

pub fn keycode_to_character(keycode: Option<Keycode>) -> Option<char> {
    let u32: u32 = get_s_val!(charpress).get_at_addr_u32_d(0);
    if u32 > 0 {
        if let Some(c) = char::from_u32(u32) {
            return Some(c);
        }     
    }
    if let Some(keycode) = keycode {
        match keycode {
            Keycode::KpEnter | Keycode::Return | Keycode::Return2 => Some('\n'),
            _ => None,
        }
    } else {
        None
    }
}
