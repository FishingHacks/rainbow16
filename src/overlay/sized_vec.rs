#![allow(dead_code)]
use std::collections::VecDeque;

pub struct SizedVec<T> {
    vec: VecDeque<T>,
    size: usize,
}

impl<T> SizedVec<T> {
    pub const fn new(max_size: usize) -> Self {
        Self {
            vec: VecDeque::new(),
            size: max_size,
        }
    }

    pub fn push(&mut self, value: T) {
        self.vec.push_back(value);
        if self.vec.len() > self.size {
            self.vec.pop_front();
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.vec.pop_back()
    }

    pub fn capacity(&self) -> usize {
        self.size
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn capacity_remaining(&self) -> usize {
        self.size - self.vec.len()
    }
}