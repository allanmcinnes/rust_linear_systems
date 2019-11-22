pub type Value = f32;

pub struct Signal<'a> {
    values: &'a mut dyn Iterator<Item = Value>, // Must be mutable to be able to call `next`
}

impl<'a> Signal<'a> {
    pub fn new<I: Iterator<Item = Value>>(iter: &'a mut I) -> Signal<'a> {
        Signal { values: iter }
    }
}

impl<'a> Iterator for Signal<'a> {
    type Item = Value;
    fn next(&mut self) -> Option<Value> {
        self.values.next()
    }
}
