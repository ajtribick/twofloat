use rand::Rng;
use core::convert::TryFrom;
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
            if let Ok(result) = TwoFloat::try_from((a, b)) {
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

#[test]
fn sinh_asinh_test() {
    let mut rng = rand::thread_rng();
    let dist = rand::distributions::Uniform::new_inclusive(-20.0, 20.0);
    for _ in 0..TEST_ITERS {
        let source = TwoFloat::from(rng.sample(dist));
        let result = source.sinh().asinh();
        assert!(
            result.is_valid(),
            "Angle {:?} does not produce valid value for sinh/asinh round trip",
            source
        );
        assert!(
            (source - result).abs() < 1e-5,
            "Angle {:?} does not return same value after sinh/asinh round trip ({:?})",
            source,
            result
        );
    }
}

#[test]
fn cosh_acosh_test() {
    let mut rng = rand::thread_rng();
    let dist = rand::distributions::Uniform::new_inclusive(0.0, 20.0);
    for _ in 0..TEST_ITERS {
        let source = TwoFloat::from(rng.sample(dist));
        let result = source.cosh().acosh();
        assert!(
            result.is_valid(),
            "Angle {:?} does not produce valid value for cosh/acosh round trip",
            source
        );
        assert!(
            (source - result).abs() < 1e-5,
            "Angle {:?} does not return same value after cosh/acosh round trip ({:?})",
            source,
            result
        );
    }
}

#[test]
fn tanh_atanh_test() {
    let mut rng = rand::thread_rng();
    let dist = rand::distributions::Uniform::new_inclusive(-10.0, 10.0);
    for _ in 0..TEST_ITERS {
        let source = TwoFloat::from(rng.sample(dist));
        let result = source.tanh().atanh();
        assert!(
            result.is_valid(),
            "Angle {:?} does not produce valid value for tanh/atanh round trip",
            source
        );
        assert!(
            (source - result).abs() < 1e-5,
            "Angle {:?} does not return same value after tanh/atanh round trip ({:?})",
            source,
            result
        );
    }
}
