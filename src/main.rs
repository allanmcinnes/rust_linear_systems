mod primitives;
use primitives::{signal, fir};

fn main() {
    let mut data = vec![1.0, 2.0, 3.0, 4.0, 5.0].into_iter();
    // Use into_iter instead of iter so that we take ownership of the vector
    let mut xs = signal::Signal::new(&mut data);
    let mut filt1 = fir::Fir::new(vec![0.5, 0.5], &mut xs);
    let mut x2s = signal::Signal::new(&mut filt1);
    let mut filt2 = fir::Fir::new(vec![1.], &mut x2s);
    let ys = signal::Signal::new(&mut filt2);

    for y in ys {
        println!("{}", y);
    }
}
