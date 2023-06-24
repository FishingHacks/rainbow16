use crate::{c_singleton, image::{Image, parse_image}, get_s_val, Singleton};
use super::{canvas_functions::set_pixel, globals::OVERLAY, OverlayType, editor_manager::{CURRENT_EDITOR, Editor}, overlay::is_overlay_active};

static mut M_X: u32 = 0;
static mut M_Y: u32 = 0;

c_singleton!(CURSOR_NORMAL, Image, ||parse_image(6, 7, "0f0000fcf000fccf00fcccf0fccccffccff00ffcf0".to_string()).unwrap());
c_singleton!(CURSOR_HIGHLIGHT, Image, ||parse_image(7, 7, "000f00000fcf000f000f0fc000cf0f000f000fcf00000f000".to_string()).unwrap());

pub fn render() {
    let (mut x, mut y) = unsafe {(M_X as i32, M_Y as i32)};
    if get_s_val!(OVERLAY) == &OverlayType::CodeEditor && is_overlay_active() && get_s_val!(CURRENT_EDITOR) == &Editor::Sprites && x >= 71 && x < 153 && y >= 22 && y < 104 {
        x -= 2;
        y -= 3;
        get_s_val!(CURSOR_HIGHLIGHT).put_on_canvas(custom_set_pixel, x, y);
    } else {
        x -= 2;
        y -= 3;
        get_s_val!(CURSOR_NORMAL).put_on_canvas(custom_set_pixel, x, y);
    }
}

fn custom_set_pixel(x: i32, y: i32, color: u8) {
    if color == 0 || x < 0 || y < 0 || x >= 200 || y >= 180  {
        return;
    }
    set_pixel(x, y, color)
}

pub fn mousemove(x: u32, y: u32) {
    unsafe {
        M_X = x;
        M_Y = y;
    }
}