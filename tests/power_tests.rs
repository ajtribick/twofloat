#![cfg(feature = "math_funcs")]
#![allow(clippy::float_cmp)]

#[macro_use]
pub mod common;

use common::*;
use rand::Rng;
use twofloat::TwoFloat;

#[test]
fn recip_test() {
    repeated_test(|| {
        let source = get_valid_twofloat(|x, _| x.abs() > 1e-300);
        let result = source.recip();

        assert!(
            result.is_valid(),
            "Reciprocal of {:?} produced invalid value",
            source
        );

        let difference = (result.recip() - source) / source;
        assert!(
            difference.abs() < 1e-10,
            "{:?}.recip().recip() not close to original value",
            source
        );
    })
}

#[test]
fn sqrt_test() {
    repeated_test(|| {
        let source = get_valid_twofloat(|x, _| x > 0.0);
        let result = source.sqrt();

        assert!(
            result.is_valid(),
            "Square root of {:?} produced invalid value",
            source
        );
        let difference = (result * result - source).abs() / source;
        assert!(
            difference < 1e-16,
            "Square root of {:?} ({:?}) squared gives high relative difference {}",
            source,
            result,
            difference.hi()
        );
    });
}

#[test]
fn sqrt_negative_test() {
    repeated_test(|| {
        let source = get_valid_twofloat(|x, _| x < 0.0);
        let result = source.sqrt();
        assert!(
            !result.is_valid(),
            "Square root of negative number {:?} gave non-error result",
            source
        );
    });
}

#[test]
fn cbrt_test() {
    repeated_test(|| {
        let source = get_twofloat();
        let result = source.cbrt();
        assert!(
            result.is_valid(),
            "Cube root of {:?} produced invalid value",
            source
        );
        let difference = (result.powi(3) - source).abs() / source;
        assert!(
            difference < 1e-16,
            "Cube root of {:?} ({:?}) squared gives high relative difference {}",
            source,
            result,
            difference.hi()
        );
    });
}

#[test]
fn powi_0_test() {
    repeated_test(|| {
        let source = get_valid_twofloat(|x, _| x != 0.0);
        let expected = TwoFloat::from(1.0);
        let result = source.powi(0);

        assert!(
            result.is_valid(),
            "Result of {:?}.powi(0) produced invalid value",
            source
        );
        assert_eq!(result, expected, "{source:?}.powi(0) did not return 1");
    })
}

#[test]
fn powi_1_test() {
    repeated_test(|| {
        let source = get_twofloat();
        let result = source.powi(1);

        assert!(
            result.is_valid(),
            "{:?}.powi(1) produced invalid value",
            source
        );
        assert_eq!(
            result, source,
            "{source:?}.powi(1) did not return same number"
        );
    });
}

#[test]
fn powi_value_test() {
    let mut rng = rand::thread_rng();
    repeated_test(|| {
        let source = TwoFloat::new_add(rng.gen_range(-128.0..128.0), rng.gen_range(-1.0..1.0));
        let exponent = rng.gen_range(1..20);
        let mut expected = TwoFloat::from(1.0);
        for _ in 0..exponent {
            expected *= &source;
        }

        let result = source.powi(exponent);
        assert!(
            result.is_valid(),
            "{:?}.powi({}) produced invalid value",
            source,
            exponent
        );

        let difference = (result - expected) / expected;
        assert!(
            difference.abs() < 1e-10,
            "Value mismatch in {:?}.powi({})",
            source,
            exponent
        );
    });
}

#[test]
fn powi_reciprocal_test() {
    let mut rng = rand::thread_rng();
    repeated_test(|| {
        let source = TwoFloat::new_add(rng.gen_range(-128.0..128.0), rng.gen_range(-1.0..1.0));
        let exponent = rng.gen_range(1..20);
        let expected = 1.0 / source.powi(exponent);
        let result = source.powi(-exponent);

        assert!(
            result.is_valid(),
            "{:?}.powi({}) produced invalid value",
            source,
            -exponent
        );
        assert_eq!(
            result, expected,
            "{0:?}.powi({1}) was not reciprocal of {0:?}.powi({2})",
            source, -exponent, exponent
        );
    });
}

#[test]
fn zero_powf_test() {
    repeated_test(|| {
        let source = get_twofloat();
        let result = TwoFloat::from(0.0).powf(source);

        if source == 0.0 {
            assert!(!result.is_valid(), "0^0 returned valid result");
        } else {
            assert!(result.is_valid(), "0^{} produced invalid value", source);
            assert_eq!(result, 0.0, "0^{source} did not return 0");
        }
    })
}

#[test]
fn powf_zero_test() {
    repeated_test(|| {
        let source = get_twofloat();
        let result = source.powf(TwoFloat::from(0.0));

        if source == 0.0 {
            assert!(!result.is_valid(), "0^0 returned valid result");
        } else {
            assert!(result.is_valid(), "{}^0 returned invalid value", source);
            assert_eq!(result, 1.0, "{source}^0 did not return 1");
        }
    });
}

#[test]
fn powf_test() {
    let mut rng = rand::thread_rng();
    let value_dist = rand::distributions::Uniform::new(1.0f64, 20.0f64);
    repeated_test(|| {
        let a = rng.sample(value_dist);
        let b = rng.sample(value_dist);

        let expected = a.powf(b);
        let result = TwoFloat::from(a).powf(TwoFloat::from(b));

        assert!(result.is_valid(), "{}^{} resulted in invalid value", a, b);

        let difference = (result - expected).abs().hi() / expected;

        assert!(
            difference < 1e-8,
            "{}^{} resulted in different value {} vs {}",
            a,
            b,
            result,
            expected
        );
    });
}

#[test]
fn powf_integers_test() {
    let mut rng = rand::thread_rng();
    let value_dist = rand::distributions::Uniform::new(-20.0f64, 20.0f64);
    repeated_test(|| {
        let a = rng.sample(value_dist);
        let b = rng.sample(value_dist).floor();

        let expected = a.powf(b);
        let result = TwoFloat::from(a).powf(TwoFloat::from(b));
        if expected.is_nan() {
            assert!(result.hi().is_nan());
            assert!(result.lo().is_nan());
        } else {
            let difference = (result - expected).abs().hi() / expected;

            assert!(
                difference < 1e-8,
                "{}^{} resulted in different value {} vs {}",
                a,
                b,
                result,
                expected
            );
        }
    });
}

#[test]
fn powf_negative_test() {
    let mut rng = rand::thread_rng();
    let value_dist = rand::distributions::Uniform::new(-20.0f64, 20.0f64);
    repeated_test(|| {
        let a = rng.sample(value_dist);
        let b = rng.sample(value_dist);

        let expected = a.powf(b);
        let result = TwoFloat::from(a).powf(TwoFloat::from(b));
        //println!("{}^{} :{} | {}", a,b,expected, Into::<f64>::into(result));
        if expected.is_nan() {
            assert!(result.hi().is_nan());
            assert!(result.lo().is_nan());
        } else {
            let difference = (result - expected).abs().hi() / expected;

            assert!(
                difference < 1e-8,
                "{}^{} resulted in different value {} vs {}",
                a,
                b,
                result,
                expected
            );
        }
    });
}
