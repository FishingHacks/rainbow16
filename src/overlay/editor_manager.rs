use super::editor::{
    handle_key as handle_key_code, handle_mousedown as handle_mousedown_code,
    handle_scroll as handle_scroll_code, init as init_code, render as render_code,
};
use super::overlay::hide_overlay;
use super::spr::{render as render_spr, keydown as keydown_spr, mousedown as mousedown_spr, mousemove as handle_mousemove_spr};
use sdl2::{keyboard::Keycode, mouse::MouseButton};

use super::canvas_functions::*;

use crate::sprites::LOGO_BG_RED_FG_PURP;
use crate::{
    c_singleton, get_s_val,
    image::{parse_image, Image},
    pub_c_singleton, set_s_val, Singleton,
};

pub_c_singleton!(IMAGE_CEDIT, Image, || parse_image(
    10,
    7,
    "2222222222222c22c22222c2222c222cc2222cc222c2222c22222c22c2222222222222".to_string()
)
.unwrap());
pub_c_singleton!(IMAGE_CEDIT_SEL, Image, || parse_image(
    10,
    7,
    "3333333333333c33c33333c3333c333cc3333cc333c3333c33333c33c3333333333333".to_string()
)
.unwrap());
pub_c_singleton!(IMAGE_SFXEDIT, Image, || parse_image(
    10,
    7,
    "222222222222ccc222222222c222222222c222222222cccc222222cccc222222222222".to_string()
)
.unwrap());
pub_c_singleton!(IMAGE_SFXEDIT_SEL, Image, || parse_image(
    10,
    7,
    "333333333333ccc333333333c333333333c333333333cccc333333cccc333333333333".to_string()
)
.unwrap());
pub_c_singleton!(IMAGE_SPREDIT, Image, || parse_image(
    9,
    7,
    "222222222222ccc22222c2c2c2222ccccc2222ccccc2222c2c2c22222222222".to_string()
)
.unwrap());
pub_c_singleton!(IMAGE_SPREDIT_SEL, Image, || parse_image(
    9,
    7,
    "333333333333ccc33333c3c3c3333ccccc3333ccccc3333c3c3c33333333333".to_string()
)
.unwrap());

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum Editor {
    #[default]
    Code,
    Sfx,
    Sprites,
}

impl Editor {
    fn to_string(&self) -> String {
        match self {
            Editor::Code => "code editor",
            Editor::Sfx => "sounds editor",
            Editor::Sprites => "sprite editor",
        }
        .to_string()
    }
}

c_singleton!(CURRENT_EDITOR, Editor, || Editor::default());

fn render_titlebar() {
    let cur_sel = get_s_val!(CURRENT_EDITOR);
    rectfill(0, 0, 200, 7, 2);
    get_s_val!(LOGO_BG_RED_FG_PURP).put_on_canvas(set_pixel, 1, 0);
    print(&cur_sel.to_string(), Some(12), Some(1), None);
    if cur_sel == &Editor::Code {
        get_s_val!(IMAGE_CEDIT_SEL).put_on_canvas(set_pixel, 190, 0);
    } else {
        get_s_val!(IMAGE_CEDIT).put_on_canvas(set_pixel, 190, 0);
    }
    if cur_sel == &Editor::Sfx {
        get_s_val!(IMAGE_SFXEDIT_SEL).put_on_canvas(set_pixel, 180, 0);
    } else {
        get_s_val!(IMAGE_SFXEDIT).put_on_canvas(set_pixel, 180, 0);
    }
    if cur_sel == &Editor::Sprites {
        get_s_val!(IMAGE_SPREDIT_SEL).put_on_canvas(set_pixel, 170, 0);
    } else {
        get_s_val!(IMAGE_SPREDIT).put_on_canvas(set_pixel, 170, 0);
    }
}

pub fn handle_mousedown(button: MouseButton, x: u32, y: u32) {
    if y <= 7 && button == MouseButton::Left {
        if x >= 190 {
            set_s_val!(CURRENT_EDITOR, Editor::Code);
        } else if x >= 180 {
            set_s_val!(CURRENT_EDITOR, Editor::Sfx);
        } else if x >= 170 {
            set_s_val!(CURRENT_EDITOR, Editor::Sprites);
        }
    } else {
        match get_s_val!(CURRENT_EDITOR) {
            Editor::Code => handle_mousedown_code(button, x, y),
            Editor::Sprites => mousedown_spr(button, x, y),
            _ => {}
        }
    }
}

pub fn handle_scroll(dy: i32) {
    match get_s_val!(CURRENT_EDITOR) {
        Editor::Code => handle_scroll_code(dy),
        _ => {}
    }
}

pub fn handle_key(key: Keycode) {
    if key == Keycode::Escape {
        hide_overlay();
        return;
    }
    match get_s_val!(CURRENT_EDITOR) {
        Editor::Code => handle_key_code(key),
        Editor::Sprites => keydown_spr(key),
        _ => {}
    }
}

pub fn render() {
    match get_s_val!(CURRENT_EDITOR) {
        Editor::Code => render_code(),
        Editor::Sprites => render_spr(),
        _ => {}
    }
    render_titlebar();
}

pub fn update() {
    match get_s_val!(CURRENT_EDITOR) {
        _ => {}
    }
}

pub fn init() {
    set_s_val!(CURRENT_EDITOR, Editor::Code);
    init_code();
}

pub fn handle_mousemove(x: u32, y: u32) {
    match get_s_val!(CURRENT_EDITOR) {
        Editor::Sprites => handle_mousemove_spr(x, y),
        _ => {}
    };
}