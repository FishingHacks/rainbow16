use sdl2::{keyboard::Keycode, mouse::MouseButton};

use crate::{
    c_singleton,
    gamestate::get_image_vec,
    get_s_val,
    keyboard::mouse_button_down,
    sprites::{IMG_ARR_LEFT, IMG_ARR_RIGHT, IMG_TAB_ONE_SELECTED, IMG_TAB_TWO_SELECTED},
    Singleton,
};

use super::canvas_functions::*;

static mut SELECTED: u8 = 0;

fn get_offset() -> (u32, u32) {
    let x = unsafe { CURSPR % 16 } * 8;
    let y = unsafe { CURSPR / 16 } * 8;
    (x, y)
}

pub fn pad_start(str: String, char: char, mut length: usize) -> String {
    length = length.max(str.len());
    let padding_needed = length - str.len();
    let mut _s = String::with_capacity(length);
    _s.extend((0..padding_needed).map(|_| char));
    _s.push_str(&str);
    return _s;
}

pub fn render() {
    let s = unsafe { SELECTED };
    let s_y = (s / 4) as i32;
    let s_x = (s % 4) as i32;
    clear(Some(0));
    rect(7, 41, 44, 44, 12);
    rect(8, 42, 42, 42, 0);
    for x in 0..4 {
        for y in 0..4 {
            rectfill(x * 10 + 9, y * 10 + 43, 10, 10, (y * 4 + x) as u8);
        }
    }

    // selected
    rect(8 + s_x * 10, 42 + s_y * 10, 12, 12, 12);
    rect(9 + s_x * 10, 43 + s_y * 10, 10, 10, 0);
    //canvas
    rect(70, 21, 84, 84, 12);
    rect(71, 22, 82, 82, 0);
    let (ox, oy) = get_offset();
    for x in 0..8 {
        for y in 0..8 {
            rectfill(
                72 + x * 10,
                23 + y * 10,
                10,
                10,
                get_s_val!(IMG)[((y + oy as i32) * 128 + x + ox as i32) as usize] as u8,
            )
        }
    }

    // image selector
    get_s_val!(IMG_ARR_LEFT).put_on_canvas(set_pixel, 15, 94);
    get_s_val!(IMG_ARR_RIGHT).put_on_canvas(set_pixel, 37, 94);
    rectfill(22, 93, 13, 7, 15);
    print(
        &format!("{}", unsafe { pad_start(CURSPR.to_string(), '0', 3) }),
        Some(23),
        Some(94),
        None,
    );

    // tilesheet
    print(&"tilesheet".to_string(), Some(1), Some(102), None);
    rectfill(0, 108, 129, 1, 15);
    rectfill(0, 108, 1, 65, 15);
    rectfill(129, 108, 1, 65, 15);

    let is_first_tab = unsafe { CURSPR } < MAXSPR / 2;

    if is_first_tab {
        get_s_val!(IMG_TAB_ONE_SELECTED).put_on_canvas(set_pixel, 130, 110);
    } else {
        get_s_val!(IMG_TAB_TWO_SELECTED).put_on_canvas(set_pixel, 130, 110);
    }

    let image_vec = get_image_vec();

    for y in 0..64 {
        for x in 0..128 {
            let c = if is_first_tab {
                image_vec[y * 128 + x]
            } else {
                image_vec[(y + 64) * 128 + x]
            };
            set_pixel(x as i32 + 1, y as i32 + 109, c);
        }
    }

    let (cx, mut cy) = get_offset();
    cy %= 64;
    cy += 108;
    rect(cx as i32, cy as i32, 10, 10, 12);

    // bottom bar
    rectfill(0, 173, 200, 7, 2);
    let (ox, oy) = get_offset();
    print(
        &format!(
            "color:{s} x:{} y:{} selected:{}",
            unsafe { CURX as u32 + ox },
            unsafe { CURY as u32 + oy },
            unsafe { CURSPR }
        ),
        Some(1),
        Some(174),
        None,
    );
}

fn change_selected(value: i32) {
    let mut s = unsafe { CURSPR };
    if value < 0 {
        let val = value.abs() as u32;
        if s >= val {
            s -= val;
        } else {
            let val = val - s;
            s = 255;
            s -= val;
        }
    } else {
        let val = value.abs() as u32;
        if 255 - s >= val {
            s += val;
        } else {
            let val = val - (255 - s + 1);
            s = 0;
            s += val;
        }
    }
    unsafe {
        CURSPR = s;
    }
}

pub fn keydown(key: Keycode) {
    match key {
        Keycode::Left => change_selected(-1),
        Keycode::Right => change_selected(1),
        Keycode::Up => change_selected(-16),
        Keycode::Down => change_selected(16),
        _ => {}
    }
}

static mut CURX: u8 = 0;
static mut CURY: u8 = 0;
static mut CURSPR: u32 = 0;
static MAXSPR: u32 = 256;

c_singleton!(IMG, &'static mut Vec<u8>, get_image_vec);

pub fn mousedown(button: MouseButton, mut x: u32, mut y: u32) {
    if x >= 9 && x <= 49 && y >= 43 && y <= 83 && button == MouseButton::Left {
        x -= 9;
        y -= 43;
        x /= 10;
        y /= 10;
        unsafe {
            SELECTED = (y * 4 + x) as u8;
        }
    }

    if x >= 71 && x < 153 && y >= 22 && y < 104 && button == MouseButton::Left {
        x -= 71;
        y -= 22;
        x /= 10;
        y /= 10;
        if x < 8 && y < 8 {
            unsafe {
                CURX = x as u8;
                CURY = y as u8;
                let (ox, oy) = get_offset();
                get_s_val!(IMG)[((CURY as u32 + oy) * 128 + CURX as u32 + ox) as usize] =
                    SELECTED % 16;
            }
        }
    }
    if x >= 71 && x < 153 && y >= 22 && y < 104 && button == MouseButton::Right {
        x -= 71;
        y -= 22;
        x /= 10;
        y /= 10;
        if x < 8 && y < 8 {
            unsafe {
                CURX = x as u8;
                CURY = y as u8;
                let (ox, oy) = get_offset();
                SELECTED = get_s_val!(IMG)[((CURY as u32 + oy) * 128 + CURX as u32 + ox) as usize];
            }
        }
    }
    if button == MouseButton::Left
        && y >= 94
        && y <= 99
        && ((x >= 15 && x <= 19) || (x >= 37 && x <= 42))
    {
        let mut _s = unsafe { CURSPR };
        if x < 37 {
            // left, -1
            if _s > 0 {
                _s -= 1;
            } else {
                _s = MAXSPR - 1
            }
        } else {
            // right, +1
            _s += 1;
            if _s >= MAXSPR {
                _s = 0;
            }
        }
        unsafe { CURSPR = _s };
    }
    if button == MouseButton::Left && y >= 110 && y <= 125 && x >= 130 && x <= 138 {
        if y > 118 {
            unsafe {
                CURSPR = MAXSPR / 2;
            };
        } else {
            unsafe {
                CURSPR = 0;
            }
        }
    }
    if button == MouseButton::Left && y >= 109 && y < 173 && x >= 1 && x < 129 {
        unsafe {
            CURSPR = (y - 109 + (if CURSPR < MAXSPR / 2 { 0 } else { 64 })) / 8 * 16 + (x - 1) / 8
        }
    }
}

pub fn mousemove(x: u32, y: u32) {
    if x >= 71 && x < 153 && y >= 22 && y < 104 && mouse_button_down(MouseButton::Left) {
        mousedown(MouseButton::Left, x, y);
    }
    if x >= 71 && x < 153 && y >= 22 && y < 104 && mouse_button_down(MouseButton::Right) {
        mousedown(MouseButton::Right, x, y);
    }
    if mouse_button_down(MouseButton::Left) && y >= 109 && y < 173 && x >= 1 && x < 129 {
        mousedown(MouseButton::Left, x, y);
    }
}