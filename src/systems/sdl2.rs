use crate::{
    audio::get_amplitude,
    c_singleton,
    canvas_functions::sdl_apply_canvas,
    get_s_val,
    overlay::overlay::{is_overlay_active, ov_write_to_sdl},
    set_s_val,
    system::{Event, Keycode, MouseButton},
    update, Singleton, HEIGHT, WIDTH,
};
use sdl2::{
    audio::{AudioCallback, AudioSpecDesired, AudioStatus},
    keyboard::Keycode as sdl_keycode,
    mouse::MouseButton as sdl_mousebutton,
    pixels::Color,
    rect::Rect,
    Sdl, render::WindowCanvas,
};
use std::{fs::read, path::Path};

static BLACK: Color = Color::RGB(0, 0, 0);
c_singleton!(WINDOW_CANVAS, Option<WindowCanvas>, || None);
c_singleton!(SDL_CTX, Option<Sdl>, || None);

pub fn get_size() -> (u32, u32) {
    get_s_val!(WINDOW_CANVAS).as_ref().unwrap().window().size()
}

pub fn init() {
    let sdl_context = sdl2::init().expect("Could not initialize SDL2");
    set_s_val!(SDL_CTX, Some(sdl_context));
    let sdl_context = get_s_val!(SDL_CTX).as_ref().unwrap();
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

    let windowcanvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .expect("Could not initialize the canvas");
    set_s_val!(WINDOW_CANVAS, Some(windowcanvas));
    let windowcanvas = get_s_val!(WINDOW_CANVAS).as_mut().unwrap();

    let mut event_pump = sdl_context
        .event_pump()
        .expect("Could not get the event pump");
    
    
    windowcanvas.set_draw_color(BLACK);
    windowcanvas.clear();

    let texturecreator = windowcanvas.texture_creator();
    let mut texture = texturecreator
        .create_texture(None, sdl2::render::TextureAccess::Target, 200, 180)
        .expect("Could not create texture!");

    let mouse_util = sdl_context.mouse();

    mouse_util.show_cursor(false);

    let device = sdl_context
        .audio()
        .expect("Failed to initialize the audio")
        .open_playback(None, &DESIRED_SPEC, |_| SimpleDevice())
        .expect("Failed to create the sfx audio device");

    'running: loop {
        if device.status() == AudioStatus::Paused {
            device.resume();
        }
        let mut events: Vec<Event> = Vec::new();

        for event in event_pump.poll_iter() {
            if event.get_window_id().unwrap_or(w_id) == w_id {
                match event {
                    sdl2::event::Event::Quit { .. } => {
                        break 'running;
                    }
                    sdl2::event::Event::DropFile { filename, .. } => {
                        println!("{}", filename);
                        match read(Path::new(&filename)) {
                            Err(e) => println!("failed to read the dropped file: {e}"),
                            Ok(file_data) => events.push(Event::Drop {
                                file_data,
                                name: filename,
                            }),
                        }
                    }
                    sdl2::event::Event::KeyDown {
                        keycode: Some(k), ..
                    } => {
                        if let Some(key) = sdl_keycode_to_keycode(k) {
                            events.push(Event::Keydown { keycode: key });
                        }
                    }
                    sdl2::event::Event::KeyUp {
                        keycode: Some(k), ..
                    } => {
                        if let Some(key) = sdl_keycode_to_keycode(k) {
                            events.push(Event::Keyup { keycode: key });
                        }
                    }
                    sdl2::event::Event::TextInput { text, .. } => {
                        for char in text.chars() {
                            events.push(Event::Text { char });
                        }
                    }
                    sdl2::event::Event::MouseButtonDown {
                        mouse_btn, x, y, ..
                    } => {
                        if let Some(btn) = sdl_mousebutton_to_mousebutton(mouse_btn) {
                            events.push(Event::MouseDown { button: btn, x, y });
                        }
                    }
                    sdl2::event::Event::MouseButtonUp {
                        mouse_btn, x, y, ..
                    } => {
                        if let Some(btn) = sdl_mousebutton_to_mousebutton(mouse_btn) {
                            events.push(Event::MouseUp { button: btn, x, y });
                        }
                    }
                    sdl2::event::Event::MouseWheel { x, y, .. } => {
                        events.push(Event::Scroll { x, y });
                    }
                    sdl2::event::Event::MouseMotion { x, y, .. } => {
                        events.push(Event::MouseMove { x, y });
                    }
                    _ => {}
                }
            }
        }

        update(events);

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

        windowcanvas.set_draw_color(BLACK);
        windowcanvas.clear();

        let sz = windowcanvas.window().size();
        let mult = (sz.0 / WIDTH).min(sz.1 / HEIGHT);
        windowcanvas
            .copy(&texture, None, Rect::new(0, 0, WIDTH * mult, HEIGHT * mult))
            .expect("Could not update window");
        windowcanvas.present();
    }
}

fn sdl_mousebutton_to_mousebutton(sdl_mousebutton: sdl_mousebutton) -> Option<MouseButton> {
    match sdl_mousebutton {
        sdl_mousebutton::Left => Some(MouseButton::Left),
        sdl_mousebutton::Middle => Some(MouseButton::Middle),
        sdl_mousebutton::Right => Some(MouseButton::Right),
        _ => None,
    }
}

pub fn show_cursor(value: bool) {
    get_s_val!(SDL_CTX).as_ref().unwrap().mouse().show_cursor(value);
}

fn sdl_keycode_to_keycode(sdl_keycode: sdl_keycode) -> Option<Keycode> {
    match sdl_keycode {
        sdl_keycode::Backspace | sdl_keycode::KpBackspace => Some(Keycode::Backspace),
        sdl_keycode::Tab => Some(Keycode::Tab),
        sdl_keycode::Return | sdl_keycode::Return2 | sdl_keycode::KpEnter => Some(Keycode::Return),
        sdl_keycode::Escape => Some(Keycode::Escape),
        sdl_keycode::Space | sdl_keycode::KpSpace => Some(Keycode::Space),
        sdl_keycode::Num0 | sdl_keycode::Kp0 => Some(Keycode::Num0),
        sdl_keycode::Num1 | sdl_keycode::Kp1 => Some(Keycode::Num1),
        sdl_keycode::Num2 | sdl_keycode::Kp2 => Some(Keycode::Num2),
        sdl_keycode::Num3 | sdl_keycode::Kp3 => Some(Keycode::Num3),
        sdl_keycode::Num4 | sdl_keycode::Kp4 => Some(Keycode::Num4),
        sdl_keycode::Num5 | sdl_keycode::Kp5 => Some(Keycode::Num5),
        sdl_keycode::Num6 | sdl_keycode::Kp6 => Some(Keycode::Num6),
        sdl_keycode::Num7 | sdl_keycode::Kp7 => Some(Keycode::Num7),
        sdl_keycode::Num8 | sdl_keycode::Kp8 => Some(Keycode::Num8),
        sdl_keycode::Num9 | sdl_keycode::Kp9 => Some(Keycode::Num9),
        sdl_keycode::A => Some(Keycode::A),
        sdl_keycode::B => Some(Keycode::B),
        sdl_keycode::C => Some(Keycode::C),
        sdl_keycode::D => Some(Keycode::D),
        sdl_keycode::E => Some(Keycode::E),
        sdl_keycode::F => Some(Keycode::F),
        sdl_keycode::G => Some(Keycode::G),
        sdl_keycode::H => Some(Keycode::H),
        sdl_keycode::I => Some(Keycode::I),
        sdl_keycode::J => Some(Keycode::J),
        sdl_keycode::K => Some(Keycode::K),
        sdl_keycode::L => Some(Keycode::L),
        sdl_keycode::M => Some(Keycode::M),
        sdl_keycode::N => Some(Keycode::N),
        sdl_keycode::O => Some(Keycode::O),
        sdl_keycode::P => Some(Keycode::P),
        sdl_keycode::Q => Some(Keycode::Q),
        sdl_keycode::R => Some(Keycode::R),
        sdl_keycode::S => Some(Keycode::S),
        sdl_keycode::T => Some(Keycode::T),
        sdl_keycode::U => Some(Keycode::U),
        sdl_keycode::V => Some(Keycode::V),
        sdl_keycode::W => Some(Keycode::W),
        sdl_keycode::X => Some(Keycode::X),
        sdl_keycode::Y => Some(Keycode::Y),
        sdl_keycode::Z => Some(Keycode::Z),
        sdl_keycode::Delete => Some(Keycode::Delete),
        sdl_keycode::CapsLock => Some(Keycode::CapsLock),
        sdl_keycode::F1 => Some(Keycode::F1),
        sdl_keycode::F2 => Some(Keycode::F2),
        sdl_keycode::F3 => Some(Keycode::F3),
        sdl_keycode::F4 => Some(Keycode::F4),
        sdl_keycode::F5 => Some(Keycode::F5),
        sdl_keycode::F6 => Some(Keycode::F6),
        sdl_keycode::F7 => Some(Keycode::F7),
        sdl_keycode::F8 => Some(Keycode::F8),
        sdl_keycode::F9 => Some(Keycode::F9),
        sdl_keycode::F10 => Some(Keycode::F10),
        sdl_keycode::F11 => Some(Keycode::F11),
        sdl_keycode::F12 => Some(Keycode::F12),
        sdl_keycode::Right => Some(Keycode::Right),
        sdl_keycode::Left => Some(Keycode::Left),
        sdl_keycode::Down => Some(Keycode::Down),
        sdl_keycode::Up => Some(Keycode::Up),
        sdl_keycode::LCtrl | sdl_keycode::RCtrl => Some(Keycode::Ctrl),
        sdl_keycode::LShift | sdl_keycode::RShift => Some(Keycode::Shift),
        sdl_keycode::RAlt => Some(Keycode::AltGr),
        sdl_keycode::LAlt => Some(Keycode::Alt),
        _ => None,
    }
}

static DESIRED_SPEC: AudioSpecDesired = AudioSpecDesired {
    channels: Some(1),
    freq: Some(44100),
    samples: None,
};

struct SimpleDevice();

impl AudioCallback for SimpleDevice {
    type Channel = f32;

    fn callback(&mut self, x: &mut [Self::Channel]) {
        get_amplitude(x);
    }
}

pub fn copy_to_clipboard(str: &str) {
    get_s_val!(SDL_CTX)
        .as_ref()
        .expect("Failed to obtain the sdl context")
        .video()
        .expect("Failed to obtain the video context")
        .clipboard()
        .set_clipboard_text(str)
        .err();
}

pub fn read_clipboard() -> String {
    let clip = get_s_val!(SDL_CTX)
        .as_ref()
        .expect("Failed to obtain the sdl context")
        .video()
        .expect("Failed to obtain the video context")
        .clipboard();

    if !clip.has_clipboard_text() {
        String::new()
    } else {
        clip.clipboard_text().expect("Failed to read the clipboard")
    }
}
