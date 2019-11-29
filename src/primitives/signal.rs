//! Signals are made up of Values that must meet certain trait bounds in order to
//! be processed
use std::ops::{Add, Mul};

// See https://www.worthe-it.co.za/programming/2017/01/15/aliasing-traits-in-rust.html
pub trait Value: PartialEq + Add<Output = Self> + Mul<Output = Self> + From<i32> + Copy
where
    Self: std::marker::Sized,
{
    fn zero() -> Self;
}

impl<V> Value for V
where
    V: PartialEq + Add<Output = Self> + Mul<Output = Self> + From<i32> + Copy,
{
    fn zero() -> V {
        0.into()
    }
}
