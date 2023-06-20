use sdl2::keyboard::Keycode;

use crate::utils::{is_alt_pressed, is_altgr_pressed, is_ctrl_pressed, is_shift_pressed};

static NORMAL: &[u8] = " abcdefghijklmnopqrstuvwxyz-.,<^1234567890#+\n".as_bytes();
static SHIFT: &[u8] = " ABCDEFGHIJKLMNOPQRSTUVWXYZ_:;>^!\"4$%&/()='*\n".as_bytes();
static ALT: &[u8] = " abcdefghijklmnopqrstuvwxyz-.,|^123456{[]}#~\n".as_bytes();

pub fn keycode_to_character(key: Keycode) -> Option<char> {
    assert_eq!(NORMAL.len(), SHIFT.len());
    assert_eq!(NORMAL.len(), ALT.len());

    let index: i32 = match key {
        Keycode::Space | Keycode::KpSpace => 0,
        Keycode::A => 1,
        Keycode::B => 2,
        Keycode::C => 3,
        Keycode::D => 4,
        Keycode::E => 5,
        Keycode::F => 6,
        Keycode::G => 7,
        Keycode::H => 8,
        Keycode::I => 9,
        Keycode::J => 10,
        Keycode::K => 11,
        Keycode::L => 12,
        Keycode::M => 13,
        Keycode::N => 14,
        Keycode::O => 15,
        Keycode::P => 16,
        Keycode::Q => 17,
        Keycode::R => 18,
        Keycode::S => 19,
        Keycode::T => 20,
        Keycode::U => 21,
        Keycode::V => 22,
        Keycode::W => 23,
        Keycode::X => 24,
        Keycode::Y => 25,
        Keycode::Z => 26,
        Keycode::Minus => 27,
        Keycode::Period => 28,
        Keycode::Comma | Keycode::KpComma => 29,
        Keycode::Less => 30,
        Keycode::Caret => 31,
        Keycode::Num1 => 32,
        Keycode::Num2 => 33,
        Keycode::Num3 => 34,
        Keycode::Num4 => 35,
        Keycode::Num5 => 36,
        Keycode::Num6 => 37,
        Keycode::Num7 => 38,
        Keycode::Num8 => 39,
        Keycode::Num9 => 40,
        Keycode::Num0 => 41,
        Keycode::Hash => 42,
        Keycode::Plus => 43,
        Keycode::KpEnter | Keycode::Return | Keycode::Return2 => 44,
        _ => -1,
    };

    if index < 0 || index >= NORMAL.len() as i32 {
        return None;
    }

    if ((is_alt_pressed() && is_ctrl_pressed()) || is_altgr_pressed())
        && ALT[index as usize] != NORMAL[index as usize]
    {
        return Some(ALT[index as usize] as char);
    } else if is_shift_pressed() {
        return Some(SHIFT[index as usize] as char);
    } else {
        return Some(NORMAL[index as usize] as char);
    }
}
