pub mod audio;
pub mod canvas_functions;
pub mod charmap;
pub mod custom_canvas_functions;
pub mod file_parser;
pub mod frequencies;
pub mod fs;
pub mod game_handle_key;
pub mod gamestate;
pub mod image;
pub mod info;
pub mod keyboard;
pub mod luastd;
pub mod luautils;
pub mod screenshot_saver;
pub mod sprites;
pub mod system;
pub mod systems;
pub mod utils;
pub mod waves;
#[macro_use]
pub mod memory;
pub mod overlay;
pub mod singleton;

// wasm

use fs::{create_dir, remove_file};
#[cfg(target_family = "wasm")]
use std::panic;
#[cfg(target_family = "wasm")]
use console_error_panic_hook;
#[cfg(target_family = "wasm")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use canvas_functions::*;
use dirs::home_dir;
use gamestate::{draw_game, update_game};
use info::{NAME, VERSION};
use keyboard::{
    handle_acc_keys_down, handle_acc_keys_up, handle_keydown, handle_keyup, handle_mousedown,
    handle_mousemove, handle_mouseup, handle_scroll, keyboard_update, reset_scroll,
};
use overlay::{
    add_line_to_stdout,
    overlay::{
        is_overlay_active, ov_handle_keydown, ov_handle_keyup, ov_handle_mousedown,
        ov_handle_mousemove, ov_handle_scroll, renderoverlay, updateoverlay,
    },
};
use rand::rngs::ThreadRng;
use rand::thread_rng;
use singleton::Singleton;
use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
use system::{get_size, init, show_cursor, Event, Keycode};

use crate::{
    game_handle_key::game_handle_keydown,
    gamestate::{load_game, run_game},
    keyboard::handle_textinput,
    memory::{charpress, init_memory_sections},
};

#[macro_export]
macro_rules! swap {
    ($x: expr, $y: expr) => {{
        let tmp = $x;
        $x = $y;
        $y = tmp;
    }};
}

static WIDTH: u32 = 200;
static HEIGHT: u32 = 180;

static TIME_STEP_MS: u32 = 1000 / 30;

pub_c_singleton!(TIME, u64, || 0);
c_singleton!(RNG, ThreadRng, thread_rng);

c_singleton!(PATH, PathBuf, || {
    let mut p = PathBuf::from(home_dir().expect("No homedir found!"));
    p.push("rainbow16");

    p
});

pub_c_singleton!(CARTSPATH, PathBuf, || get_s_val!(PATH).join("carts"));
pub_c_singleton!(EXPLORECACHEPATH, PathBuf, || get_s_val!(PATH)
    .join("explore_cache"));
pub_c_singleton!(LOGSPATH, PathBuf, || get_s_val!(PATH).join("logs"));
pub_c_singleton!(SCREENSHOTSPATH, PathBuf, || get_s_val!(PATH)
    .join("screenshots"));

fn create_dir_if_necessary(path: &PathBuf) -> Result<(), ()> {
    if !path.exists() {
        create_dir(path)?;
    } else if !path.is_dir() {
        remove_file(path)?;
        create_dir(path)?;
    }

    Ok(())
}

fn setup_folders() -> Result<(), ()> {
    create_dir_if_necessary(get_s_val!(PATH))?;
    create_dir_if_necessary(get_s_val!(CARTSPATH))?;
    create_dir_if_necessary(get_s_val!(EXPLORECACHEPATH))?;
    create_dir_if_necessary(get_s_val!(LOGSPATH))?;
    create_dir_if_necessary(get_s_val!(SCREENSHOTSPATH))?;

    Ok(())
}

fn real_coordinates_to_pixels(realx: u32, realy: u32, window_sz: (u32, u32)) -> Option<(u32, u32)> {
    let mult = (window_sz.0 / WIDTH).min(window_sz.1 / HEIGHT);

    let x = realx / mult;
    let y = realy / mult;

    if y > 180 || x > 200 {
        None
    } else {
        Some((x, y))
    }
}

pub fn main() {
    #[cfg(target_family="wasm")]
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    if let Err(..) = setup_folders() {
        eprintln!("Failed to setup the folders!");
        std::process::exit(1);
    }
    init_memory_sections();

    add_line_to_stdout(format!("{} {}", NAME, VERSION));

    init();
}

static mut NEXT_GAME_STEP: u32 = 0;

pub fn update(events: Vec<Event>) {
    let mut keyup_events: Vec<Keycode> = vec![];
    let mut keydown_events: Vec<Keycode> = vec![];
    handle_textinput('\0');

    for event in events {
        match event {
            Event::Drop { file_data, name } => {
                if !load_game(file_data, Some(name)) {
                    println!("failed to load the game");
                } else {
                    if let Some(err) = run_game() {
                        println!("failed to start the game: {err}");
                    }
                }
            }
            Event::Keydown { keycode } => keydown_events.push(keycode),
            Event::Keyup { keycode } => keyup_events.push(keycode),
            Event::Text { char } => handle_textinput(char),
            Event::MouseDown { button, x, y, .. } => {
                if x >= 0 && y >= 0 {
                    if let Some((x, y)) = real_coordinates_to_pixels(x as u32, y as u32, get_size())
                    {
                        handle_mousedown(button);
                        ov_handle_mousedown(button, x, y);
                    }
                }
            }
            Event::MouseUp { button, .. } => {
                handle_mouseup(button);
            }
            Event::Scroll { y, .. } => {
                handle_scroll(y);
                ov_handle_scroll(y);
            }
            Event::MouseMove { x, y, .. } => {
                if x >= 0 && y >= 0 {
                    if let Some((x, y)) = real_coordinates_to_pixels(x as u32, y as u32, get_size())
                    {
                        show_cursor(false);
                        ov_handle_mousemove(x, y);
                        handle_mousemove(x, y);
                    } else {
                        show_cursor(true);
                    }
                } else {
                    show_cursor(true);
                }
            }
        }
    }

    if get_s_val!(charpress).get_at_addr_u32_d(0) > 0 && keydown_events.len() < 1 {
        keydown_events.push(Keycode::Unknown);
    }

    for key in keydown_events {
        game_handle_keydown(key);
        handle_acc_keys_down(key);
        ov_handle_keydown(key);
        handle_keydown(key);
    }

    for key in keyup_events {
        handle_acc_keys_up(key);
        ov_handle_keyup(key);
        handle_keyup(key);
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards, wut")
        .as_millis() as u32;
    if unsafe { NEXT_GAME_STEP } <= now {
        unsafe {
            NEXT_GAME_STEP += TIME_STEP_MS;
            if NEXT_GAME_STEP <= now {
                NEXT_GAME_STEP = now;
            }
        }

        keyboard_update();

        updateoverlay();
        if !is_overlay_active() {
            set_s_val!(TIME, *get_s_val!(TIME) + 1u64);
            update_game();
        }
        reset_scroll();

        renderoverlay();
        if !is_overlay_active() {
            cursor(None, None);
            draw_game();
        }
    }
}
