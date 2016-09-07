use std::collections::VecDeque;

type Value = f32;

// TODO: move delay line into its own structure

/// Contains the coefficients and data structures necessary to realize
/// an FIR filter.
struct Fir<'a> {
    /// Filter coefficients (impulse response)
    h : Vec<Value>,
    /// Filter order (number of coefficients)
    order : usize,
    /// Filter delay line--stores previous input values for use in calculating the
    /// current output
    delay: VecDeque<Value>,
    /// Filter input signal
    input : &'a mut Iterator<Item=Value> // Must be mutable to be able to call `next`
}

impl<'a> Fir<'a> {
    /// Constructs an FIR filter with the specified coefficients (`h`), operating on the input
    /// signal `signal`
    fn new<Signal: Iterator<Item=Value>>(h : Vec<Value>, signal: &'a mut Signal) -> Fir<'a> {
        let size = h.len();
        Fir { h: h, order: size, delay: VecDeque::new(), input: signal }
    }

    /// Computes a filter output for a given state and coefficient set
    fn filter<H, D>(h: H, d: D) -> Value
        where H: Iterator<Item=&'a Value>, D: Iterator<Item=&'a Value>  {
        let zipped = d.zip(h);
        zipped.fold(0 as Value, |y, (x, h)| y + h * x)
    }
}

/// The filter output is an iterator that can be treated as an input signal to other filters
impl<'a> Iterator for Fir<'a> {
    type Item = Value;
    fn next(&mut self) -> Option<Value> {
        match self.input.next() {
            None if self.delay.iter().filter(|&i| *i != 0.).collect::<Vec<&Value>>().is_empty() => None,
            None => {
                self.delay.push_front(0.);
                let y = Fir::filter(self.h.iter(), self.delay.iter());
                self.delay.pop_back();
                Some(y)
            },
            Some(x) => {
                self.delay.push_front(x);
                let y = Fir::filter(self.h.iter(), self.delay.iter());
                if self.delay.len() >= self.order {
                    self.delay.pop_back();
                }
                Some(y)
            }
        }
    }
}




fn main() {
    let data : Vec<Value> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let mut xs = data.into_iter(); // Use into_iter instead of iter so that we take ownership of the evctor
    let mut x2s = Fir::new(vec![0.5, 0.5], &mut xs);
    let ys = Fir::new(vec![1.], &mut x2s);

    for y in ys {
        println!("{}", y);
    }
}
