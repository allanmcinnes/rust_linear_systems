use std::collections::VecDeque;
use std::collections::vec_deque;

/// A finite-length queue of values
pub struct Delay<V: PartialEq> {
    size: usize,
    buffer: VecDeque<Option<V>>,
}

pub struct DelayIter<'a, V: 'a + PartialEq> {
    iter: vec_deque::Iter<'a, Option<V>>
}

impl<'a, V: 'a + PartialEq> Iterator for DelayIter<'a, V> {
    type Item = &'a Option<V>;
    fn next(&mut self) -> Option<&'a Option<V>> { self.iter.next() }
}

impl<V: PartialEq> Delay<V> {
    /// Construct a delay of `size`.
    pub fn new(size: usize) -> Delay<V> {
        Delay {
            size: size,
            buffer: VecDeque::new()
        }
    }

    /// Insert a new value into the delay, and moves
    /// all other values one step further through the delay
    /// line. Values that have been in the delay for
    /// greater than `size` insertions are dropped.
    pub fn push(&mut self, value: Option<V>) {
        self.buffer.push_front(value);
        if self.buffer.len() > self.size {
            self.buffer.pop_back();
        }
    }

    /// Returns an iterator over the delay.
    pub fn iter(&self) -> DelayIter<V> {
        DelayIter { iter: self.buffer.iter() }
    }

    /// Returns true if the delay line is empty (either not yet
    /// filled, or contains only `None`).
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty() || self.buffer.iter().all(|i| *i == None)
    }
}
