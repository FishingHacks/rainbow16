use super::canvas_functions::*;

static R1: i32 = 100;
static R2: i32 = 90;

pub fn update_bg() {
    unsafe {
        TIMER += 1;
    }

}

static mut TIMER: u64 = 120;

pub fn render_bg() {
    clear(Some(0));
    let mut c: u8 = 1;

    let mut y = -R2;
    while y <= R2 {
        c += 1;
        c %= 16;
        if c == 2 || c == 12 {
            c += 1;
        }
        
        let mut x = -R1;
        while x <= R1 {
            let dist = ((x * x + y * y) as f64).sqrt();
            let z = ((dist / 40.0 - (unsafe { TIMER } as f64 / 60.0)).cos() * 6.0).floor() as i32;
            set_pixel(R1 + x, R2 + y - z, c);

            x += 2;
        }

        y += 3;
    }
}