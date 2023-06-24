pub mod audio;
pub mod canvas_functions;
pub mod charmap;
pub mod file_parser;
pub mod frequencies;
pub mod gamestate;
pub mod image;
pub mod info;
pub mod keyboard;
pub mod luastd;
pub mod luautils;
pub mod sprites;
pub mod utils;
pub mod waves;
#[macro_use]
pub mod memory;
pub mod overlay;
pub mod singleton;

use std::{
    fs::{create_dir, remove_file},
    io::Error as IoErr,
    path::PathBuf,
};

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
        ov_handle_mousemove, ov_handle_scroll, ov_write_to_sdl, renderoverlay, updateoverlay,
    },
};
use rand::rngs::ThreadRng;
use rand::thread_rng;
use sdl2::{event::Event, video::Window};
use sdl2::{keyboard::Keycode, rect::Rect};
use sdl2::{pixels::Color, render::WindowCanvas};
use singleton::Singleton;

use crate::{
    audio::tick_audio, keyboard::handle_textinput, memory::{charpress, init_memory_sections}
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
static BLACK: Color = Color::RGB(0, 0, 0);

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

fn create_dir_if_necessary(path: &PathBuf) -> Result<(), IoErr> {
    if !path.exists() {
        create_dir(path)?;
    } else if !path.is_dir() {
        remove_file(path)?;
        create_dir(path)?;
    }

    Ok(())
}

fn setup_folders() -> Result<(), IoErr> {
    create_dir_if_necessary(get_s_val!(PATH))?;
    create_dir_if_necessary(get_s_val!(CARTSPATH))?;
    create_dir_if_necessary(get_s_val!(EXPLORECACHEPATH))?;
    create_dir_if_necessary(get_s_val!(LOGSPATH))?;

    Ok(())
}

fn real_coordinates_to_pixels(realx: u32, realy: u32, window: &Window) -> Option<(u32, u32)> {
    let sz = window.size();
    let mult = min(sz.0 / WIDTH, sz.1 / HEIGHT);

    let x = realx / mult;
    let y = realy / mult;

    if y > 180 || x > 200 {
        None
    } else {
        Some((x, y))
    }
}

pub_c_singleton!(WINDOW_CANVAS, Option<WindowCanvas>, || None);
pub_c_singleton!(SDL_CONTEXT, Option<sdl2::Sdl>, || None);

fn main() {
    if let Err(e) = setup_folders() {
        eprintln!("Failed to setup the folders:\n{e}");
        std::process::exit(1);
    }
    init_memory_sections();

    set_s_val!(
        SDL_CONTEXT,
        Some(sdl2::init().expect("Could not initialize SDL2"))
    );
    let sdl_context = get_s_val!(SDL_CONTEXT)
        .as_ref()
        .expect("Failed to obtainm the std context");
    let video = sdl_context
        .video()
        .expect("Could not get the video context");

    let window = video
        .window("Rainbow16", WIDTH, HEIGHT)
        .resizable()
        .position_centered()
        .build()
        .expect("Could not initialize the window");

    let w_id = window.id();

    set_s_val!(
        WINDOW_CANVAS,
        Some(
            window
                .into_canvas()
                .present_vsync()
                .build()
                .expect("Could not initialize the canvas")
        )
    );

    let windowcanvas = get_s_val!(WINDOW_CANVAS)
        .as_mut()
        .expect("Failed to obtain the window canvas");

    let mut event_pump = sdl_context
        .event_pump()
        .expect("Could not get the event pump");
    windowcanvas.set_draw_color(BLACK);
    windowcanvas.clear();

    let texturecreator = windowcanvas.texture_creator();
    let mut texture = texturecreator
        .create_texture(None, sdl2::render::TextureAccess::Target, 200, 180)
        .expect("Could not create texture!");

    let timer = sdl_context.timer().expect("Could not create timer!");
    let mut next_game_step = timer.ticks();

    add_line_to_stdout(format!("{} {}", NAME, VERSION));

    tick_audio();

    let mouse_util = sdl_context.mouse();

    mouse_util.show_cursor(false);

    'running: loop {
        handle_textinput('\0');
        let mut keyup_events: Vec<Keycode> = vec![];
        let mut keydown_events: Vec<Keycode> = vec![];
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(k),
                    window_id,
                    ..
                } => {
                    if window_id == w_id {
                        keydown_events.push(k);
                    }
                }
                Event::KeyUp {
                    keycode: Some(k), ..
                } => {
                    keyup_events.push(k);
                }
                Event::TextInput { text, .. } => {
                    let _char = if text.len() > 0 {
                        let char = text.chars().nth(0);
                        char.unwrap_or('\0')
                    } else {
                        '\0'
                    };
                    handle_textinput(_char);
                }
                Event::MouseButtonDown {
                    mouse_btn, x, y, ..
                } => {
                    if x >= 0 && y >= 0 {
                        if let Some((x, y)) =
                            real_coordinates_to_pixels(x as u32, y as u32, windowcanvas.window())
                        {
                            handle_mousedown(mouse_btn);
                            ov_handle_mousedown(mouse_btn, x, y);
                        }
                    }
                }
                Event::MouseButtonUp {
                    mouse_btn, ..
                } => {
                    handle_mouseup(mouse_btn);
                }
                Event::MouseWheel { y, .. } => {
                    handle_scroll(y);
                    ov_handle_scroll(y);
                }
                Event::MouseMotion { x, y, .. } => {
                    if x >= 0 && y >= 0 {
                        if let Some((x, y)) =
                            real_coordinates_to_pixels(x as u32, y as u32, windowcanvas.window())
                        {
                            mouse_util.show_cursor(false);
                            ov_handle_mousemove(x, y);
                            handle_mousemove(x, y);
                        } else {
                            mouse_util.show_cursor(true);
                        }
                    } else {
                        mouse_util.show_cursor(true);
                    }
                }
                _ => {}
            }
        }

        if get_s_val!(charpress).get_at_addr_u32_d(0) > 0 && keydown_events.len() < 1 {
            keydown_events.push(Keycode::Www);
        }

        for key in keydown_events {
            handle_acc_keys_down(key);
            ov_handle_keydown(key);
            handle_keydown(key);
        }

        for key in keyup_events {
            handle_acc_keys_up(key);
            ov_handle_keyup(key);
            handle_keyup(key);
        }

        tick_audio();

        let now = timer.ticks();

        if next_game_step <= now {
            keyboard_update();
            next_game_step += TIME_STEP_MS;

            if next_game_step <= now {
                next_game_step = now;
            }

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

            let mut vec = <Vec<u8>>::with_capacity((WIDTH * HEIGHT * 4) as usize);
            if !is_overlay_active() {
                sdl_apply_canvas(&mut vec);
            }
            ov_write_to_sdl(&mut vec);

            texture
                .update(None, &vec, (WIDTH * 4) as usize)
                .err()
                .and_then(|e| {
                    eprintln!("Error: Could not update the texture: {}", e);
                    Some(e)
                });
        }

        windowcanvas.set_draw_color(BLACK);
        windowcanvas.clear();

        let sz = windowcanvas.window().size();
        let mult = min(sz.0 / WIDTH, sz.1 / HEIGHT);
        windowcanvas
            .copy(&texture, None, Rect::new(0, 0, WIDTH * mult, HEIGHT * mult))
            .expect("Could not update window");
        windowcanvas.present();
    }
}

fn min(x: u32, y: u32) -> u32 {
    if x < y {
        x
    } else {
        y
    }
}
