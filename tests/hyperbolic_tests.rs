use rand::Rng;
use twofloat::*;

pub mod common;
use common::*;

#[test]
fn cosh_sinh_test() {
    let mut rng = rand::thread_rng();
    let mut get_f64 = float_generator();
    let dist = rand::distributions::Uniform::new_inclusive(-20.0, 20.0);
    for _ in 0..TEST_ITERS {
        let source = loop {
            let a = rng.sample(dist);
            let b = get_f64();
            if let Ok(result) = TwoFloat::try_new(a, b) {
                break result;
            }
        };

        let sinh = source.sinh();
        assert!(
            sinh.is_valid(),
            "sinh({:?}) returned invalid result",
            source
        );

        let cosh = source.cosh();
        assert!(
            cosh.is_valid(),
            "cosh({:?}) returned invalid result",
            source
        );

        let result = cosh * cosh - sinh * sinh;
        let difference = (1.0 - result).abs();
        assert!(
            difference < 1e-10,
            "cosh^2 - sinh^2 for {:?} returned value different from 1",
            source
        );
    }
}
