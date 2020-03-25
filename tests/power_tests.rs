use twofloat::*;
use rand::Rng;

pub mod common;
use common::*;

randomized_test!(recip_test, |rng: F64Rand| {
    let source = get_valid_twofloat(rng, |x, _| { x.abs() > 1e-300 });
    let result = source.recip();

    assert!(
        no_overlap(result.hi(), result.lo()),
        "Reciprocal of {:?} contained overlap",
        source
    );

    let difference = (result.recip() - &source) / &source;
    assert!(
        difference.abs() < 1e-10,
        "{:?}.recip().recip() not close to original value",
        source
    );
});

randomized_test!(sqrt_test, |rng: F64Rand| {
    let source = get_valid_twofloat(rng, |x, _| { x > 0.0 });
    let result = source.sqrt();

    assert!(
        no_overlap(result.hi(), result.lo()),
        "Square root of {:?} gave overlap",
        source
    );
    let difference = (&result * &result - &source).abs() / &source;
    assert!(
        difference < 1e-16,
        "Square root of {:?} ({:?}) squared gives high relative difference {}",
        source,
        result,
        difference.hi()
    );
});

randomized_test!(sqrt_negative_test, |rng: F64Rand| {
    let source = get_valid_twofloat(rng, |x, _| { x < 0.0 });
    let result = source.sqrt();
    assert!(
        !result.is_valid(),
        "Square root of negative number {:?} gave non-error result",
        source
    );
});

randomized_test!(cbrt_test, |rng: F64Rand| {
    let source = get_twofloat(rng);
    let result = source.cbrt();
    assert!(
        no_overlap(result.hi(), result.lo()),
        "Cube root of {:?} gave overlap",
        source
    );
    let difference = (result.powi(3) - &source).abs() / &source;
    assert!(
        difference < 1e-16,
        "Cube root of {:?} ({:?}) squared gives high relative difference {}",
        source,
        result,
        difference.hi()
    );
});

randomized_test!(powi_0_test, |rng: F64Rand| {
    let source = get_valid_twofloat(rng, |x, _| { x != 0.0 });
    let expected = TwoFloat::from(1.0);
    let result = source.powi(0);

    assert!(
        no_overlap(result.hi(), result.lo()),
        "Result of {:?}.powi(0) contained overlap",
        source
    );
    assert_eq!(result, expected, "{:?}.powi(0) did not return 1", source);
});

randomized_test!(powi_1_test, |rng: F64Rand| {
    let source = get_twofloat(rng);
    let result = source.powi(1);

    assert!(
        no_overlap(result.hi(), result.lo()),
        "{:?}.powi(1) contained overlap",
        source
    );
    assert_eq!(
        result, source,
        "{:?}.powi(1) did not return same number",
        source
    );
});

#[test]
fn powi_value_test() {
    let mut rng = rand::thread_rng();
    for _ in 0..TEST_ITERS {
        let source = TwoFloat::new_add(rng.gen_range(-128.0, 128.0), rng.gen_range(-1.0, 1.0));
        let exponent = rng.gen_range(1, 20);
        let mut expected = TwoFloat::from(1.0);
        for _ in 0..exponent {
            expected *= &source;
        }

        let result = source.powi(exponent);
        assert!(
            no_overlap(result.hi(), result.lo()),
            "{:?}.powi({}) contained overlap",
            source,
            exponent
        );

        let difference = (&result - &expected) / &expected;
        assert!(
            difference.abs() < 1e-10,
            "Value mismatch in {:?}.powi({})",
            source,
            exponent
        );
    }
}

#[test]
fn powi_reciprocal_test() {
    let mut rng = rand::thread_rng();
    for _ in 0..TEST_ITERS {
        let source = TwoFloat::new_add(rng.gen_range(-128.0, 128.0), rng.gen_range(-1.0, 1.0));
        let exponent = rng.gen_range(1, 20);
        let expected = 1.0 / source.powi(exponent);
        let result = source.powi(-exponent);

        assert!(
            no_overlap(result.hi(), result.lo()),
            "{:?}.powi({}) contained overlap",
            source,
            -exponent
        );
        assert_eq!(
            result, expected,
            "{0:?}.powi({1}) was not reciprocal of {0:?}.powi({2})",
            source, -exponent, exponent
        );
    }
}

randomized_test!(zero_powf_test, |rng: F64Rand| {
    let source = get_twofloat(rng);
    let result = TwoFloat::from(0.0).powf(&source);

    if source == 0.0 {
        assert!(!result.is_valid(), "0^0 returned valid result");
    } else {
        assert!(
            no_overlap(result.hi(), result.lo()),
            "0^{} returned overlap",
            source
        );
        assert_eq!(result, 0.0, "0^{} did not return 0", source);
    }
});

randomized_test!(powf_zero_test, |rng: F64Rand| {
    let source = get_twofloat(rng);
    let result = source.powf(&TwoFloat::from(0.0));

    if source == 0.0 {
        assert!(!result.is_valid(), "0^0 returned valid result");
    } else {
        assert!(
            no_overlap(result.hi(), result.lo()),
            "{}^0 returned overlap",
            source
        );
        assert_eq!(result, 1.0, "{}^0 did not return 1", source);
    }
});

#[test]
fn powf_test() {
    let mut rng = rand::thread_rng();
    let value_dist = rand::distributions::Uniform::new(1.0f64, 20.0f64);
    for _ in 0..TEST_ITERS {
        let a = rng.sample(value_dist);
        let b = rng.sample(value_dist);

        let expected = a.powf(b);
        let result = TwoFloat::from(a).powf(&TwoFloat::from(b));

        assert!(
            no_overlap(result.hi(), result.lo()),
            "{}^{} resulted in overlap",
            a,
            b
        );

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
}
