mod primitives;
use primitives::{signal, fir};

fn main() {
    let data : Vec<signal::Value> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let mut xs = data.into_iter(); // Use into_iter instead of iter so that we take ownership of the evctor
    let mut x2s = fir::Fir::new(vec![0.5, 0.5], &mut xs);
    let ys = fir::Fir::new(vec![0., 1.], &mut x2s);

    for y in ys {
        println!("{}", y);
    }
}
