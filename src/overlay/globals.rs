#![allow(dead_code)]

use std::fmt::Debug;

use crate::{pub_c_singleton, Singleton, HEIGHT, WIDTH};

use super::terminal::init;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OverlayType {
    PauseMenu,
    Options,
    CodeEditor,
    Explore,
    None,
}

pub_c_singleton!(OVERLAY, OverlayType, || {
    init();
    OverlayType::None
});

pub_c_singleton!(DISPLAYMEM, Vec<u8>, || {
    let mut vec = <Vec<u8>>::new();
    for _ in 0..WIDTH * HEIGHT {
        vec.push(0);
    }
    vec
});
