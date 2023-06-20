use crate::{Singleton, pub_c_singleton, image::{Image, parse_image}};

pub_c_singleton!(IMG_ARR_LEFT, Image, ||parse_image(5, 5, "000cc0ccccccccc0cccc000cc".to_string()).unwrap());
pub_c_singleton!(IMG_ARR_RIGHT, Image, ||parse_image(5, 5, "cc000cccc0ccccccccc0cc000".to_string()).unwrap());
pub_c_singleton!(LOGO_BG_RED_FG_PURP, Image, ||parse_image(8, 7, "22222222111221221212112211222111121221211212111122222222".to_string()).unwrap());
pub_c_singleton!(LOGO, Image, ||parse_image(8, 7, "00000000777007007070770077000777707007077070777700000000".to_string()).unwrap());
pub_c_singleton!(IMG_TAB_ONE_SELECTED, Image, ||parse_image(8, 16, "fffffff0fffcffffffccfffffffcfffffffcffffffcccffffffffff00000000000000000fffff000fcccff00fffcff00fcccff00fcffff00fcccff00fffff000".to_string()).unwrap());
pub_c_singleton!(IMG_TAB_TWO_SELECTED, Image, ||parse_image(8, 16, "fffff000ffcfff00fccfff00ffcfff00ffcfff00fcccff00fffff0000000000000000000fffffff0ffcccfffffffcfffffcccfffffcfffffffcccffffffffff0".to_string()).unwrap());