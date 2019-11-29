extern crate linear_systems;
use linear_systems::primitives::{fir, signal};

#[test]
fn test_filter_chain() {
    let mut data = vec![1.0, 2.0, 3.0, 4.0, 5.0].into_iter();
    let mut filt1 = fir::Fir::new(vec![0.5, 0.5], &mut data);
    let x2s = filt1.output();
    let mut filt2 = fir::Fir::new(vec![1.], x2s);
    let ys = filt2.output();

    let expected = vec![0.5, 1.5, 2.5, 3.5, 4.5, 2.5];
    for (y, ex) in ys.zip(expected) {
        assert_eq!(y, ex);
    }
}
