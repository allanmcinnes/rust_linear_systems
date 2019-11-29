//! A Finite Impulse Response (FIR) filter
use super::delay;
use super::signal;

/// FIR filter coefficients and state
pub struct Fir<'a, V: signal::Value> {
    /// Filter coefficients (impulse response)
    h: Vec<V>,
    /// Filter delay line--stores previous input values for use in calculating the
    /// current output
    delay: delay::Delay<V>,
    /// Filter input signal
    input: &'a mut dyn Iterator<Item = V>,
}

impl<'a, V: signal::Value> Fir<'a, V> {
    /// Constructs an FIR filter with the specified coefficients (`h`), operating on the input
    /// signal `signal`
    pub fn new(h: Vec<V>, input: &'a mut dyn Iterator<Item = V>) -> Fir<'a, V> {
        let size = h.len();
        Fir {
            h,
            delay: delay::Delay::new(size),
            input,
        }
    }

    /// Return the filter output signal
    pub fn output(&mut self) -> &mut dyn Iterator<Item = V> {
        self
    }

    /// Computes a filter output for a given state and coefficient set
    fn filter<H, D>(h: H, d: D) -> V
    where
        H: Iterator<Item = &'a V>,
        D: Iterator<Item = &'a Option<V>>,
    {
        let zipped = h.zip(d);
        zipped.fold(signal::Value::zero(), |y, (&h, &maybe_x)| {
            let x = match maybe_x {
                Some(x) => x,
                None => signal::Value::zero(),
            };
            y + (h * x)
        })
    }
}

/// The filter output is an iterator that can be treated as an input signal to other filters
impl<'a, V: signal::Value> Iterator for Fir<'a, V> {
    type Item = V;
    fn next(&mut self) -> Option<V> {
        let x = self.input.next();
        self.delay.push(x);
        if self.delay.is_empty() {
            None
        } else {
            Some(Fir::filter(self.h.iter(), self.delay.iter()))
        }
    }
}
