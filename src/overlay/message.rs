use super::canvas_functions::*;
use std::time::{SystemTime, UNIX_EPOCH};

static mut MESSAGE: String = String::new();

static mut OPEN: u8 = 0;
static mut LAST_UPDATE: u64 = 0;
static mut LAST_CHANGE: u64 = 0;
static mut SHOULD_CLOSE: bool = false;

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

pub fn set_message(new_message: &str) {
    unsafe {
        MESSAGE = new_message.to_lowercase();
        LAST_CHANGE = now();
        SHOULD_CLOSE = false;
    }
}

pub fn reset_statusbar() {
    unsafe {
        MESSAGE.clear();
        OPEN = 0;
        LAST_CHANGE = 0;
        LAST_UPDATE = 0;
        SHOULD_CLOSE = false;
    }
}

pub fn render() {    
    unsafe {
        if now() - LAST_UPDATE > 100 {
            if MESSAGE.len() > 0 && !SHOULD_CLOSE && OPEN < 7 {
                OPEN += 1
            } else if (MESSAGE.len() < 1 || SHOULD_CLOSE) && OPEN > 0 {
                OPEN -= 1;
            }
        }
        if now() - LAST_CHANGE > 1500 && OPEN > 0 && !SHOULD_CLOSE {
            SHOULD_CLOSE = true;
        }

        if OPEN < 1 {
            return;
        }

        rectfill(0, 180 - OPEN as i32, 200, 7, 2);
        print(&MESSAGE, Some(1), Some(181 - OPEN as i32), None);
    }
}
