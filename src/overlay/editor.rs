use std::path::Path;

use super::{
    canvas_functions::*,
    key_utils::keycode_to_character,
    message::{now, set_message},
    overlay::hide_overlay,
    syntax_parser::{highlight_code, print_highlighted_code, SyntaxToken},
    terminal::add_line_to_stdout
};
use crate::{
    c_singleton,
    gamestate::{get_code, get_path, run_game, set_code as __set_code, gamedata_to_string},
    get_s_val,
    luautils::print_err,
    set_s_val,
    utils::{is_alt_pressed, is_ctrl_pressed, is_shift_pressed},
    system::{copy_to_clipboard, read_clipboard},
    Singleton, system::{MouseButton, Keycode}, fs::write,
};

fn set_code(code: &Vec<String>) {
    __set_code(code);
    highlight();
}

#[derive(Clone, Copy, Debug)]
pub struct SelectionCoordinate {
    pub line: u32,
    pub col: u32,
}

impl SelectionCoordinate {
    fn new() -> SelectionCoordinate {
        SelectionCoordinate { line: 0, col: 0 }
    }

    pub fn reset(&mut self) {
        self.line = 0;
        self.col = 0;
    }
}

fn has_selection() -> bool {
    let start = get_s_val!(START_SEL);
    let end = get_s_val!(END_SEL);

    start.line != end.line || start.col != end.col
}

c_singleton!(CODE, Vec<String>, || Vec::new());
c_singleton!(START_SEL, SelectionCoordinate, SelectionCoordinate::new);
c_singleton!(END_SEL, SelectionCoordinate, SelectionCoordinate::new);

fn reset_selection(left: bool) {
    if !has_selection() {
        return;
    }

    let mut start = get_s_val!(START_SEL);
    let mut end = get_s_val!(END_SEL);
    if start.line > end.line {
        let tmp = start;
        start = end;
        end = tmp;
    } else if start.line == end.line && end.col < start.col {
        let tmp = start;
        start = end;
        end = tmp;
    }

    unsafe {
        if left {
            COL = start.col as usize;
            LINE = start.line as usize;
        } else {
            COL = end.col as usize;
            LINE = end.line as usize;
        }
    }

    start.reset();
    end.reset();
}

static mut LINE: usize = 0;
static mut COL: usize = 0;

pub fn init() {
    set_s_val!(CODE, get_code());
    highlight();
}

pub fn render() {
    unsafe {
        _render();
    }
}

pub fn highlight() {
    match highlight_code(get_s_val!(CODE).join("\n")) {
        Err(e) => {
            println!("Tokenization failed: {e}");
            set_s_val!(HIGHLIGHTED_CODE, None)
        }
        Ok(v) => {
            set_s_val!(HIGHLIGHTED_CODE, Some(v));
        }
    }
}

c_singleton!(HIGHLIGHTED_CODE, Option<Vec<SyntaxToken>>, || None);

static mut SCROLL: i32 = 0;

unsafe fn ensure_inbounds() {
    let code = get_s_val!(CODE);
    if LINE >= code.len() {
        LINE = code.len() - 1;
    }
    if COL > code[LINE].len() {
        COL = code[LINE].len();
    }
}

unsafe fn _render() {
    let code = get_s_val!(CODE);
    ensure_inbounds();
    let off = LINE as i32 * 6;
    if off - 5 + SCROLL + 7 < 7 {
        SCROLL = -off;
    }
    if off + SCROLL + 12 > 173 {
        SCROLL = 173 - off - 13;
    }
    if now() % 1000 < 500 && !has_selection() {
        rectfill(COL as i32 * 4 - 1, LINE as i32 * 6 + SCROLL + 7, 5, 5, 2);
    }
    if let Some(code) = get_s_val!(HIGHLIGHTED_CODE) {
        print_highlighted_code(
            code,
            0,
            SCROLL + 7,
            get_s_val!(START_SEL),
            get_s_val!(END_SEL),
        );
    } else {
        for i in 0..code.len() {
            print(&code[i], Some(0), Some(i as i32 * 6 + SCROLL + 7), None);
        }
    }
    rectfill(0, 173, 200, 7, 2);
    print(
        &format!("line {} col {}", LINE, COL),
        Some(1),
        Some(174),
        None,
    );
}

fn remove_selection() {
    if !has_selection() {
        return;
    }

    let mut start = get_s_val!(START_SEL);
    let mut end = get_s_val!(END_SEL);
    if start.line > end.line {
        let tmp = start;
        start = end;
        end = tmp;
    } else if start.line == end.line && end.col < start.col {
        let tmp = start;
        start = end;
        end = tmp;
    }
    let code = get_s_val!(CODE);

    unsafe {
        COL = start.col as usize;
        LINE = start.line as usize;
        ensure_inbounds();
    }

    for _ in start.line + 1..end.line {
        code.remove((start.line + 1) as usize);
    }

    if start.line == end.line {
        code[start.line as usize].replace_range(start.col as usize..end.col as usize, "");
    } else {
        let str1 = &mut code[start.line as usize];
        if str1.len() > 0 {
            if start.col >= str1.len() as u32 {
                start.col = str1.len() as u32 - 1;
            }
            str1.replace_range(start.col as usize..str1.len(), "");
        } else {
            code.remove(start.line as usize);
        }
        if code.len() > start.line as usize + 1 {
            let str2 = &mut code[start.line as usize + 1];
            if str2.len() > 0 {
                if end.col > str2.len() as u32 {
                    end.col = str2.len() as u32;
                }
                str2.replace_range(0..end.col as usize, "");
            } else {
                code.remove(start.line as usize + 1);
            }
        }
    }

    start.reset();
    end.reset();
}

pub fn handle_key(key: Keycode) {
    unsafe {
        let code = CODE.get();
        if key == Keycode::Escape {
            hide_overlay();
            return;
        }
        if is_ctrl_pressed() && !is_alt_pressed() {
            match key {
                Keycode::R => {
                    set_code(code);
                    if let Some(err) = run_game() {
                        let err = print_err(err);
                        let str = err.as_bytes();
                        let mut vec: Vec<String> = Vec::new();

                        let mut len: usize = 0;

                        for i in 0..str.len() {
                            if i % 50 == 0 {
                                vec.push(String::new());
                                len += 1;
                            }
                            vec[len - 1].push(str[i] as char);
                        }

                        for v in vec {
                            add_line_to_stdout(v);
                        }
                    }
                    hide_overlay();
                }
                Keycode::S => save(),
                Keycode::V => {
                    remove_selection();
                    let len = code[LINE].len();
                    let remainder = &code[LINE].clone()[COL..len];
                    code[LINE].replace_range(COL..len, "");
                    let mut first = true;
                    for l in read_clipboard().split('\n') {
                        if !first {
                            LINE += 1;
                            code.insert(LINE, String::new());
                            COL = 0;
                        }
                        code[LINE].push_str(l);
                        COL += l.len();
                        first = false;
                    }
                    code[LINE].push_str(remainder);
                    set_code(code);
                }
                Keycode::C => {
                    let mut str = String::new();
                    if !has_selection() {
                        str = code[LINE].clone();
                    } else {
                        let mut start = get_s_val!(START_SEL);
                        let mut end = get_s_val!(END_SEL);
                        if start.line > end.line {
                            let tmp = start;
                            start = end;
                            end = tmp;
                        } else if start.line == end.line && end.col < start.col {
                            let tmp = start;
                            start = end;
                            end = tmp;
                        }

                        if start.line == end.line {
                            str += &code[start.line as usize][start.col as usize..end.col as usize];
                        } else {
                            str += &code[start.line as usize][start.col as usize..];
                            str.push('\n');
                            for i in start.line + 1..end.line {
                                str += &code[i as usize];
                                str.push('\n');
                            }
                            str += &code[end.line as usize][..end.col as usize];
                        }
                    }
                    copy_to_clipboard(str.as_str());
                }
                Keycode::A => {
                    let start = get_s_val!(START_SEL);
                    let end = get_s_val!(END_SEL);
                    start.col = 0;
                    start.line = 0;
                    end.col = code[code.len() - 1].len() as u32;
                    end.line = code.len() as u32 - 1;
                }
                _ => {}
            }
            return;
        }
        if let Some(char) = keycode_to_character(Some(key)) {
            if char.len_utf8() > 1 {
                return;
            }
            remove_selection();
            if char == '\n' {
                let len = code[LINE].len();
                let remaining = 
                if COL > len {
                    Vec::new()
                } else {
                    Vec::from_iter(code[LINE].as_mut_vec().splice(COL..len, vec![]))
                };
                LINE += 1;
                COL = 0;
                code.insert(LINE, String::from_utf8(remaining).unwrap_or(String::new()));
            } else {
                code[LINE].as_mut_vec().insert(COL, char as u8);
                COL += 1;
            }
            set_code(code);
            return;
        }

        match key {
            Keycode::Up => {
                if !has_selection() && is_shift_pressed() {
                    let s = get_s_val!(START_SEL);
                    s.col = COL as u32;
                    s.line = LINE as u32;
                }
                if !is_shift_pressed() {
                    reset_selection(true);
                }
                if LINE > 0 {
                    LINE -= 1;
                } else {
                    COL = 0;
                }
                if is_shift_pressed() {
                    get_s_val!(END_SEL).line = LINE as u32;
                    get_s_val!(END_SEL).col = COL as u32;
                }
                ensure_inbounds();
            }
            Keycode::Down => {
                if !has_selection() && is_shift_pressed() {
                    let s = get_s_val!(START_SEL);
                    s.col = COL as u32;
                    s.line = LINE as u32;
                }
                if !is_shift_pressed() {
                    reset_selection(false);
                }
                if code.len() > 0 {
                    if LINE < code.len() - 1 {
                        LINE += 1;
                    } else {
                        COL = code[LINE].len();
                    }
                    if is_shift_pressed() {
                        get_s_val!(END_SEL).line = LINE as u32;
                        get_s_val!(END_SEL).col = COL as u32;
                    }
                    ensure_inbounds();
                }
            }
            Keycode::Left => {
                if !has_selection() && is_shift_pressed() {
                    let s = get_s_val!(START_SEL);
                    s.col = COL as u32;
                    s.line = LINE as u32;
                }
                if !is_shift_pressed() {
                    reset_selection(true);
                }
                if COL > 0 {
                    COL -= 1;
                } else if LINE > 0 {
                    LINE -= 1;
                    COL = code[LINE].len();
                }
                if is_shift_pressed() {
                    let s = get_s_val!(END_SEL);
                    s.col = COL as u32;
                    s.line = LINE as u32;
                }
            }
            Keycode::Right => {
                let max = code[LINE].len();
                if !has_selection() && is_shift_pressed() {
                    let s = get_s_val!(START_SEL);
                    s.col = COL as u32;
                    s.line = LINE as u32;
                }
                if !is_shift_pressed() {
                    reset_selection(false);
                }
                if COL < max {
                    COL += 1;
                } else if LINE < code.len() - 1 {
                    LINE += 1;
                    COL = 0;
                }
                if is_shift_pressed() {
                    let s = get_s_val!(END_SEL);
                    s.col = COL as u32;
                    s.line = LINE as u32;
                }
            }
            Keycode::Backspace => {
                if has_selection() {
                    remove_selection();
                } else {
                    if COL == 0 && LINE > 0 {
                        COL = code[LINE - 1].len();
                        let line = code[LINE].clone();
                        let _line = &mut code[LINE - 1];
                        _line.push_str(&line);
                        code.remove(LINE);
                        LINE -= 1;
                    } else if COL > 0 {
                        let line = &mut code[LINE];
                        if COL <= line.len() {
                            if let Some((l, _)) = line.char_indices().nth(COL - 1) {
                                line.remove(l);
                            }
                        }
                        COL -= 1;
                    }
                }
                set_code(code);
            }
            Keycode::Delete => {
                if has_selection() {
                    remove_selection();
                } else {
                    if COL >= code[LINE].len() && LINE < code.len() - 1 {
                        let line = code[LINE + 1].clone();
                        code[LINE].push_str(&line);
                        code.remove(LINE + 1);
                    } else if COL < code[LINE].len() {
                        let line = &mut code[LINE];
                        if let Some((l, _)) = line.char_indices().nth(COL) {
                            line.remove(l);
                        }
                    }
                }
                set_code(code);
            }
            _ => {}
        }
    }
}

pub fn save() {
    if let Some(path) = get_path() {
        let code = gamedata_to_string();
        if let Err(..) = write(&Path::new(&path).to_path_buf(), code.as_bytes()) {
            set_message("saving failed");
        } else {
            set_message("file saved");
        }
    } else {
        set_message("file is untitled. use save <path>")
    }
}

pub fn handle_scroll(dy: i32) {
    unsafe {
        reset_selection(dy < 0);
        if (dy > 0 && LINE > 0) || (dy < 0 && LINE < get_s_val!(CODE).len() - 1) {
            LINE = (LINE as i32 - dy) as usize;
        }
    }
}

pub fn handle_mousedown(btn: MouseButton, x: u32, y: u32) {
    if y > 7 && y < 173 && btn == MouseButton::Left {
        get_s_val!(START_SEL).reset();
        get_s_val!(END_SEL).reset();
        unsafe {
            COL = x as usize / 4;
            LINE = (y as usize - 7) / 6;
        }
    }
}
