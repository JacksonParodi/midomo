use std::collections::VecDeque;

pub struct RingBuffer<T> {
    buffer: VecDeque<T>,
    size: usize,
}

impl<T> RingBuffer<T> {
    pub fn new(size: usize) -> Self {
        RingBuffer {
            buffer: VecDeque::with_capacity(size),
            size,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.buffer.len() == self.size {
            self.buffer.pop_front();
        }
        self.buffer.push_back(value);
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.buffer.get(index)
    }

    pub fn get_back(&self) -> Option<&T> {
        self.buffer.back()
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }
}
