#![allow(dead_code)]

use crate::charmap::{get_char, put_char_on_canvas};
use crate::gamestate::get_image_vec;
use crate::memory::displaymemory;
use crate::singleton::Singleton;
use crate::{c_singleton, get_s_val, get_s_val_c, set_s_val, swap, HEIGHT, WIDTH};

c_singleton!(MIN_X, i32, || 0);
c_singleton!(MIN_Y, i32, || 0);
c_singleton!(MAX_X, i32, || WIDTH as i32);
c_singleton!(MAX_Y, i32, || HEIGHT as i32);

c_singleton!(OX, i32, || 0);
c_singleton!(OY, i32, || 0);

#[derive(Clone, Copy)]
pub struct Color(u8, u8, u8);

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color(r, g, b)
    }

    pub const fn from_hex(c: u32) -> Self {
        Color(
            (c >> 16 & 0xff) as u8,
            (c >> 8 & 0xff) as u8,
            (c & 0xff) as u8,
        )
    }

    pub fn sdl_write_to_vec(&self, memory: &mut Vec<u8>) {
        memory.push(self.2); //r
        memory.push(self.1); //g
        memory.push(self.0); //b
        memory.push(255); //a
    }

    pub fn get_values(&self) -> (u8, u8, u8) {
        (self.0, self.1, self.2)
    }
}

pub static PALETTE1: [Color; 16] = [
    Color::from_hex(0x1a1c2c),
    Color::from_hex(0x5d275d),
    Color::from_hex(0xb13e53),
    Color::from_hex(0xef7d57),
    Color::from_hex(0xffcd75),
    Color::from_hex(0xa7f070),
    Color::from_hex(0x38b764),
    Color::from_hex(0x257179),
    Color::from_hex(0x29366f),
    Color::from_hex(0x3b5dc9),
    Color::from_hex(0x41a6f6),
    Color::from_hex(0x73eff7),
    Color::from_hex(0xf4f4f4),
    Color::from_hex(0x94b0c2),
    Color::from_hex(0x566c86),
    Color::from_hex(0x333c57),
];
static PALETTE2: [Color; 16] = [
    Color::from_hex(0x28282e),
    Color::from_hex(0x6c5671),
    Color::from_hex(0xd9c8bf),
    Color::from_hex(0xf98284),
    Color::from_hex(0xb0a9e4),
    Color::from_hex(0xaccce4),
    Color::from_hex(0xb3e3da),
    Color::from_hex(0xfeaae4),
    Color::from_hex(0x87a889),
    Color::from_hex(0xb0eb93),
    Color::from_hex(0xe9f59d),
    Color::from_hex(0xffe6c6),
    Color::from_hex(0xdea38b),
    Color::from_hex(0xffc384),
    Color::from_hex(0xfff7a0),
    Color::from_hex(0xfff7e4),
];
static PALETTE3: [Color; 16] = [
    Color::from_hex(0x00033c),
    Color::from_hex(0x005260),
    Color::from_hex(0x009d4a),
    Color::from_hex(0x0aff52),
    Color::from_hex(0x003884),
    Color::from_hex(0x008ac5),
    Color::from_hex(0x00f7ff),
    Color::from_hex(0xff5cff),
    Color::from_hex(0xac29ce),
    Color::from_hex(0x600088),
    Color::from_hex(0xb10585),
    Color::from_hex(0xff004e),
    Color::from_hex(0x2a2e79),
    Color::from_hex(0x4e6ea8),
    Color::from_hex(0xadd4fa),
    Color::from_hex(0xffffff),
];
static PALETTE4: [Color; 16] = [
    Color::from_hex(0x000000),
    Color::from_hex(0x430067),
    Color::from_hex(0x94216a),
    Color::from_hex(0xff004d),
    Color::from_hex(0xff8426),
    Color::from_hex(0xffdd34),
    Color::from_hex(0x50e112),
    Color::from_hex(0x3fa66f),
    Color::from_hex(0x365987),
    Color::from_hex(0x0033ff),
    Color::from_hex(0x29adff),
    Color::from_hex(0x00ffcc),
    Color::from_hex(0xfff1e8),
    Color::from_hex(0xc2c3c7),
    Color::from_hex(0xab5236),
    Color::from_hex(0x5f574f),
];

static SCREEN_MEM_LEN: u32 = WIDTH * HEIGHT;

#[inline(always)]
pub fn color_index_to_color(mut index: u8) -> Color {
    index %= 16;
    index = get_s_val!(displaymemory).get_at_addr_d(index as u32) % 16;
    let paletteindex = get_s_val!(displaymemory).get_at_addr_d(16) % 4;
    let palette = {
        if paletteindex == 0 {
            PALETTE1
        } else if paletteindex == 1 {
            PALETTE2
        } else if paletteindex == 2 {
            PALETTE3
        } else {
            PALETTE4
        }
    };
    palette[index as usize]
}

pub fn sdl_apply_canvas(memory: &mut Vec<u8>) {
    memory.clear();
    let screenmem = get_s_val!(displaymemory);
    for i in 0..SCREEN_MEM_LEN {
        color_index_to_color(screenmem.get_at_addr_d(i + 19)).sdl_write_to_vec(memory);
    }
}

#[inline(always)]
pub fn set_pixel(x: i32, y: i32, color: u8) {
    if !in_bounds(x, y) {
        return;
    }

    if let Some(c) = get_color(color) {
        get_s_val!(displaymemory).set_at_addr((y * WIDTH as i32 + x + 19).abs() as u32, c);
    }
}

fn to_safe_rect(
    mut x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
) -> Option<(i32, i32, i32, i32)> {
    x1 += get_s_val_c!(OX);
    x2 += get_s_val_c!(OX);
    y1 += get_s_val_c!(OY);
    y2 += get_s_val_c!(OY);
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
    let mem = get_s_val!(displaymemory);
    let c = mem.get_at_addr_d(color.unwrap_or(0) as u32);
    for i in 0..SCREEN_MEM_LEN {
        get_s_val!(displaymemory).set_at_addr(i + 19, c);
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
    !(x < 0
        || x < get_s_val_c!(MIN_X)
        || x >= WIDTH as i32
        || x >= get_s_val_c!(MAX_X)
        || y < 0
        || y < get_s_val_c!(MIN_Y)
        || y >= HEIGHT as i32
        || y >= get_s_val_c!(MAX_Y))
}

pub fn line(mut x1: i32, mut y1: i32, mut x2: i32, mut y2: i32, color: u8) {
    x1 += get_s_val_c!(OX);
    x2 += get_s_val_c!(OX);
    y1 += get_s_val_c!(OY);
    y2 += get_s_val_c!(OY);

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

pub fn camera(x: Option<i32>, y: Option<i32>) {
    if let Some(_x) = x {
        set_s_val!(OX, _x);
    }
    if let Some(_y) = y {
        set_s_val!(OY, _y);
    }
    if y.is_none() && x.is_none() {
        set_s_val!(OX, 0);
        set_s_val!(OY, 0);
    }
}

pub fn pal(col1: Option<u8>, col2: Option<u8>) {
    let displaymem = get_s_val!(displaymemory);
    if let Some(c1) = col1 {
        let c2 = col2.unwrap_or(c1);
        displaymem.set_at_addr((c1 % 16) as u32, c2 % 16);
    } else {
        for i in 0..16 {
            displaymem.set_at_addr(i, i as u8);
        }
    }
}

pub fn palt(col1: Option<u8>, transparency: Option<bool>) {
    let displaymem = get_s_val!(displaymemory);
    if let Some(c1) = col1 {
        let t = transparency.unwrap_or(if c1 == 0 { true } else { false });
        let mut byte = displaymem.get_at_addr_d(17 + c1 as u32 / 8);
        byte &= 0xff ^ (1 << (c1 % 8));
        if t {
            byte |= 1 << c1 % 8;
        }
        displaymem.set_at_addr(17 + c1 as u32 / 8, byte);
    } else {
        displaymem.set_at_addr(16, 1);
        displaymem.set_at_addr(17, 0);
    }
}

pub fn switch_palette(palette: u8) {
    get_s_val!(displaymemory).set_at_addr(16, palette % 4);
}

#[inline(always)]
pub fn get_color(mut color: u8) -> Option<u8> {
    color %= 16;
    let displaymem = get_s_val!(displaymemory);
    if (displaymem.get_at_addr_d(17 + color as u32 / 8) >> color % 8) & 1 > 0 {
        None
    } else {
        Some(displaymem.get_at_addr_d(color as u32))
    }
}

static mut CURSORX: i32 = 0;
static mut CURSORY: i32 = 0;

pub fn cursor(x: Option<i32>, y: Option<i32>) {
    if x.is_none() && y.is_none() {
        unsafe {
            CURSORX = 0;
            CURSORY = 0;
        }
    }
    if let Some(_x) = x {
        unsafe {
            CURSORX = _x;
        }
    }
    if let Some(_y) = y {
        unsafe {
            CURSORY = _y;
        }
    }
}

pub fn print<T: Into<String>>(text: T, x: Option<i32>, y: Option<i32>, color: Option<u8>) {
    if x.is_some() || y.is_some() {
        // we have to check because cursor() will reset the cursor to 0 0, which we don't want in that case
        cursor(x, y);
    }
    let text: String = text.into();
    let bytes: Vec<char> = text.chars().collect();

    let mut i: usize = 0;

    let col = color.unwrap_or(match get_s_val!(displaymemory).get_at_addr_d(16) % 4 {
        2 | 3 => 15,
        _ => 12,
    });

    while i < bytes.len() {
        let char = get_char(bytes[i]);
        match bytes[i] {
            '\n' => cursor(Some(x.unwrap_or(0)), Some(unsafe { CURSORY } + 6)),
            _ => unsafe {
                put_char_on_canvas(&char, CURSORX, CURSORY, col);

                CURSORX += 4;
            },
        };
        i += 1;
    }
    cursor(Some(x.unwrap_or(0)), Some(unsafe { CURSORY } + 6));
}

pub fn sspr(sx: i32, sy: i32, x: u32, y: u32, w: u32, h: u32) {
    if x >= 128 || y >= 128 {
        return;
    }

    let vec = get_image_vec();

    for oy in 0..h {
        if y + oy >= 128 {
            break;
        }
        for ox in 0..w {
            if x + ox >= 128 {
                break;
            }

            let off = (y + oy) * 128 + x + ox;
            set_pixel(sx + ox as i32, sy + oy as i32, vec[off as usize]);
        }
    }
}

pub fn spr(idx: u32, x: i32, y: i32) {
    if idx >= 255 {
        return;
    }
    sspr(x, y, idx % 16 * 8, idx / 16 * 8, 8, 8);
}
