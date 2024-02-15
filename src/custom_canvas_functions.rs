use crate::charmap::put_char_on_canvas_custom;

pub fn print<T: Into<String>, F>(set_pixel: F, text: T, x: u32, y: u32, color: u8)
where F: Fn(i32, i32, u8)->()
 {
    let text: String = text.into();
    let bytes: Vec<char> = text.chars().collect();

    let mut cx = x;
    let mut cy = y;

    let mut i: usize = 0;

    while i < bytes.len() {
        match bytes[i] {
            '\n' => {
                cx = x;
                cy += 6;
            },
            _ => {
                cx += put_char_on_canvas_custom(bytes[i], x as i32, y as i32, color, &set_pixel);
            },
        };
        i += 1;
    }
}