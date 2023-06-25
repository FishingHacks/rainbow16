#![allow(non_upper_case_globals)]
use crate::keyboard::{
    button_is_pressed, is_scrolling_down, is_scrolling_up, u8_to_button, Button,
};

use super::canvas_functions::*;

static mut selected: u32 = 0;

static mut y: f32 = 0.0f32;

pub fn reset() {
    unsafe {
        selected = 0;
        y = 0.0f32;
    }
}

pub fn update(items: &Vec<String>) -> Option<u32> {
    if items.len() < 1 {
        return None;
    }

    if unsafe { selected } >= items.len() as u32 {
        unsafe {
            selected = items.len() as u32 - 1;
        }
    }

    if button_is_pressed(Button::Up) || is_scrolling_up() {
        unsafe {
            if selected > 0 {
                selected -= 1;
            } else {
                selected = items.len() as u32 - 1;
            }
        }
    }
    if button_is_pressed(Button::Down) || is_scrolling_down() {
        unsafe {
            if selected < items.len() as u32 - 1 {
                selected += 1;
            } else {
                selected = 0;
            }
        }
    }

    let mut pressed = false;

    for i in 2..8u8 {
        if button_is_pressed(u8_to_button(i)) {
            pressed = true;
        }
    }

    if pressed {
        Some(unsafe { selected })
    } else {
        None
    }
}

pub fn render(items: &Vec<String>) {
    let selected_item = unsafe { selected };
    let dsty = selected_item as i32 * 8 - 1;
    if dsty != unsafe { y.floor() as i32 } {
        unsafe { y += 1.0 * (dsty as f32 - y).signum() }
    } else {
        unsafe {
            y = dsty as f32;
        }
    }
    rectfill(0, 86, 200, 7, 2);
    for i in 0..items.len() {
        cursor(
            Some((200 - items[i].len() as i32 * 4) / 2),
            Some(i as i32 * 8 + 86 - unsafe { y.floor() as i32 }),
        );
        print(&items[i].to_lowercase(), None, None, None);
    }
}
