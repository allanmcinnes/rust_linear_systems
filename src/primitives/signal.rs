

pub type Value = f32;


// TODO: make signal return an iterator instead of implement iterator?
pub struct Signal<'a> {
    values: &'a mut Iterator<Item = Value>, // Must be mutable to be able to call `next`
}

impl<'a> Signal<'a> {
    pub fn new<I: Iterator<Item = Value>>(iter: &'a mut I) -> Signal<'a> {
        Signal { values: iter }
    }
}

/// The filter output is an iterator that can be treated as an input signal to other filters
impl<'a> Iterator for Signal<'a> {
    type Item = Value;
    fn next(&mut self) -> Option<Value> {
        self.values.next()
    }
}
