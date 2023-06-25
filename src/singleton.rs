#![allow(dead_code)]
use std::fmt::Display;

pub struct Singleton<T> {
    value: Option<T>,
    init: fn() -> T,
}

impl<T> Singleton<T> {
    pub const fn new(init: fn() -> T) -> Self {
        Singleton { value: None, init }
    }

    pub fn get(&mut self) -> &mut T {
        if self.value.is_none() {
            let init = self.init;
            self.value = Some(init());
        }
        let value = self.value.as_mut().unwrap();
        value
    }

    pub fn set(&mut self, value: T) {
        self.value = Some(value);
    }

    pub fn reset(&mut self) {
        let init = self.init;
        self.value = Some(init());
    }
}

impl<T: Display> Display for Singleton<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(val) = &self.value {
            f.write_fmt(format_args!("Singleton with value {}", val))
        } else {
            f.write_str("Uninitialized Singleton")
        }
    }
}

#[macro_export]
macro_rules! get_s_val {
    ($s: expr) => {
        unsafe { $s.get() }
    };
}

#[macro_export]
macro_rules! get_s_val_c {
    ($s: expr) => {
        unsafe { $s.get().clone() }
    };
}

#[macro_export]
macro_rules! c_singleton {
    ($n: ident, $type: ty, $fn: expr) => {
        static mut $n: Singleton<$type> = Singleton::new($fn);
    };
}
#[macro_export]
macro_rules! pub_c_singleton {
    ($n: ident, $type: ty, $fn: expr) => {
        pub static mut $n: Singleton<$type> = Singleton::new($fn);
    };
}

#[macro_export]
macro_rules! set_s_val {
    ($s: expr, $v: expr) => {
        unsafe {
            $s.set($v);
        }
    };
}
