use sdl2::{keyboard::Keycode, mouse::MouseButton};

use crate::{
    get_s_val,
    memory::{charpress, keymemory},
};

pub fn is_key_down(key: Keycode) -> bool {
    if key as u32 == 0 {
        return false;
    }
    let mem = get_s_val!(keymemory);
    for i in 0u32..10 {
        if mem.get_at_addr_u32_d(i * 4 + 1) == key as u32 {
            return true;
        }
    }

    return false;
}

pub fn handle_keydown(key: Keycode) {
    let mem = get_s_val!(keymemory);
    for i in 0u32..10 {
        if mem.get_at_addr_u32_d(i * 4 + 1) == 0 {
            mem.set_at_addr(i * 4 + 1, (key as u32 & 0xff) as u8);
            mem.set_at_addr(i * 4 + 2, (((key as u32) >> 8) & 0xff) as u8);
            mem.set_at_addr(i * 4 + 3, (((key as u32) >> 16) & 0xff) as u8);
            mem.set_at_addr(i * 4 + 4, (((key as u32) >> 24) & 0xff) as u8);
            break;
        }
    }
}

pub fn handle_acc_keys_down(key: Keycode) {
    let mem = get_s_val!(keymemory);
    if key == Keycode::LCtrl || key == Keycode::RCtrl {
        mem.set_at_addr(0, mem.get_at_addr_d(0) | 1); // 1. bit set to 1
    }
    if key == Keycode::LAlt {
        mem.set_at_addr(0, mem.get_at_addr_d(0) | 2); // 2. bit set to 1
    }
    if key == Keycode::LShift || key == Keycode::RShift {
        mem.set_at_addr(0, mem.get_at_addr_d(0) | 4); // 3. bit set to 1
    }
    if key == Keycode::CapsLock {
        mem.set_at_addr(0, mem.get_at_addr_d(0) | 8); // 4. bit set to 1
    }
    if key == Keycode::RAlt {
        mem.set_at_addr(0, mem.get_at_addr_d(0) | 16); // 5. bit set to 1
    }
}

pub fn handle_acc_keys_up(key: Keycode) {
    let mem = get_s_val!(keymemory);
    if key == Keycode::LCtrl || key == Keycode::RCtrl {
        mem.set_at_addr(0, mem.get_at_addr_d(0) & 254); // bitmask to exclude the 1. bit
    }
    if key == Keycode::LAlt {
        mem.set_at_addr(0, mem.get_at_addr_d(0) & 253); // bitmask to exclude the 2. bit
    }
    if key == Keycode::LShift || key == Keycode::RShift {
        mem.set_at_addr(0, mem.get_at_addr_d(0) & 251); // bitmask to exclude the 3. bit
    }
    if key == Keycode::CapsLock {
        mem.set_at_addr(0, mem.get_at_addr_d(0) & 247); // bitmask to exclude the 4. bit
    }
    if key == Keycode::RAlt {
        mem.set_at_addr(0, mem.get_at_addr_d(0) & 239); // bitmask to exclude the 5. bit
    }
}

pub fn handle_keyup(key: Keycode) {
    if key as u32 == 0 {
        return;
    }
    let mem = get_s_val!(keymemory);
    for i in 0u32..10 {
        if mem.get_at_addr_u32_d(i * 4 + 1) == key as u32 {
            mem.set_at_addr(i * 4 + 1, 0);
            mem.set_at_addr(i * 4 + 2, 0);
            mem.set_at_addr(i * 4 + 3, 0);
            mem.set_at_addr(i * 4 + 4, 0);
        }
    }
}

pub fn handle_textinput(char: char) {
    get_s_val!(charpress).set_at_addr_u32(0, char as u32);
}

unsafe fn increment_down_counter(key: u32, reset: bool) {
    let mem = get_s_val!(keymemory);
    if reset {
        mem.set_at_addr(key, 0);
    } else {
        let mut byte = mem.get_at_addr_d(key);
        byte += 1;
        if byte >= 7 {
            byte = 5;
        }
        mem.set_at_addr(key, byte);
    }
}

pub fn keyboard_update() {
    unsafe {
        increment_down_counter(0x26, !is_key_down(Keycode::Up));
        increment_down_counter(0x27, !is_key_down(Keycode::Down));
        increment_down_counter(0x28, !is_key_down(Keycode::Left));
        increment_down_counter(0x29, !is_key_down(Keycode::Right));
        increment_down_counter(0x2a, !is_key_down(Keycode::U));
        increment_down_counter(0x2b, !is_key_down(Keycode::I));
        increment_down_counter(0x2c, !is_key_down(Keycode::O));
        increment_down_counter(0x2d, !is_key_down(Keycode::P));
    }
}

pub fn button_is_pressed(button: Button) -> bool {
    let ticks = get_s_val!(keymemory)
        .get_at_addr(0x26 + button as u32)
        .unwrap_or(0);
    ticks == 1 || ticks > 5 && ticks % 3 == 0
}

pub fn button_is_down(button: Button) -> bool {
    let ticks = get_s_val!(keymemory)
        .get_at_addr(0x26 + button as u32)
        .unwrap_or(0);
    ticks > 0
}

pub fn u8_to_button(u8: u8) -> Button {
    match u8 % 8 {
        0 => Button::Up,
        1 => Button::Down,
        2 => Button::Left,
        3 => Button::Right,
        4 => Button::A,
        5 => Button::B,
        6 => Button::X,
        7 => Button::Y,
        _ => panic!("Unreachable (u8_to_button)"),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Button {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
    A = 4,
    B = 5,
    X = 6,
    Y = 7,
}

pub fn handle_mousedown(button: MouseButton) {
    let mem = get_s_val!(keymemory);
    let mut val = mem.get_at_addr_d(0x2e);
    match button {
        MouseButton::Left => val |= 1,
        MouseButton::Right => val |= 2,
        MouseButton::Middle => val |= 4,
        _ => {}
    }
    mem.set_at_addr(0x2e, val);
}

pub fn handle_mouseup(button: MouseButton) {
    let mem = get_s_val!(keymemory);
    let mut val = mem.get_at_addr_d(0x2e);
    match button {
        MouseButton::Left => val &= 254,
        MouseButton::Right => val &= 253,
        MouseButton::Middle => val &= 251,
        _ => {}
    }
    mem.set_at_addr(0x2e, val);
}

pub fn reset_scroll() {
    get_s_val!(keymemory).set_at_addr_u32(0x2f, i32::MAX as u32);
}

pub fn handle_scroll(x: i32) {
    let newx = x as i64 + get_s_val!(keymemory).get_at_addr_u32_d(0x2f) as i64;
    get_s_val!(keymemory).set_at_addr_u32(0x2f, newx as u32);
}

pub fn handle_mousemove(x: u32, y: u32) {
    let mem = get_s_val!(keymemory);
    mem.set_at_addr_u32(0x33, x);
    mem.set_at_addr_u32(0x37, y);
}

pub fn mouse_button_down(button: MouseButton) -> bool {
    let val = get_s_val!(keymemory).get_at_addr_d(0x2e);
    match button {
        MouseButton::Left => val & 1 > 0,
        MouseButton::Right => val & 2 > 0,
        MouseButton::Middle => val & 4 > 0,
        _ => false,
    }
}
