use crate::system::Keycode;
use std::{
    fs::{create_dir_all, read, read_dir, remove_dir_all, remove_file, write, File},
    io::Write,
};

use crate::{
    c_singleton,
    canvas_functions::PALETTE1,
    custom_canvas_functions::print as c_print,
    gamestate::{
        gamedata_to_string, get_code, get_path, get_preview_image, load_code, load_game, run_game,
        set_file_name,
    },
    get_s_val,
    info::VERSION,
    luautils::print_err,
    screenshot_saver::write as write_png,
    sprites::CARTRIDGE,
    utils::{is_alt_pressed, is_ctrl_pressed},
    Singleton, CARTSPATH,
};

use super::{canvas_functions::*, key_utils::keycode_to_character, overlay::set_overlay};
use std::{
    collections::VecDeque,
    time::{SystemTime, UNIX_EPOCH},
};

static mut STDOUT: VecDeque<String> = <VecDeque<String>>::new();
static mut STDIN: String = String::new();
static mut HISTORY: Vec<String> = Vec::new();

static C_LF: char = '\n';
static C_CR: char = '\r';
static C_HT: char = '\t';

unsafe fn __add_line_to_stdout<T: Into<String>>(line: T) {
    let l = line.into();
    if l.find(|c| c == C_LF || c == C_CR || c == C_HT).is_some() {
        return;
    }
    while STDOUT.len() >= 29 {
        STDOUT.pop_front();
    }
    STDOUT.push_back(l);
}

pub fn add_line_to_stdout<T: Into<String>>(line: T) {
    line.into()
        .replace('\r', "")
        .replace('\t', "")
        .split('\n')
        .for_each(|line| unsafe { __add_line_to_stdout(line) });
}

pub fn add_char_to_stdin(char: char) {
    if char == '\n' {
        unsafe {
            __add_line_to_stdout(STDIN.clone());
            let s = &STDIN.as_str();
            HISTORY.push(s.to_string());
            run_command(&s[1..].trim());
            STDIN.clear();
            STDIN.push_str(">")
        }

        return;
    }
    unsafe {
        STDIN.push(char);
    }
}

c_singleton!(CWD, Vec<String>, || Vec::new());

pub fn run_command(mut cmd: &str) {
    cmd = cmd.trim();
    if cmd.len() < 1 {
        return;
    }
    let mut args: Vec<&str> = Vec::new();
    let mut __args = cmd.split(' ');
    let mut cmd = "";
    while let Some(arg) = __args.next() {
        if cmd.len() < 1 {
            cmd = arg;
        } else {
            args.push(arg);
        }
    }
    if cmd.len() < 1 {
        return;
    }
    match cmd {
        "pwd" => {
            if get_s_val!(CWD).len() < 1 {
                add_line_to_stdout("~");
            } else {
                add_line_to_stdout("~/".to_string() + &get_s_val!(CWD).join("/"));
            }
        }
        "mkdir" => {
            let joined = args.join("");
            if let Err(e) = create_dir_all(get_s_val!(CARTSPATH).join(joined.as_str())) {
                add_line_to_stdout(e.to_string());
            }
        }
        "ls" => match read_dir(get_s_val!(CARTSPATH).join(get_s_val!(CWD).join("/"))) {
            Err(e) => add_line_to_stdout(format!("{e}")),
            Ok(entries) => {
                if get_s_val!(CWD).len() < 1 {
                    add_line_to_stdout("~: ");
                } else {
                    add_line_to_stdout("~/".to_string() + &get_s_val!(CWD).join("/") + ":");
                }
                for e in entries {
                    if let Ok(ent) = e {
                        if match ent.file_type() {
                            Err(..) => false,
                            Ok(f) => f.is_dir(),
                        } {
                            add_line_to_stdout(
                                ent.file_name().to_str().unwrap_or("").to_owned() + "/",
                            );
                        } else {
                            add_line_to_stdout(ent.file_name().to_str().unwrap_or(""));
                        }
                    }
                }
            }
        },
        "rm" => {
            let p = get_s_val!(CARTSPATH).join(args.join(" "));
            let res = if p.is_dir() {
                remove_dir_all(p)
            } else {
                remove_file(p)
            };
            if let Err(e) = res {
                add_line_to_stdout(format!("{e}"));
            }
        }
        "new" => {
            if args.len() < 1 {
                load_code(NEW_STR.to_string(), None);
                set_overlay(super::OverlayType::CodeEditor);
            } else {
                let name = args[0].trim();
                if name.len() > 0 {
                    let name = name.to_owned();
                    if let Some(p) = get_s_val!(CARTSPATH)
                        .join(if name.ends_with(".r16") || name.ends_with(".r16.png") {
                            name
                        } else {
                            name + ".r16"
                        })
                        .to_str()
                    {
                        match write(p, NEW_STR) {
                            Ok(..) => {
                                // TODO: Load
                                load_code(NEW_STR.to_string(), Some(p.to_string()));
                                set_overlay(super::OverlayType::CodeEditor);
                            }
                            Err(e) => {
                                add_line_to_stdout(format!("Error: {e}"));
                            }
                        }
                    }
                }
            }
        }
        "cd" => {
            let joined = args.join(" ");
            let cwd = get_s_val!(CWD);
            if joined.len() < 1 {
                cwd.clear();
            } else {
                let newpath = joined.split("/");
                for path in newpath {
                    if path == ".." {
                        cwd.pop();
                    } else if path == "~" {
                        cwd.clear();
                    } else if path.len() > 0 {
                        get_s_val!(CWD).push(path.to_string());
                    }
                }
            }
        }
        "exit" => std::process::exit(0),
        "clear" => unsafe { STDOUT.clear() },
        "save" => save(args),
        "load" => {
            let name = args[0].trim();
            if name.len() < 1 {
                return;
            }
            let name = name.to_owned();
            if let Some(p) = get_s_val!(CARTSPATH)
                .join(if name.ends_with(".r16") || name.ends_with(".r16.png") {
                    name.clone()
                } else {
                    name.clone() + ".r16"
                })
                .to_str()
            {
                let p = p.to_string();
                match read(p.clone()) {
                    Err(e) => add_line_to_stdout(format!("{e}")),
                    Ok(str) => {
                        if !load_game(str, Some(p)) {
                            add_line_to_stdout(format!("Failed to load {}", name));
                        }
                    }
                };
            }
        }
        "explore" => set_overlay(super::OverlayType::Explore),
        "run" => {
            if let Some(err) = run_game() {
                add_line_to_stdout(print_err(err));
            }
        }
        "version" => {
            add_line_to_stdout(VERSION.to_string());
        }
        "folder" => {
            #[allow(unused_must_use)]
            open::that_in_background(get_s_val!(CARTSPATH));
        }
        "export" => {
            let mut filename = args.join(" ");
            let img = get_preview_image();
            if filename.len() < 1 {
                add_line_to_stdout("you have to give the image a name!");
            } else if img.is_none() {
                add_line_to_stdout("no preview image!");
            } else {
                if !filename.ends_with(".r16.png") {
                    filename.push_str(".r16.png");
                }
                let path = get_s_val!(CARTSPATH).join(filename);
                match File::create(path) {
                    Err(e) => add_line_to_stdout(&format!("{}", e).to_lowercase()),
                    Ok(mut f) => {
                        let mut vec = Vec::with_capacity(250 * 300 * 4);
                        let tmp = get_s_val!(CARTRIDGE).clone();
                        let tmp = tmp.as_bytes_unmut();
                        static mut BYTES: Vec<u8> = Vec::new();

                        for i in tmp {
                            unsafe {
                                BYTES.push(i.clone());
                            }
                        }

                        let lines = get_code();
                        let line1 = if lines.len() > 0 && lines[0].len() > 2 {
                            &lines[0][2..]
                        } else {
                            ""
                        };
                        let line2 = if lines.len() > 1 && lines[1].len() > 2 {
                            &lines[1][2..]
                        } else {
                            ""
                        };

                        c_print(
                            |x: u32, y: u32, c: u8| {
                                if x < 250 && y < 300 {
                                    unsafe {
                                        BYTES[(y * 250 + x) as usize] = c;
                                    }
                                }
                            },
                            line1.to_string() + "\n" + line2,
                            25,
                            240,
                            12,
                        );

                        img.clone().unwrap().put_on_canvas(
                            |x, y, c| unsafe {
                                BYTES[(y * 250 + x) as usize] = c;
                            },
                            24,
                            37,
                        );

                        for y in 0..300 {
                            for x in 0..250 {
                                let col = unsafe {
                                    PALETTE1[BYTES[(299 - y) * 250 + x].clone() as usize]
                                };
                                let (r, g, b) = col.get_values();
                                vec.push(r);
                                vec.push(g);
                                vec.push(b);
                                vec.push(0xff);
                            }
                        }

                        let mut str = gamedata_to_string();
                        let bytes = unsafe { str.as_mut_vec() };
                        let len = bytes.len() as u32;

                        bytes.push((len & 0xff) as u8);
                        bytes.push(((len >> 8) & 0xff) as u8);
                        bytes.push(((len >> 16) & 0xff) as u8);
                        bytes.push(((len >> 24) & 0xff) as u8);

                        match write_png(&mut f, &vec, 250, 300) {
                            Ok(..) => {
                                if let Err(e) = f.write(&bytes) {
                                    add_line_to_stdout(&format!("{}", e).to_lowercase());
                                } else {
                                    add_line_to_stdout("written successfully!");
                                }
                            }
                            Err(e) => add_line_to_stdout(&format!("{:?}", e).to_lowercase()),
                        }
                    }
                }
            }
        }
        _ => add_line_to_stdout(format!("unknown command: {}", cmd)),
    }
}

pub fn save(args: Vec<&str>) {
    if args.len() < 1 {
        if let Some(path) = get_path() {
            if let Err(e) = write(path, gamedata_to_string()) {
                add_line_to_stdout(format!("error: {e}").to_lowercase());
            } else {
                set_overlay(super::OverlayType::CodeEditor);
            }
        } else {
            add_line_to_stdout("file is untitled. use save <path>")
        }
    } else {
        let mut path = get_s_val!(CARTSPATH).clone();
        for p in get_s_val!(CWD) {
            path.push(p);
        }
        path.push(args.join(" ") + ".r16");
        set_file_name(path.to_str().and_then(|f| Some(f.to_string())));
        if let Err(e) =
            File::create(path).and_then(|mut f| f.write(gamedata_to_string().as_bytes()))
        {
            add_line_to_stdout(format!("error: {e}").to_lowercase());
        } else {
            set_overlay(super::OverlayType::CodeEditor);
        }
    }
}

static NEW_STR: &str =
    "function _init()\n\nend\n\nfunction _update()\n\nend\n\nfunction _draw()\n\nend\n";

pub fn render() {
    clear(Some(0));
    let mut y = 0;
    for line in unsafe { STDOUT.iter() } {
        print(&line.to_lowercase(), Some(0), Some(y), None);
        y += 6;
    }
    print(unsafe { &STDIN.to_lowercase() }, Some(0), Some(y), None);
    if let Ok(dur) = SystemTime::now().duration_since(UNIX_EPOCH) {
        if dur.as_millis() % 1000 < 500 {
            rectfill(
                unsafe { STDIN.chars().collect::<Vec<char>>().len() as i32 } * 4,
                y,
                3,
                5,
                2,
            );
        }
    }
}

pub fn update() {}

pub fn init() {
    unsafe {
        STDIN.clear();
        STDIN.push_str(">");
    }
}

pub fn handle_key(key: Keycode) {
    if key == Keycode::Escape {
        set_overlay(super::OverlayType::CodeEditor);
        return;
    }
    if is_ctrl_pressed() && !is_alt_pressed() {
        if match key {
            Keycode::L => {
                unsafe { STDOUT.clear() }
                true
            }
            Keycode::C => {
                unsafe {
                    STDIN.clear();
                    STDIN.push('>');
                }
                true
            }
            _ => false,
        } {
            return;
        }
    }
    if let Some(char) = keycode_to_character(Some(key)) {
        return add_char_to_stdin(char);
    }
    unsafe {
        match key {
            Keycode::Backspace => {
                if STDIN.len() > 1 {
                    STDIN.pop();
                }
            }
            Keycode::Up => {
                if let Some(h) = HISTORY.pop() {
                    STDIN.clear();
                    STDIN.push_str(&h);
                }
            }
            _ => {}
        };
    }
}
