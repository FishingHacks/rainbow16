use crate::charmap::get_char;

pub fn print<T: Into<String>, F>(set_pixel: F, text: T, x: u32, y: u32, color: u8)
where F: Fn(u32, u32, u8)->()
 {
    let text: String = text.into();
    let bytes: Vec<char> = text.chars().collect();

    let mut cx = x;
    let mut cy = y;

    let mut i: usize = 0;

    while i < bytes.len() {
        let char = get_char(bytes[i]);
        match bytes[i] {
            '\n' => {
                cx = x;
                cy += 6;
            },
            _ => {
                for oy in 0..5 {
                    for ox in 0..3 {
                        let off = (4-oy) * 3 + ox;
                        if char >> off & 0x1 == 1 {
                            set_pixel(cx+(2-ox), cy+oy, color);
                        }
                    }
                }

                cx += 4;
            },
        };
        i += 1;
    }
}