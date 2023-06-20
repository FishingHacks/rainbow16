use super::{
    background::{render_bg, update_bg},
    menu,
    overlay::{hide_overlay, set_overlay}, terminal::add_line_to_stdout,
};
use crate::{
    c_singleton,
    gamestate::{stop_game, run_game},
    get_s_val,
    luautils::print_err,
    Singleton,
};
use std::process::exit;

c_singleton!(ITEMS, Vec<String>, || {
    let mut v = <Vec<String>>::new();

    v.push("Continue game".to_string());
    v.push("Reset Game".to_string());
    v.push("Stop Game".to_string());
    v.push("".to_string());
    v.push("Options".to_string());
    v.push("".to_string());
    v.push("Quit Rainbow16".to_string());

    v
});

pub fn render() {
    render_bg();
    menu::render(get_s_val!(ITEMS));
}

pub fn init() {
    menu::reset();
}

pub fn update() {
    update_bg();
    if let Some(selected) = menu::update(get_s_val!(ITEMS)) {
        match selected {
            0 => hide_overlay(),
            1 => {
                stop_game();
                if let Some(err) = run_game() {
                    add_line_to_stdout(print_err(err));
                    stop_game();
                }
                hide_overlay();
            }
            2 => {
                stop_game();
                set_overlay(super::OverlayType::None);
            }
            4 => set_overlay(super::OverlayType::Options),
            6 => exit(0),
            _ => {}
        }
    }
}
