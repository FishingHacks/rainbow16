#![allow(dead_code)]
use crate::{get_s_val, memory::keymemory, SDL_CONTEXT};

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

pub fn copy_to_clipboard(str: &str) {
    get_s_val!(SDL_CONTEXT)
        .as_ref()
        .expect("Failed to obtain the sdl context")
        .video()
        .expect("Failed to obtain the video context")
        .clipboard()
        .set_clipboard_text(str)
        .err();
}

pub fn read_clipboard() -> String {
    let clip = get_s_val!(SDL_CONTEXT)
        .as_ref()
        .expect("Failed to obtain the sdl context")
        .video()
        .expect("Failed to obtain the video context")
        .clipboard();

    if !clip.has_clipboard_text() {
        String::new()
    } else {
        clip.clipboard_text().expect("Failed to read the clipboard")
    }
}
