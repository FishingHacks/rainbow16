use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use crate::audio::{is_muted, set_muted};
use crate::canvas_functions::PALETTE1;
use crate::gamestate::game_is_running;
use crate::get_s_val;
use crate::memory::keymemory;
use crate::utils::is_ctrl_pressed;

use super::canvas_functions::{clear, cursor};
use super::editor_manager::{
    handle_key as handle_key_editor, handle_mousedown as handle_mousedown_editor,
    handle_mousemove as handle_mousemove_editor, handle_scroll as handle_scroll_editor,
    init as init_editor, render as render_editor, update as update_editor,
};
use super::explore::{init as init_explore, render as render_explore, update as update_explore};
use super::globals::{OverlayType, DISPLAYMEM, OVERLAY};
use super::message::{render as rendermessage, set_message};
use super::mouse_cursor::{mousemove as mousemove_cursor, render as render_cursor};
use super::options::{cycle_palette, init as initopt, render as renderopt, update as updateopt};
use super::pause_menu::{init as initpm, render as renderpm, update as updatepm};
use super::terminal::{
    handle_key as term_handle_key, init as initterm, render as renderterm, update as updateterm,
};

pub fn hide_overlay() {
    unsafe {
        OVERLAY.set(OverlayType::None);
    }
}

pub fn is_overlay_active() -> bool {
    (*unsafe { OVERLAY.get() }) != OverlayType::None || !game_is_running()
}

pub fn set_overlay(new: OverlayType) {
    match new {
        OverlayType::PauseMenu => initpm(),
        OverlayType::Options => initopt(),
        OverlayType::None => initterm(),
        OverlayType::CodeEditor => init_editor(),
        OverlayType::Explore => init_explore(),
    }
    unsafe { OVERLAY.set(new) };
}

pub fn renderoverlay() {
    clear(Some(0));
    if is_overlay_active() {
        cursor(None, None);
        let overlaytype = unsafe { OVERLAY.get() };
        match overlaytype {
            OverlayType::PauseMenu => renderpm(),
            OverlayType::Options => renderopt(),
            OverlayType::None => renderterm(),
            OverlayType::CodeEditor => render_editor(),
            OverlayType::Explore => render_explore(),
        }
    }
    if is_overlay_active() || get_s_val!(keymemory).get_at_addr_d(0x3b) > 0 {
        render_cursor();
    }
    rendermessage();
}

pub fn updateoverlay() {
    if !is_overlay_active() {
        return;
    }
    let overlaytype = unsafe { OVERLAY.get() };
    match overlaytype {
        OverlayType::PauseMenu => updatepm(),
        OverlayType::Options => updateopt(),
        OverlayType::None => updateterm(),
        OverlayType::CodeEditor => update_editor(),
        OverlayType::Explore => update_explore(),
    }
}

pub fn ov_write_to_sdl(mem: &mut Vec<u8>) {
    let imagemem = unsafe { DISPLAYMEM.get() };
    if mem.len() != imagemem.len() * 4 || is_overlay_active() {
        mem.clear();
        for i in 0..imagemem.len() {
            PALETTE1[imagemem[i] as usize].sdl_write_to_vec(mem);
        }
    } else {
        for i in 0..imagemem.len() {
            if imagemem[i] != 0 {
                let (r, g, b) = PALETTE1[imagemem[i] as usize].get_values();
                let off = i * 4;
                mem[off] = b;
                mem[off + 1] = g;
                mem[off + 2] = r;
                mem[off + 3] = 255;
            }
        }
    }
}

pub fn ov_handle_keydown(key: Keycode) {
    if !match key {
        Keycode::Escape => {
            if !game_is_running() {
                false
            } else {
                if is_overlay_active() {
                    hide_overlay();
                } else {
                    set_overlay(OverlayType::PauseMenu);
                }
                true
            }
        }
        Keycode::P => {
            if is_ctrl_pressed() {
                cycle_palette(true);
                set_message("cycled palette (ctrl+p)");
                true
            } else {
                false
            }
        }
        Keycode::M => {
            if game_is_running() && is_ctrl_pressed() {
                set_muted(None);
                if is_muted() {
                    set_message("muted (ctrl+m)");
                } else {
                    set_message("unmuted (ctrl+m)");
                }
                true
            } else {
                false
            }
        }

        _ => false,
    } {
        if is_overlay_active() {
            match get_s_val!(OVERLAY) {
                OverlayType::None => term_handle_key(key),
                OverlayType::CodeEditor => handle_key_editor(key),
                _ => {}
            };
        }
    }
}

pub fn ov_handle_keyup(key: Keycode) {
    if !match key {
        _ => false,
    } {
        if is_overlay_active() {
            match get_s_val!(OVERLAY) {
                _ => {}
            }
        }
    }
}

pub fn ov_handle_scroll(dy: i32) {
    if !is_overlay_active() {
        return;
    }
    match get_s_val!(OVERLAY) {
        OverlayType::CodeEditor => handle_scroll_editor(dy),
        _ => {}
    }
}

pub fn ov_handle_mousedown(button: MouseButton, x: u32, y: u32) {
    if !is_overlay_active() {
        return;
    }
    match get_s_val!(OVERLAY) {
        OverlayType::CodeEditor => handle_mousedown_editor(button, x, y),
        _ => {}
    }
}

pub fn ov_handle_mousemove(x: u32, y: u32) {
    if is_overlay_active() || get_s_val!(keymemory).get_at_addr_d(0x3b) > 0 {
        mousemove_cursor(x, y);
    }
    if !is_overlay_active() {
        return;
    }
    match get_s_val!(OVERLAY) {
        OverlayType::CodeEditor => handle_mousemove_editor(x, y),
        _ => {}
    };
}
