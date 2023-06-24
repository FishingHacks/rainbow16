use crate::{Singleton, pub_c_singleton, image::{Image, parse_image}};

pub_c_singleton!(IMG_ARR_LEFT, Image, ||parse_image(5, 5, "000cc0ccccccccc0cccc000cc".to_string()).unwrap());
pub_c_singleton!(IMG_ARR_RIGHT, Image, ||parse_image(5, 5, "cc000cccc0ccccccccc0cc000".to_string()).unwrap());
pub_c_singleton!(LOGO_BG_RED_FG_PURP, Image, ||parse_image(8, 7, "22222222111221221212112211222111121221211212111122222222".to_string()).unwrap());
pub_c_singleton!(LOGO, Image, ||parse_image(8, 7, "00000000777007007070770077000777707007077070777700000000".to_string()).unwrap());
pub_c_singleton!(IMG_TAB_ONE_SELECTED, Image, ||parse_image(8, 16, "fffffff0fffcffffffccfffffffcfffffffcffffffcccffffffffff00000000000000000fffff000fcccff00fffcff00fcccff00fcffff00fcccff00fffff000".to_string()).unwrap());
pub_c_singleton!(IMG_TAB_TWO_SELECTED, Image, ||parse_image(8, 16, "fffff000ffcfff00fccfff00ffcfff00ffcfff00fcccff00fffff0000000000000000000fffffff0ffcccfffffffcfffffcccfffffcfffffffcccffffffffff0".to_string()).unwrap());

pub_c_singleton!(SQUARE_WAVE, Image, ||parse_image(8, 6, "ffffffffffcccccfffcfffcfffcfffcfcccfffccffffffff".to_string()).unwrap());
pub_c_singleton!(NOISE_WAVE, Image, ||parse_image(8, 6, "ffffffffffcfffcfcfcfcfcfccccccccfcfcfcfcfcfffffc".to_string()).unwrap());
pub_c_singleton!(ORGAN_WAVE, Image, ||parse_image(8, 6, "ffffffffffcffffffcfcffffcfffcfcfcfffccfcffffffff".to_string()).unwrap());
pub_c_singleton!(SAWTOOTH_WAVE, Image, ||parse_image(8, 6, "ffffffffffffffccffffccfcffccfffcccfffffcffffffff".to_string()).unwrap());
pub_c_singleton!(SINE_WAVE, Image, ||parse_image(8, 6, "ffffffffffcfffcffcfcfcfcfcfcfcfccfffcfffffffffff".to_string()).unwrap());
pub_c_singleton!(TRIANGLE_WAVE, Image, ||parse_image(8, 6, "fffffffffffccfffffcffcfffcffffcfcffffffcffffffff".to_string()).unwrap());
pub_c_singleton!(TILTED_SAWTOOTH_WAVE, Image, ||parse_image(8, 6, "fffffffffffffcfffffccfcffccfffcfcffffffcffffffff".to_string()).unwrap());

pub_c_singleton!(SQUARE_WAVE_SELECTED, Image, ||parse_image(8, 6, "3333333333ccccc333c333c333c333c3ccc333cc33333333".to_string()).unwrap());
pub_c_singleton!(NOISE_WAVE_SELECTED, Image, ||parse_image(8, 6, "ddddddddddcdddcdcdcdcdcdccccccccdcdcdcdcdcdddddc".to_string()).unwrap());
pub_c_singleton!(ORGAN_WAVE_SELECTED, Image, ||parse_image(8, 6, "9999999999c999999c9c9999c999c9c9c999cc9c99999999".to_string()).unwrap());
pub_c_singleton!(SAWTOOTH_WAVE_SELECTED, Image, ||parse_image(8, 6, "66666666666666cc6666cc6c66cc666ccc66666c66666666".to_string()).unwrap());
pub_c_singleton!(SINE_WAVE_SELECTED, Image, ||parse_image(8, 6, "aaaaaaaaaacaaacaacacacacacacacaccaaacaaaaaaaaaaa".to_string()).unwrap());
pub_c_singleton!(TRIANGLE_WAVE_SELECTED, Image, ||parse_image(8, 6, "77777777777cc77777c77c777c7777c7c777777c77777777".to_string()).unwrap());
pub_c_singleton!(TILTED_SAWTOOTH_WAVE_SELECTED, Image, ||parse_image(8, 6, "5555555555555c55555cc5c55cc555c5c555555c55555555".to_string()).unwrap());