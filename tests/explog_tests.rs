use rand::Rng;
use twofloat::*;

pub mod common;
use common::*;

const EXP_UPPER_LIMIT: f64 = 709.782712893384;

#[test]
fn exp_test() {
    let mut rng = rand::thread_rng();
    let src_dist = rand::distributions::Uniform::new(-600.0, EXP_UPPER_LIMIT);

    for _ in 0..TEST_ITERS {
        let a = rng.sample(src_dist);
        let b = TwoFloat::from(a);

        let exp_a = a.exp();
        let exp_b = b.exp();

        assert!(exp_b.is_valid(), "exp({}) produced invalid value", a);

        let difference = ((exp_b - exp_a) / exp_a).abs();

        assert!(
            difference < 1e-10,
            "Mismatch in exp({}): {} vs {:?}",
            a,
            exp_a,
            exp_b
        );
    }
}

#[test]
fn exp_m1_test() {
    let mut rng = rand::thread_rng();
    let src_dist = rand::distributions::Uniform::<f64>::new(-10.0, 10.0);

    for _ in 0..TEST_ITERS {
        let a = rng.sample(src_dist);
        let b = TwoFloat::from(a);

        let exp_a = a.exp_m1();
        let exp_b = b.exp_m1();

        assert!(exp_b.is_valid(), "exp_m1({}) produced invalid value", a);

        let difference = exp_b - exp_a;

        assert!(
            difference < 1e-10,
            "Mismatch in exp({}): {} vs {:?}",
            a,
            exp_a,
            exp_b
        );
    }
}

#[test]
fn ln_test() {
    let mut rng = rand::thread_rng();
    let src_dist = rand::distributions::Uniform::new(f64::from_bits(1u64), f64::MAX);

    for _ in 0..TEST_ITERS {
        let a = rng.sample(src_dist);
        let b = TwoFloat::from(a);

        let ln_a = a.ln();
        let ln_b = b.ln();

        assert!(ln_b.is_valid(), "ln({}) produced invalid value", a);

        let difference = (ln_b - ln_a).abs();

        assert!(
            difference < 1e-10,
            "Mismatch in ln({}): {} vs {:?}",
            a,
            ln_a,
            ln_b
        );
    }
}

randomized_test!(ln_negative_test, |rng: F64Rand| {
    let a = get_valid_twofloat(rng, |x, _| x < 0.0);
    let result = a.ln();
    assert!(!result.is_valid(), "ln({:?}) produced a valid result", a);
});
