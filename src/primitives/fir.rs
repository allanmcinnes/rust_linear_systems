//! A Finite Impulse Response (FIR) filter
use super::delay;
use super::signal;

/// FIR filter coefficients and state
pub struct Fir<'a> {
    /// Filter coefficients (impulse response)
    h: Vec<signal::Value>,
    /// Filter delay line--stores previous input values for use in calculating the
    /// current output
    delay: delay::Delay<signal::Value>,
    /// Filter input signal
    input: &'a mut signal::Signal<'a>, // Must be mutable to be able to call `next`
}

impl<'a> Fir<'a> {
    /// Constructs an FIR filter with the specified coefficients (`h`), operating on the input
    /// signal `signal`
    pub fn new(h: Vec<signal::Value>, signal: &'a mut signal::Signal<'a>) -> Fir<'a> {
        let size = h.len();
        Fir {
            h,
            delay: delay::Delay::new(size),
            input: signal,
        }
    }

    /// Return the filter output signal
    pub fn output(&mut self) -> signal::Signal {
        signal::Signal::new(self)
    }

    /// Computes a filter output for a given state and coefficient set
    fn filter<H, D>(h: H, d: D) -> signal::Value
    where
        H: Iterator<Item = &'a signal::Value>,
        D: Iterator<Item = &'a Option<signal::Value>>,
    {
        let zipped = h.zip(d);
        zipped.fold(0 as signal::Value, |y, (h, &maybe_x)| {
            let x = match maybe_x {
                Some(x) => x,
                None => 0 as signal::Value,
            };
            y + (h * x)
        })
    }
}

/// The filter output is an iterator that can be treated as an input signal to other filters
impl<'a> Iterator for Fir<'a> {
    type Item = signal::Value;
    fn next(&mut self) -> Option<signal::Value> {
        let x = self.input.next();
        self.delay.push(x);
        if self.delay.is_empty() {
            None
        } else {
            Some(Fir::filter(self.h.iter(), self.delay.iter()))
        }
    }
}
