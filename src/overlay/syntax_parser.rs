use highlighter::core::language::Language;
use highlighter::core::language::Scope;
use highlighter::core::Error;
use highlighter::highlight;

use crate::charmap::get_char;

use super::canvas_functions::put_char_on_canvas;
use super::canvas_functions::rectfill;
use super::editor::SelectionCoordinate;

pub struct R16Lang;

#[derive(Clone, Copy)]
pub enum SyntaxColor {
    Fg = 12,
    Builtin = 1,
    String = 6,
    Number = 9,
    Operator = 11,
    Comment = 13,
    Const = 10,
    Keyword = 4,
}

pub struct SyntaxToken {
    str: String,
    color: SyntaxColor,
}

pub fn vec_to_regex(vec: Vec<&str>) -> String {
    format!("\\b({})\\b", vec.join("|"))
}

impl Language for R16Lang {
    fn name(&self) -> String {
        "r16".to_string()
    }

    fn init(
        &self,
        x: &mut highlighter::core::LexerContext,
    ) -> Result<(), highlighter::core::Error> {
        // TODO: Fix dis
        x.token(Scope::Comment, "--\\[\\[([^]]*\\]\\])")?;
        x.token(Scope::Comment, "--([^\n]*)")?;
        x.token(
            Scope::StringQuoted,
            "(\"[^\"\\\\\\n]*(?:\\\\.[^\"\\\\]*)*\")",
        )?;
        x.token(Scope::StringQuoted, "\\[\\[([^]]*)\\]\\]")?;
        x.token(Scope::ConstantNumber, "\\b([0-9][0-9.]*)\\b")?;
        x.token(
            Scope::KeywordControl,
            vec_to_regex(vec![
                "if", "ifelse", "else", "end", "do", "then", "while", "for", "break", "goto",
                "and", "or", "not", "local", "return", "continue", "until", "function",
            ]),
        )?;
        x.token(
            Scope::ConstantLanguage,
            vec_to_regex(vec!["true", "false", "nil"]),
        )?;
        x.token(
            Scope::KeywordOperator,
            vec![
                "==", "=", "~=", "\\|", ">>", "<<", "&", "\\+", "-", "\\*", "/", "<=", ">=", "<", ">"
            ]
            .join("|"),
        )?;
        x.token(
            Scope::NameFunction,
            vec_to_regex(vec!["_init", "_update", "_draw"]),
        )?;
        x.token(
            Scope::NameFunction,
            vec_to_regex(vec![
                "sleep", "add", "stop", "Exit", "peek", "poke", "btn", "btnp", "setp", "cls",
                "rectfill", "cursor", "print", "rect", "ellipse", "circle", "line", "camera",
                "pal", "palt", "setpal", "sspr", "spr", "rnd", "time", "cos", "sin", "sqrt",
                "flr", "sfx"
            ]),
        )?;

        Ok(())
    }
}

pub fn highlight_code(code: String) -> Result<Vec<SyntaxToken>, Error> {
    let mut vec: Vec<SyntaxToken> = Vec::new();

    let parsed = highlight(R16Lang, &code)?;

    for t in parsed {
        vec.push(SyntaxToken {
            str: t.value,
            color: match t.scope {
                Scope::StringQuoted => SyntaxColor::String,
                Scope::ConstantNumber => SyntaxColor::Number,
                Scope::KeywordControl => SyntaxColor::Keyword,
                Scope::ConstantLanguage => SyntaxColor::Const,
                Scope::KeywordOperator => SyntaxColor::Operator,
                Scope::Comment => SyntaxColor::Comment,
                Scope::NameFunction => SyntaxColor::Builtin,
                _ => SyntaxColor::Fg,
            },
        })
    }

    Ok(vec)
}

fn is_in_selection(x: i32, y: i32, start: &SelectionCoordinate, end: &SelectionCoordinate) -> bool {
    let mut _start = start;
    let mut _end = end;
    if start.line > end.line {
        let tmp = start;
        _start = end;
        _end = tmp;
    } else if start.line == end.line && end.col < start.col {
        let tmp = start;
        _start = end;
        _end = tmp;
    }

    if x < 0 || y < 0 {
        return false;
    }
    let y = y as u32;
    let x = x as u32;

    if y < _start.line || y > _end.line {
        return false;
    } else if y == _start.line && x < _start.col {
        return false;
    } else if y == _end.line && x >= _end.col {
        return false;
    }

    true
}

pub fn print_highlighted_code(
    code: &Vec<SyntaxToken>,
    ox: i32,
    oy: i32,
    selection_start: &SelectionCoordinate,
    selection_end: &SelectionCoordinate,
) {
    let mut y = oy;
    let mut x = ox;

    let has_selection =
        selection_start.col != selection_end.col || selection_start.line != selection_end.line;

    for t in code {
        let bytes: Vec<char> = t.str.chars().collect();
        for i in 0..bytes.len() {
            if has_selection
                && is_in_selection((x - ox) / 4, (y - oy) / 6, selection_start, selection_end)
            {
                rectfill(x, y, 4, 6, 7);
            }
            match bytes[i] {
                // \n
                '\n' => {
                    y += 6;
                    x = ox;
                }
                // ' '
                ' ' => x += 4,
                _ => {
                    let char = get_char(bytes[i]);
                    put_char_on_canvas(&char, x, y, t.color as u8);
                    x += 4;
                }
            }
        }
    }
}
