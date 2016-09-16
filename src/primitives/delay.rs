use std::collections::VecDeque;
use std::collections::vec_deque;

use super::signal::Value;

/// A finite-length queue of values
pub struct Delay {
    size: usize,
    buffer: VecDeque<Option<Value>>,
}

pub struct DelayIter<'a> {
    iter: vec_deque::Iter<'a, Option<Value>>
}

impl<'a> Iterator for DelayIter<'a> {
    type Item = &'a Option<Value>;
    fn next(&mut self) -> Option<&'a Option<Value>> { self.iter.next() }
}

impl Delay {
    /// Construct a delay of `size`.
    pub fn new(size: usize) -> Delay {
        Delay {
            size: size,
            buffer: VecDeque::new()
        }
    }

    /// Insert a new value into the delay, and moves
    /// all other values one step further through the delay
    /// line. Values that have been in the delay for
    /// greater than `size` insertions are dropped.
    pub fn push(&mut self, value: Option<Value>) {
        self.buffer.push_front(value);
        if self.buffer.len() > self.size {
            self.buffer.pop_back();
        }
    }

    /// Returns an iterator over the delay.
    pub fn iter(&self) -> DelayIter {
        DelayIter { iter: self.buffer.iter() }
    }

    /// Returns true if the delay line is empty (either not yet
    /// filled, or contains only `None`).
    pub fn is_empty(&self) -> bool {
        // TODO: optimize to ! (any != None) ?
        self.buffer.is_empty() || self.buffer.iter().all(|&i| i == None)
    }
}
