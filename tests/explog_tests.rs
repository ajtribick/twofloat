#![cfg(feature = "math_funcs")]

#[macro_use]
pub mod common;

use common::*;
use rand::Rng;
use twofloat::TwoFloat;

#[test]
fn exp_test() {
    let mut rng = rand::thread_rng();
    let src_dist = rand::distributions::Uniform::new(-600_f64, 600.0);

    repeated_test(|| {
        let a = rng.sample(src_dist);
        let b = TwoFloat::from(a);

        let exp_a = a.exp();
        let exp_b = b.exp();

        assert!(exp_b.is_valid(), "exp({}) produced invalid value", a);

        let difference = ((exp_b - exp_a) / exp_a).abs();

        assert!(
            difference < 1e-15,
            "Mismatch in exp({}): {} vs {}",
            a,
            exp_a,
            exp_b
        );
    });
}

#[test]
fn exp_m1_test() {
    let mut rng = rand::thread_rng();
    let src_dist = rand::distributions::Uniform::<f64>::new(-10.0, 10.0);

    repeated_test(|| {
        let a = rng.sample(src_dist);
        let b = TwoFloat::from(a);

        let exp_a = a.exp_m1();
        let exp_b = b.exp_m1();

        assert!(exp_b.is_valid(), "exp_m1({}) produced invalid value", a);

        let difference = ((exp_b - exp_a) / exp_a).abs();

        assert!(
            difference < 1e-14,
            "Mismatch in exp({}): {:e} vs {}",
            a,
            exp_a,
            exp_b
        );
    });
}

#[test]
fn ln_test() {
    let mut rng = rand::thread_rng();
    let src_dist = rand::distributions::Uniform::new(0f64, 1e300);

    repeated_test(|| {
        let a = rng.sample(src_dist);
        let b = TwoFloat::from(a);

        let ln_a = a.ln();
        let ln_b = b.ln();

        assert!(ln_b.is_valid(), "ln({:e}) produced invalid value", a);

        let difference = ((ln_b - ln_a) / ln_a).abs();

        assert!(
            difference < 1e-16,
            "Mismatch in ln({:e}): {} vs {:?}",
            a,
            ln_a,
            ln_b
        );
    });
}

#[test]
fn ln_negative_test() {
    repeated_test(|| {
        let a = get_valid_twofloat(|x, _| x < 0.0);
        let result = a.ln();
        assert!(!result.is_valid(), "ln({:e}) produced a valid result", a);
    })
}

#[test]
fn ln_1p_test() {
    let mut rng = rand::thread_rng();
    let src_dist = rand::distributions::Uniform::new(-1.0f64, 1e300);

    repeated_test(|| {
        let a = rng.sample(src_dist);
        let b = TwoFloat::from(a);

        let ln_a = a.ln_1p();
        let ln_b = b.ln_1p();

        assert!(ln_b.is_valid(), "ln({:e}) produced invalid value", a);

        let difference = ((ln_b - ln_a) / ln_a).abs();

        assert!(
            difference < 1e-16,
            "Mismatch in ln({:e}): {} vs {:?}",
            a,
            ln_a,
            ln_b
        );
    });
}

#[test]
fn ln_exp_test() {
    let mut rng = rand::thread_rng();
    let src_dist = rand::distributions::Uniform::new(-600.0, 600.0);

    repeated_test(|| {
        let expected = TwoFloat::from(rng.sample(src_dist));

        // Compensate for when the original number is small
        let result = if expected.abs() < 0.25 {
            expected.exp_m1().ln_1p()
        } else {
            expected.exp().ln()
        };

        assert!(
            result.is_valid(),
            "ln(exp({})) produced invalid value",
            expected
        );

        let difference = ((result - expected) / expected).abs();

        assert!(
            difference < 1e-30,
            "Mismatch {}: {:?} vs {:?}",
            difference,
            expected,
            result,
        );
    });
}
