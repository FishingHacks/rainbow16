use std::collections::HashMap;

use crate::{c_singleton, get_s_val, Singleton, set_pixel};

c_singleton!(CHARMAP, HashMap<char, u32>, || {
    let mut map = HashMap::new();

    map.insert('0', 0b111101101101111);
    map.insert('1', 0b10110010010111);
    map.insert('2', 0b111001111100111);
    map.insert('3', 0b111001011001111);
    map.insert('4', 0b100110111010010);
    map.insert('5', 0b111100111001111);
    map.insert('6', 0b100100111101111);
    map.insert('7', 0b111001001001001);
    map.insert('8', 0b111101111101111);
    map.insert('9', 0b111101111001001);
    map.insert('a', 0b10101111101101);
    map.insert('b', 0b110101110101110);
    map.insert('c', 0b011100100100011);
    map.insert('d', 0b110101101101110);
    map.insert('e', 0b111100111100111);
    map.insert('f', 0b111100110100100);
    map.insert('g', 0b11100100101111);
    map.insert('h', 0b101101111101101);
    map.insert('i', 0b111010010010111);
    map.insert('j', 0b111001001101111);
    map.insert('k', 0b101101110101101);
    map.insert('l', 0b100100100100111);
    map.insert('m', 0b111111101101101);
    map.insert('n', 0b110101101101101);
    map.insert('o', 0b11101101101110);
    map.insert('p', 0b111101111100100);
    map.insert('q', 0b111101101110011);
    map.insert('r', 0b111101110101101);
    map.insert('s', 0b11100111001110);
    map.insert('t', 0b111010010010010);
    map.insert('u', 0b101101101101111);
    map.insert('v', 0b101101101101010);
    map.insert('w', 0b101101101111111);
    map.insert('x', 0b101101010101101);
    map.insert('y', 0b101101010010010);
    map.insert('z', 0b111001010100111);
    map.insert(' ', 0b0);
    map.insert('-', 0b111000000);
    map.insert('=', 0b111000111000);
    map.insert('*', 0b101010111010101);
    map.insert('+', 0b10111010000);
    map.insert('/', 0b1001010100100);
    map.insert('\\', 0b100100010001001);
    map.insert('´', 0b10100000000000);
    map.insert('`', 0b100010000000000);
    map.insert('^', 0b10101000000000);
    map.insert('[', 0b110100100100110);
    map.insert(']', 0b11001001001011);
    map.insert('(', 0b10100100100010);
    map.insert(')', 0b10001001001010);
    map.insert('{', 0b1010110010001);
    map.insert('}', 0b100010011010100);
    map.insert('<', 0b10100010000);
    map.insert('>', 0b10001010000);
    map.insert(':', 0b10000010000);
    map.insert('.', 0b100);
    map.insert(',', 0b10100);
    map.insert('!', 0b10010010000010);
    map.insert('?', 0b111001011000010);
    map.insert('#', 0b101111101111101);
    map.insert('"', 0b101101000000000);
    map.insert('%', 0b101001010100101);
    map.insert(';', 0b100000100100);
    map.insert('&', 0b110110011101111);
    map.insert('$', 0b111110111011111);
    map.insert('~', 0b1111100000);
    map.insert('ä', 0b101000111101110);
    map.insert('ö', 0b101000111101111);
    map.insert('ü', 0b101000101101111);
    map.insert('ß', 0b110101110111100);
    map.insert('|', 0b10010010010010);
    map.insert('_', 0b111);
    map.insert('\'', 0b010010000000000);

    map
});

static UNKNOWN_CHAR: &u32 = &31599;

pub fn get_char(char: char) -> u32 {
    *get_s_val!(CHARMAP).get(&char).unwrap_or(UNKNOWN_CHAR)
}

pub fn put_char_on_canvas(char: &u32, x: i32, y: i32, color: u8) {
    for oy in 0..5 {
        for ox in 0..3 {
            let off = (4-oy) * 3 + ox;
            if char >> off & 0x1 == 1 {
                set_pixel(x+(2-ox), y+oy, color);
            }
        }
    }
}
