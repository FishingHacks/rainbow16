use sdl2::keyboard::Keycode;
use std::fs::{create_dir_all, read_dir, read_to_string, remove_dir_all, remove_file, write};

use crate::{
    c_singleton,
    gamestate::{gamedata_to_string, get_path, load_game, run_game, set_file_name, load_code},
    get_s_val,
    info::VERSION,
    luautils::print_err,
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
                    if let Some(p) = get_s_val!(CARTSPATH)
                        .join(name.to_owned() + ".r16")
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
            if let Some(p) = get_s_val!(CARTSPATH)
                .join(name.to_owned() + ".r16")
                .to_str()
            {
                let p = p.to_string();
                match read_to_string(p.clone()) {
                    Err(e) => add_line_to_stdout(format!("{e}")),
                    Ok(str) => {
                        if !load_game(str, Some(p)) {
                            add_line_to_stdout(format!("Failed to load {}", name));
                        }
                    },
                };
            }
        }
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
        let path = get_s_val!(CARTSPATH).join(args.join(" ") + ".r16");
        set_file_name(path.to_str().and_then(|f| Some(f.to_string())));
        if let Err(e) = write(path, gamedata_to_string()) {
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
