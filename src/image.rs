#[derive(Clone)]
pub struct Image {
    w: u32,
    h: u32,
    colors: Vec<u8>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        let mut new = Self {
            colors: Vec::new(),
            h: height,
            w: width,
        };
        for _ in 0..width * height {
            new.colors.push(0);
        }
        new
    }

    pub fn put_on_canvas<F>(&self, set_pixel: F, x: i32, y: i32)
    where
        F: Fn(i32, i32, u8) -> (),
    {
        for oy in 0..self.h {
            for ox in 0..self.w {
                set_pixel(
                    ox as i32 + x,
                    oy as i32 + y,
                    self.colors[(oy * self.w + ox) as usize],
                );
            }
        }
    }

    pub fn as_bytes(&mut self) -> &mut Vec<u8> {
        &mut self.colors
    }

    pub fn as_bytes_unmut(&self) -> &Vec<u8> {
        &self.colors
    }

    pub fn from_vec(vec: Vec<u8>, width: u32, height: u32) -> Self {
        Self {
            colors: vec,
            h: height,
            w: width,
        }
    }

    pub fn stringify_vec(&self) -> String {
        let mut str = String::with_capacity(self.colors.len());

        for c in self.colors.iter() {
            str.push(stringify_char(c).unwrap_or('0'));
        }

        str
    }
}

pub fn parse_image(width: u32, height: u32, str: String) -> Option<Image> {
    let mut vec: Vec<u8> = Vec::new();
    if str.len() as u32 != width * height {
        return None;
    }

    for y in 0..height {
        for x in 0..width {
            if let Some(num) = parse_char(str.as_bytes()[(y * width + x) as usize] as char) {
                vec.push(num);
            } else {
                return None;
            }
        }
    }

    Some(Image {
        w: width,
        h: height,
        colors: vec,
    })
}

fn parse_char(char: char) -> Option<u8> {
    match char {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        'a' => Some(10),
        'b' => Some(11),
        'c' => Some(12),
        'd' => Some(13),
        'e' => Some(14),
        'f' => Some(15),
        _ => None,
    }
}

fn stringify_char(u8: &u8) -> Option<char> {
    match u8 {
        0 => Some('0'),
        1 => Some('1'),
        2 => Some('2'),
        3 => Some('3'),
        4 => Some('4'),
        5 => Some('5'),
        6 => Some('6'),
        7 => Some('7'),
        8 => Some('8'),
        9 => Some('9'),
        10 => Some('a'),
        11 => Some('b'),
        12 => Some('c'),
        13 => Some('d'),
        14 => Some('e'),
        15 => Some('f'),
        _ => None,
    }
}
