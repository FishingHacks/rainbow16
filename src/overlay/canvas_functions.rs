#![allow(dead_code, unused_variables, non_upper_case_globals)]

use crate::{charmap::get_char, get_s_val, swap, HEIGHT, WIDTH};

use super::globals::DISPLAYMEM;

#[inline(always)]
pub fn set_pixel(x: i32, y: i32, color: u8) {
    if !in_bounds(x, y) {
        return;
    }

    get_s_val!(DISPLAYMEM)[(y * WIDTH as i32 + x).abs() as usize] = color % 16;
}

fn to_safe_rect(
    mut x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
) -> Option<(i32, i32, i32, i32)> {
    if x2 == x1 || y2 == y1 {
        return None;
    }

    if x1 < 0 {
        x1 = 0;
    }
    if y1 < 0 {
        y1 = 0;
    }

    if x2 < x1 {
        swap!(x2, x1);
    }
    if y2 < y1 {
        swap!(y2, y1);
    }

    if x1 < 0 {
        x1 = 0;
    }
    if y1 < 0 {
        y1 = 0;
    }

    if x2 > WIDTH as i32 {
        x2 = WIDTH as i32;
    }
    if y2 > HEIGHT as i32 {
        y2 = HEIGHT as i32;
    }

    if x2 == x1 || y2 == y1 {
        return None;
    }

    Some((x1, y1, x2, y2))
}

pub fn clear(color: Option<u8>) {
    let c = color.unwrap_or(0);
    for i in 0..WIDTH * HEIGHT {
        get_s_val!(DISPLAYMEM)[(i) as usize] = c % 16;
    }
}

pub fn rectfill(x: i32, y: i32, w: i32, h: i32, color: u8) {
    if let Some((x1, y1, x2, y2)) = to_safe_rect(x, y, x + w, y + h) {
        for y in y1..y2 {
            for x in x1..x2 {
                set_pixel(x, y, color);
            }
        }
    }
}

pub fn rect(x: i32, y: i32, w: i32, h: i32, color: u8) {
    rectfill(x, y, w, 1, color);
    rectfill(x, y, 1, h, color);
    rectfill(x, y + h - 1, w, 1, color);
    rectfill(x + w - 1, y, 1, h, color);
}

pub fn ellipse(cx: i32, cy: i32, rx: i32, ry: i32, color: u8) {
    let rx1 = rx + rx.signum();
    let ry1 = ry + ry.signum();
    if let Some((x1, y1, x2, y2)) = to_safe_rect(cx - rx1, cy - ry1, 2 * rx1, 2 * ry1) {
        for y in y1..y2 {
            for x in x1..(x2 + 1) {
                let nx: f32 = (x as f32 + 0.5 - x1 as f32) / (2.0 * rx1 as f32);
                let ny: f32 = (y as f32 + 0.5 - y1 as f32) / (2.0 * ry1 as f32);
                let dx = nx - 0.5;
                let dy = ny - 0.5;
                if dx * dx + dy * dy < 0.25 {
                    set_pixel(x, y, color);
                }
            }
        }
    }
}

pub fn circle(cx: i32, cy: i32, r: i32, color: u8) {
    let r1 = r + r.signum();
    if let Some((x1, y1, x2, y2)) = to_safe_rect(cx - r1, cy - r1, r1 + cx, r1 + cy) {
        let rsq = r * r;

        for y in y1..(y2 + 1) {
            for x in x1..(x2 + 1) {
                let dx = x - cx;
                let dy = y - cy;
                if dx * dx + dy * dy < rsq {
                    set_pixel(x, y, color);
                }
            }
        }
    }
}

pub fn in_bounds(x: i32, y: i32) -> bool {
    !(x < 0 || x >= WIDTH as i32 || y < 0 || y >= HEIGHT as i32)
}

pub fn line(mut x1: i32, mut y1: i32, x2: i32, y2: i32, color: u8) {
    let sx: i32 = if x1 < x2 { 1 } else { -1 };
    let sy: i32 = if y1 < y2 { 1 } else { -1 };
    let dx: i32 = if x1 < x2 { x2 - x1 } else { x1 - x2 };
    let dy: i32 = if y1 < y2 { y2 - y1 } else { y1 - y2 };

    let mut err: i32 = dx - dy;
    #[allow(unused)]
    let mut e2: i32 = err * 2;

    while x1 != x2 && y1 != y2 {
        e2 = err * 2;
        if e2 > -dy {
            err = err - dy;
            x1 = x1 + sx;
        }
        if e2 < dx {
            err = err + dx;
            y1 = y1 + sy;
        }
        if !in_bounds(x1, y1) {
            continue;
        }
        set_pixel(x1, y1, color)
    }
}

pub fn switch_palette(palette: u8) {
    get_s_val!(DISPLAYMEM)[16] = palette % 4;
}

static mut cursorx: i32 = 0;
static mut cursory: i32 = 0;

pub fn cursor(x: Option<i32>, y: Option<i32>) {
    if x.is_none() && y.is_none() {
        unsafe {
            cursorx = 0;
            cursory = 0;
        }
    }
    if let Some(_x) = x {
        unsafe {
            cursorx = _x;
        }
    }
    if let Some(_y) = y {
        unsafe {
            cursory = _y;
        }
    }
}

pub fn put_char_on_canvas(char: &u32, x: i32, y: i32, color: u8) {
    for oy in 0..5 {
        for ox in 0..3 {
            let off = (4 - oy) * 3 + ox;
            if char >> off & 0x1 == 1 {
                set_pixel(x + (2 - ox), y + oy, color);
            }
        }
    }
}

pub fn print(text: &String, x: Option<i32>, y: Option<i32>, color: Option<u8>) {
    let bytes = text.as_bytes();

    if x.is_some() || y.is_some() {
        cursor(x, y);
    }

    let col = color.unwrap_or(12);
    let mut i: usize = 0;
    while i < text.len() {
        match bytes[i] {
            // \n
            10 => cursor(Some(x.unwrap_or(0)), Some(unsafe { cursory } + 6)),
            // ' '
            32 => unsafe { cursorx += 4 },
            _ => unsafe {
                let char = get_char(bytes[i]);
                put_char_on_canvas(&char, cursorx, cursory, col);

                cursorx += 4;
            },
        };
        i += 1;
    }
    cursor(Some(x.unwrap_or(0)), Some(unsafe { cursory } + 6));
}
