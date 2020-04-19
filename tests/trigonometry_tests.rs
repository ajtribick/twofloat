use rand::Rng;
use twofloat::TwoFloat;

pub mod common;
use common::*;

#[test]
fn sin_cos_test() {
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

        let (sin, cos) = source.sin_cos();
        assert!(
            sin.is_valid(),
            "sin_cos({:?}).0 returned invalid result",
            source
        );
        assert_eq!(
            sin,
            source.sin(),
            "sin_cos({0:?}).0 not equal to sin({0:?})",
            source
        );

        assert!(
            cos.is_valid(),
            "sin_cos({:?}).1 returned invalid result",
            source
        );
        assert_eq!(
            cos,
            source.cos(),
            "sin_cos({0:?}).1 not equal to cos({0:?})",
            source
        );

        let result = cos * cos + sin * sin;
        let difference = (1.0 - result).abs();
        assert!(
            difference < 1e-10,
            "cos^2 + sin^2 for {:?} returned value different from 1",
            source
        );
    }
}
