use super::{
    background::{render_bg, update_bg},
    menu,
    overlay::set_overlay,
};
use crate::{
    c_singleton, get_s_val,
    keyboard::button_is_pressed,
    memory::displaymemory,
    audio::{get_volume, is_muted, set_volume, set_muted},
    utils::cycle,
    Singleton,
};

c_singleton!(ITEMS, Vec<String>, || {
    let mut vec: Vec<String> = Vec::new();

    vec.push("palette".to_string());
    vec.push("volume".to_string());
    vec.push("".to_string());
    vec.push("back".to_string());

    vec
});

pub fn render() {
    render_bg();
    let mut arr = <Vec<String>>::new();
    arr.push(format!(
        "Palette: {}",
        get_s_val!(displaymemory).get_at_addr_d(0x10)
    ));
    if get_volume() > 0 {
        arr.push(format!("Volume: {}%", get_volume()));
    } else {
        arr.push("Volume: ---%".to_string());
    }
    arr.push("".to_string());
    arr.push("Back".to_string());

    menu::render(&arr);
}

pub fn init() {
    menu::reset();
}

pub fn update() {
    update_bg();
    if let Some(selected) = menu::update(get_s_val!(ITEMS)) {
        match selected {
            0 => {
                if button_is_pressed(crate::keyboard::Button::Left) {
                    cycle_palette(false);
                }
                if button_is_pressed(crate::keyboard::Button::Right)
                    || button_is_pressed(crate::keyboard::Button::A)
                {
                    cycle_palette(true);
                }
            }
            1 => {
                if !is_muted() {
                    if button_is_pressed(crate::keyboard::Button::Left) && get_volume() > 0 {
                        set_volume(get_volume() - 1);
                    }
                    if button_is_pressed(crate::keyboard::Button::Right) {
                        set_volume(get_volume() + 1);
                    }
                }
                if button_is_pressed(crate::keyboard::Button::A) {
                    set_muted(None);
                }
            }
            3 => set_overlay(super::OverlayType::PauseMenu),
            _ => {}
        }
    }
}

pub fn cycle_palette(right: bool) {
    let mut current_palette = get_s_val!(displaymemory).get_at_addr_d(0x10);
    current_palette = cycle(current_palette, 0, 4, right);
    get_s_val!(displaymemory).set_at_addr(0x10, current_palette);
}
