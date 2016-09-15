use std::collections::VecDeque;
use std::collections::vec_deque::Iter;

use super::signal;

/// A finite-length queue of signal values
pub struct DelayLine {
    size : usize,
    buffer: VecDeque<Option<signal::Value>>
}

impl DelayLine {
    /// Construct a delay line of `size` values
    pub fn new(size: usize) -> DelayLine {
        DelayLine { size: size, buffer: VecDeque::new() }
    }

    /// Insert a new value into the delay line
    pub fn push(&mut self, value: Option<signal::Value>) {
        self.buffer.push_front(value);
        if self.buffer.len() > self.size {
            self.buffer.pop_back();
        }
    }

    /// Returns an iterator over the delay line
    pub fn iter(&self) -> Iter<Option<signal::Value>> {
        self.buffer.iter()
    }

    /// Returns true if the delay line is empty (either not yet
    /// filled, or contains only None)
    pub fn is_empty(&self) -> bool {
        // TODO: optimize to ! (any != None) ?
        self.buffer.is_empty() || self.buffer.iter().all(|&i| i == None)
    }
}
