use std::{
    io::{Error, Write},
    path::PathBuf,
};

use wasm_bindgen::prelude::*;

use crate::{
    canvas_functions::sdl_apply_canvas,
    main,
    overlay::overlay::{is_overlay_active, ov_write_to_sdl},
    system::{Event, Keycode, MouseButton},
    update as __update,
};

#[wasm_bindgen(module="/module.js")]
extern "C" {
    pub fn copy_to_clipboard(s: &str);
    pub fn read_clipboard() -> String;
    pub fn show_cursor(value: bool);
    fn __init();
    fn __update_canvas(vec: Vec<u8>);
    fn fs_create_dir(path: String) -> bool;
    fn fs_read(path: String) -> Option<Vec<u8>>;
    fn fs_read_dir(path: String) -> Option<Vec<u8>>;
    fn fs_remove_dir(path: String) -> bool;
    fn fs_remove_file(path: String) -> bool;
    fn fs_write(path: String, data: &[u8]) -> bool;
}

#[wasm_bindgen]
pub fn update() {
    unsafe {
        __update(EVENTS.clone());
        EVENTS.clear();
    }

    let mut vec: Vec<u8> = Vec::with_capacity(200 * 180 * 4);
    if !is_overlay_active() {
        sdl_apply_canvas(&mut vec);
    }
    ov_write_to_sdl(&mut vec);
    unsafe {__update_canvas(vec);}
}

pub fn init() {
    unsafe {
        __init();
    }
}

pub fn get_size() -> (u32, u32) {
    (200, 180)
}

static mut EVENTS: Vec<Event> = Vec::new();

#[wasm_bindgen]
pub fn push_event_drop(data: Vec<u8>, name: String) {
    unsafe {
        EVENTS.push(Event::Drop {
            file_data: data,
            name,
        })
    }
}

#[wasm_bindgen]
pub fn push_event_keydown(keycode: u8, key: String) {
    unsafe {
        if let Some(keycode) = js_keycode_to_keycode(keycode) {
            EVENTS.push(Event::Keydown { keycode });
        }
        if key.len() < 2 {
            if let Some(char) = key.chars().nth(0) {
                EVENTS.push(Event::Text { char });
            }
        }
    }
}

#[wasm_bindgen]
pub fn push_event_keyup(keycode: u8) {
    unsafe {
        if let Some(keycode) = js_keycode_to_keycode(keycode) {
            EVENTS.push(Event::Keyup { keycode });
        }
    }
}

#[wasm_bindgen]
pub fn push_event_mouse_down(mouse_button: u8, x: i32, y: i32) {
    unsafe {
        if let Some(button) = js_mousecode_to_mousebtn(mouse_button) {
            EVENTS.push(Event::MouseDown { button, x, y });
        }
    }
}

#[wasm_bindgen]
pub fn push_event_mouse_up(mouse_button: u8, x: i32, y: i32) {
    unsafe {
        if let Some(button) = js_mousecode_to_mousebtn(mouse_button) {
            EVENTS.push(Event::MouseUp { button, x, y });
        }
    }
}

#[wasm_bindgen]
pub fn push_event_scroll(y: i32, x: i32) {
    unsafe {
        EVENTS.push(Event::Scroll { x, y });
    }
}

#[wasm_bindgen]
pub fn push_event_mousemove(y: i32, x: i32) {
    unsafe {
        EVENTS.push(Event::MouseMove { x, y });
    }
}

fn js_mousecode_to_mousebtn(code: u8) -> Option<MouseButton> {
    match code {
        0 => Some(MouseButton::Left),
        1 => Some(MouseButton::Middle),
        2 => Some(MouseButton::Right),
        _ => None,
    }
}

fn js_keycode_to_keycode(keycode: u8) -> Option<Keycode> {
    match keycode {
        0 => Some(Keycode::Unknown),
        8 => Some(Keycode::Backspace),
        9 => Some(Keycode::Tab),
        13 => Some(Keycode::Return),
        16 => Some(Keycode::Shift),
        17 => Some(Keycode::Ctrl),
        18 => Some(Keycode::Alt),
        27 => Some(Keycode::Escape),
        32 => Some(Keycode::Space),
        37 => Some(Keycode::Left),
        38 => Some(Keycode::Up),
        39 => Some(Keycode::Right),
        40 => Some(Keycode::Down),
        46 => Some(Keycode::Delete),
        48 => Some(Keycode::Num0),
        49 => Some(Keycode::Num1),
        50 => Some(Keycode::Num2),
        51 => Some(Keycode::Num3),
        52 => Some(Keycode::Num4),
        53 => Some(Keycode::Num5),
        54 => Some(Keycode::Num6),
        55 => Some(Keycode::Num7),
        56 => Some(Keycode::Num8),
        57 => Some(Keycode::Num9),
        65 => Some(Keycode::A),
        66 => Some(Keycode::B),
        67 => Some(Keycode::C),
        68 => Some(Keycode::D),
        69 => Some(Keycode::E),
        70 => Some(Keycode::F),
        71 => Some(Keycode::G),
        72 => Some(Keycode::H),
        73 => Some(Keycode::I),
        74 => Some(Keycode::J),
        75 => Some(Keycode::K),
        76 => Some(Keycode::L),
        77 => Some(Keycode::M),
        78 => Some(Keycode::N),
        79 => Some(Keycode::O),
        80 => Some(Keycode::P),
        81 => Some(Keycode::Q),
        82 => Some(Keycode::R),
        83 => Some(Keycode::S),
        84 => Some(Keycode::T),
        85 => Some(Keycode::U),
        86 => Some(Keycode::V),
        87 => Some(Keycode::W),
        88 => Some(Keycode::X),
        89 => Some(Keycode::Y),
        90 => Some(Keycode::Z),
        96 => Some(Keycode::Num0),
        97 => Some(Keycode::Num1),
        98 => Some(Keycode::Num2),
        99 => Some(Keycode::Num3),
        100 => Some(Keycode::Num4),
        101 => Some(Keycode::Num5),
        102 => Some(Keycode::Num6),
        103 => Some(Keycode::Num7),
        104 => Some(Keycode::Num8),
        105 => Some(Keycode::Num9),
        108 => Some(Keycode::Return),
        112 => Some(Keycode::F1),
        113 => Some(Keycode::F2),
        114 => Some(Keycode::F3),
        115 => Some(Keycode::F4),
        116 => Some(Keycode::F5),
        117 => Some(Keycode::F6),
        118 => Some(Keycode::F7),
        119 => Some(Keycode::F8),
        120 => Some(Keycode::F9),
        121 => Some(Keycode::F10),
        122 => Some(Keycode::F11),
        123 => Some(Keycode::F12),
        _ => None,
    }
}

#[wasm_bindgen]
pub fn start() {
    main();
}

// FS

use crate::system::{DirEntry, DirEntryType};

pub fn create_dir(path: &PathBuf) -> Result<(), ()> {
    if fs_create_dir(path.to_str().unwrap().to_string()) {
        Ok(())
    } else {
        Err(())
    }
}

pub fn read(path: &PathBuf) -> Option<Vec<u8>> {
    fs_read(path.to_str().unwrap().to_string())
}

pub fn read_dir(path: &PathBuf) -> Option<Vec<DirEntry>> {
    fs_read_dir(path.to_str().unwrap().to_string()).map(|vec| {
        let mut new_vec: Vec<DirEntry> = Vec::new();

        let mut remaining_bytes: usize = 0;
        let mut tmp_str = String::new();
        let mut typ: DirEntryType = DirEntryType::File;
        let mut i: usize = 0;
        while i < vec.len() {
            if remaining_bytes == 0 {
                if tmp_str.len() > 0 {
                    new_vec.push(DirEntry::new(tmp_str, typ));
                }
                tmp_str = String::new();
                typ = if vec[i] & 1 > 0 {
                    DirEntryType::File
                } else {
                    DirEntryType::Folder
                };
                remaining_bytes = (vec[i] >> 1) as usize;
            } else {
                tmp_str.push(vec[i] as char);
                remaining_bytes -= 1;
            }

            i += 1;
        }
        if tmp_str.len() > 0 {
            new_vec.push(DirEntry::new(tmp_str, typ));
        }

        new_vec
    })
}

pub fn remove_dir(path: &PathBuf) -> Result<(), ()> {
    if fs_remove_dir(path.to_str().unwrap().to_string()) {
        Ok(())
    } else {
        Err(())
    }
}

pub fn remove_file(path: &PathBuf) -> Result<(), ()> {
    if fs_remove_file(path.to_str().unwrap().to_string()) {
        Ok(())
    } else {
        Err(())
    }
}

pub fn write(path: &PathBuf, data: &[u8]) -> Result<(), ()> {
    if fs_write(path.to_str().unwrap().to_string(), data) {
        Ok(())
    } else {
        Err(())
    }
}

pub struct WritableFile {
    path: PathBuf,
    buf: Vec<u8>,
}

impl WritableFile {
    fn new(filename: &PathBuf) -> Option<Self> {
        read(filename).map(|buf| Self {
            path: filename.clone(),
            buf,
        })
    }
}

impl Write for WritableFile {
    fn by_ref(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match write(&self.path, &self.buf) {
            Ok(..) => Ok(()),
            Err(..) => Err(Error::new(std::io::ErrorKind::Other, "failed to write")),
        }
    }

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.write(buf)
    }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.buf.write_all(buf)
    }

    fn write_fmt(&mut self, fmt: std::fmt::Arguments<'_>) -> std::io::Result<()> {
        self.buf.write_fmt(fmt)
    }

    fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize> {
        self.buf.write_vectored(bufs)
    }
}

impl Drop for WritableFile {
    fn drop(&mut self) {
        self.flush();
    }
}

pub fn open_file(path: &PathBuf) -> Option<WritableFile> {
    WritableFile::new(path)
}