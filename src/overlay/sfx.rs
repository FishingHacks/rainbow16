use std::time::{SystemTime, UNIX_EPOCH};

use sdl2::{keyboard::Keycode, mouse::MouseButton};

use crate::{
    audio::WaveType,
    gamestate::get_audio,
    get_s_val,
    keyboard::mouse_button_down,
    memory::sfx,
    sprites::{
        IMG_ARR_LEFT, IMG_ARR_RIGHT, NOISE_WAVE, NOISE_WAVE_SELECTED, ORGAN_WAVE,
        ORGAN_WAVE_SELECTED, SAWTOOTH_WAVE, SAWTOOTH_WAVE_SELECTED, SINE_WAVE, SINE_WAVE_SELECTED,
        SQUARE_WAVE, SQUARE_WAVE_SELECTED, TILTED_SAWTOOTH_WAVE, TILTED_SAWTOOTH_WAVE_SELECTED,
        TRIANGLE_WAVE, TRIANGLE_WAVE_SELECTED,
    },
    utils::is_shift_pressed,
};

use super::{canvas_functions::*, spr::pad_start};

static mut SELECTED: u8 = 0;

fn wave_to_color(wave: &WaveType) -> u8 {
    match wave {
        WaveType::SquareWave => 3,
        WaveType::TiltedSawtoothWave => 5,
        WaveType::SawtoothWave => 6,
        WaveType::TriangleWave => 7,
        WaveType::OrganWave => 9,
        WaveType::NoiseWave => 13,
        WaveType::SineWave => 10,
    }
}

static mut CURRENT_WAVE: WaveType = WaveType::SquareWave;

pub fn render() {
    let audio = get_audio(unsafe { SELECTED as usize });
    // sfx selector
    print(&"sfx:".to_string(), Some(5), Some(10), None);
    get_s_val!(IMG_ARR_LEFT).put_on_canvas(set_pixel, 21, 10);
    get_s_val!(IMG_ARR_RIGHT).put_on_canvas(set_pixel, 37, 10);
    rectfill(27, 9, 9, 7, 15);
    print(
        &format!("{}", unsafe { pad_start(SELECTED.to_string(), '0', 2) }),
        Some(28),
        Some(10),
        None,
    );

    // speed selector
    print(&"spd:".to_string(), Some(90), Some(10), None);
    get_s_val!(IMG_ARR_LEFT).put_on_canvas(set_pixel, 106, 10);
    get_s_val!(IMG_ARR_RIGHT).put_on_canvas(set_pixel, 126, 10);
    rectfill(112, 9, 13, 7, 15);
    print(
        &format!("{}", pad_start(audio.speed.to_string(), '0', 3)),
        Some(113),
        Some(10),
        None,
    );

    // sound selector
    print(&"sound".to_string(), Some(5), Some(19), None);
    rectfill(4, 25, 192, 120, 15);

    // volume selector
    print(&"volume".to_string(), Some(5), Some(146), None);
    rectfill(4, 152, 192, 16, 15);

    // render items
    for i in 0..audio.items.len() {
        let item = audio.items[i];
        let offset = i * 6 + 1;

        rectfill(
            (4 + offset) as i32,
            (167 - ((item.volume % 7 + 1) * 2)) as i32,
            4,
            2,
            item.volume % 7 + 1,
        );

        if item.volume > 0 {
            rectfill(
                (5 + offset) as i32,
                (145 - item.sound * 2) as i32,
                2,
                item.sound as i32 * 2,
                0,
            );
            rectfill(
                (5 + offset) as i32,
                (143 - item.sound * 2) as i32,
                2,
                2,
                wave_to_color(&item.wave_type),
            );
        }
    }

    // render buttons
    let typ = unsafe { CURRENT_WAVE };
    let waves = vec![
        if typ == WaveType::SquareWave {
            get_s_val!(SQUARE_WAVE_SELECTED)
        } else {
            get_s_val!(SQUARE_WAVE)
        },
        if typ == WaveType::SineWave {
            get_s_val!(SINE_WAVE_SELECTED)
        } else {
            get_s_val!(SINE_WAVE)
        },
        if typ == WaveType::SawtoothWave {
            get_s_val!(SAWTOOTH_WAVE_SELECTED)
        } else {
            get_s_val!(SAWTOOTH_WAVE)
        },
        if typ == WaveType::TiltedSawtoothWave {
            get_s_val!(TILTED_SAWTOOTH_WAVE_SELECTED)
        } else {
            get_s_val!(TILTED_SAWTOOTH_WAVE)
        },
        if typ == WaveType::OrganWave {
            get_s_val!(ORGAN_WAVE_SELECTED)
        } else {
            get_s_val!(ORGAN_WAVE)
        },
        if typ == WaveType::NoiseWave {
            get_s_val!(NOISE_WAVE_SELECTED)
        } else {
            get_s_val!(NOISE_WAVE)
        },
        if typ == WaveType::TriangleWave {
            get_s_val!(TRIANGLE_WAVE_SELECTED)
        } else {
            get_s_val!(TRIANGLE_WAVE)
        },
    ];

    for i in 0..waves.len() {
        waves[i].put_on_canvas(set_pixel, 115 + (i as i32 * 12), 18);
    }

    rectfill(0, 173, 200, 7, 2);
}

pub fn mousedown(button: MouseButton, x: u32, y: u32) {
    let a = get_audio(unsafe { SELECTED as usize });
    if button == MouseButton::Left {
        if x >= 21 && x <= 26 && y >= 10 && y <= 15 && unsafe { SELECTED > 0 } {
            unsafe {
                SELECTED -= 1;
            }
        }
        if x >= 37 && x <= 42 && y >= 10 && y <= 15 && unsafe { SELECTED < 32 } {
            unsafe {
                SELECTED += 1;
            }
        }
        if x >= 106
            && x <= 111
            && y >= 10
            && y <= 15
            && a.speed > (if is_shift_pressed() { 11 } else { 1 })
        {
            if is_shift_pressed() {
                a.speed -= 10;
            } else {
                a.speed -= 1;
            }
        }
        if x >= 126
            && x <= 131
            && y >= 10
            && y <= 15
            && a.speed < (if is_shift_pressed() { 245 } else { 255 })
        {
            if is_shift_pressed() {
                a.speed += 10;
            } else {
                a.speed += 1;
            }
        }
        if x >= 5 && y >= 154 && x <= 195 && y <= 169 {
            let mut val = ((14 - (y - 154).min(14)).max(1) - 1) as u8 / 2;
            if val > 7 {
                val = 7;
            }
            let mut index = ((x - 5) / 6) as usize;
            if index >= a.items.len() {
                index = a.items.len() - 1;
            }
            a.items[index].volume = val;
        }
        if x >= 5 && y >= 25 && x <= 195 && y <= 145 {
            let mut val = ((120 - (y - 25).min(120)).max(1) - 1) as u8 / 2;
            if val > 60 {
                val = 60;
            }
            let mut index = ((x - 5) / 6) as usize;
            if index >= a.items.len() {
                index = a.items.len() - 1;
            }
            if a.items[index].volume < 1 {
                a.items[index].volume = 5;
            }
            a.items[index].sound = val;
            a.items[index].wave_type = unsafe { CURRENT_WAVE };
        }
        if x >= 115 && x <= 195 && y >= 18 && y <= 24 {
            let mut idx = (x - 115) / 12;
            if idx > 6 {
                idx = 6;
            }
            unsafe {
                CURRENT_WAVE = match idx {
                    0 => WaveType::SquareWave,
                    1 => WaveType::SineWave,
                    2 => WaveType::SawtoothWave,
                    3 => WaveType::TiltedSawtoothWave,
                    4 => WaveType::OrganWave,
                    5 => WaveType::NoiseWave,
                    6 => WaveType::TriangleWave,
                    _ => WaveType::SquareWave,
                };
            }
        }
    }
}

pub fn mousemove(x: u32, y: u32) {
    if x >= 5 && y >= 154 && x <= 195 && y <= 169 && mouse_button_down(MouseButton::Left) {
        mousedown(MouseButton::Left, x, y);
    }
    if x >= 5 && y >= 25 && x <= 195 && y <= 145 && mouse_button_down(MouseButton::Left) {
        mousedown(MouseButton::Left, x, y);
    }
}

pub fn keydown(keycode: Keycode) {
    if keycode == Keycode::Space || keycode == Keycode::KpSpace {
        let mem = get_s_val!(sfx);
        if mem.get_at_addr_d(102) > 0 {
            for i in 0..=102 {
                mem.set_at_addr(i, 0);
            }
        } else {
            mem.set_at_addr_u32(
                98,
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u32,
            );
            mem.set_at_addr(102, 1);
            get_audio(unsafe { SELECTED as usize }).write_to_memory(&mem, 0);
        }
    }
}
