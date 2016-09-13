use std::collections::VecDeque;
use std::collections::vec_deque::Iter;

type Value = f32;

/// A finite-length queue of signal values
struct DelayLine {
    size : usize,
    buffer: VecDeque<Option<Value>>
}

impl DelayLine {
    /// Construct a delay line of `size` values
    fn new(size: usize) -> DelayLine {
        DelayLine { size: size, buffer: VecDeque::new() }
    }

    /// Insert a new value into the delay line
    fn push(&mut self, value: Option<Value>) {
        self.buffer.push_front(value);
        if self.buffer.len() > self.size {
            self.buffer.pop_back();
        }
    }

    /// Returns an iterator over the delay line
    fn iter(&self) -> Iter<Option<Value>> {
        self.buffer.iter()
    }

    /// Returns true if the delay line is empty (either not yet
    /// filled, or contains only None)
    fn is_empty(&self) -> bool {
        // TODO: optimize to ! (any != None) ?
        self.buffer.is_empty() || self.buffer.iter().all(|&i| i == None)
    }
}

/// Contains the coefficients and data structures necessary to realize
/// an FIR filter.
struct Fir<'a> {
    /// Filter coefficients (impulse response)
    h : Vec<Value>,
    /// Filter delay line--stores previous input values for use in calculating the
    /// current output
    delay: DelayLine,
    /// Filter input signal
    input : &'a mut Iterator<Item=Value> // Must be mutable to be able to call `next`
}

impl<'a> Fir<'a> {
    /// Constructs an FIR filter with the specified coefficients (`h`), operating on the input
    /// signal `signal`
    fn new<Signal: Iterator<Item=Value>>(h : Vec<Value>, signal: &'a mut Signal) -> Fir<'a> {
        let size = h.len();
        Fir { h: h, delay: DelayLine::new(size), input: signal }
    }

    /// Computes a filter output for a given state and coefficient set
    fn filter<H, D>(h: H, d: D) -> Value
        where H: Iterator<Item=&'a Value>, D: Iterator<Item=&'a Option<Value>>  {
        let zipped = d.zip(h);
        zipped.fold(0 as Value,
                    |y, (&maybe_x, h)| {
                        let x = match maybe_x { Some(x) => x, None => 0 as Value };
                        y + h * x})
    }
}

/// The filter output is an iterator that can be treated as an input signal to other filters
impl<'a> Iterator for Fir<'a> {
    type Item = Value;
    fn next(&mut self) -> Option<Value> {
        let x = self.input.next();
        self.delay.push(x);
        if self.delay.is_empty() {
            None
        }
        else {
            Some(Fir::filter(self.h.iter(), self.delay.iter()))
        }
    }
}




fn main() {
    let data : Vec<Value> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let mut xs = data.into_iter(); // Use into_iter instead of iter so that we take ownership of the evctor
    let mut x2s = Fir::new(vec![0.5, 0.5], &mut xs);
    let ys = Fir::new(vec![0., 1.], &mut x2s);

    for y in ys {
        println!("{}", y);
    }
}
