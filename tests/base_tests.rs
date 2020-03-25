use std::cmp::Ordering;
use twofloat::*;

pub mod common;
use common::*;

randomized_test!(copy_test, |rng: F64Rand| {
    let a = get_twofloat(rng);
    let b = a;
    assert_eq!(a.data(), b.data(), "Copy failed for {:?}", a);
});

randomized_test!(clone_test, |rng: F64Rand| {
    let a = get_twofloat(rng);
    let b = a.clone();
    assert_eq!(a.data(), b.data(), "Clone failed for {:?}", a);
});

randomized_test!(equality_test, |rng: F64Rand| {
    let a = get_twofloat(rng);
    assert_eq!(a, a, "Self-equality check failed for {:?}", a);
    assert_eq!(&a, &a, "Reference self-equality check failed for {:?}", a);

    let b = TwoFloat::try_new(a.hi(), a.lo()).unwrap();
    assert_eq!(a, b, "Equality check for equivalent value failed for {:?}", a);
    assert_eq!(&a, &b, "Equality check for reference to equivalent value failed for {:?}", a);

    let c = get_valid_twofloat(rng, |x, y| { x != a.hi() || y != a.lo() });

    assert_ne!(a, c);
    assert_ne!(&a, &c);
});

randomized_test!(equality_f64_test, |rng: F64Rand| {
    let a = rng();
    let a_twofloat = TwoFloat::from(a);
    assert_eq!(a_twofloat, a, "LHS equality check failed for f64 value {}", a);
    assert_eq!(a, a_twofloat, "RHS equality check failed for f64 value {}", a);

    if let Ok(b_twofloat) = try_get_twofloat_with_hi(rng, a) {
        assert_ne!(a_twofloat, b_twofloat);
    }

    let c_twofloat = get_valid_twofloat(rng, |x, _| { x != a });

    assert_ne!(a_twofloat, c_twofloat);
});

macro_rules! comparison_test {
    ($val_test: ident, $ref_test: ident, $op: tt, $allow_equal: expr) => {
        randomized_test!($val_test, |rng: F64Rand| {
            let a = get_twofloat(rng);

            assert_eq!($allow_equal, a $op a, "Self-comparison using {} failed", stringify!($op));

            if let Ok(b) = try_get_twofloat_with_hi(rng, a.hi()) {
                assert_eq!(a.lo() $op b.lo(), a $op b, "Comparison using {} failed", stringify!($op));
            }

            if let Ok(c) = try_get_twofloat_with_lo(rng, a.lo()) {
                assert_eq!(a.hi() $op c.hi(), a $op c, "Comparison using {} failed", stringify!($op));
            }
        });

        randomized_test!($ref_test, |rng: F64Rand| {
            let a = get_twofloat(rng);

            assert_eq!($allow_equal, &a $op &a, "Self-comparison using {} failed", stringify!($op));

            if let Ok(b) = try_get_twofloat_with_hi(rng, a.hi()) {
                assert_eq!(a.lo() $op b.lo(), &a $op &b, "Comparison using {} failed", stringify!($op));
            }

            if let Ok(c) = try_get_twofloat_with_lo(rng, a.lo()) {
                assert_eq!(a.hi() $op c.hi(), &a $op &c, "Comparison using {} failed", stringify!($op));
            }
        });
    };
}

comparison_test!(less_than_test, less_than_ref_test, <, false);
comparison_test!(greater_than_test, greater_than_ref_test, >, false);
comparison_test!(less_equal_test, less_equal_ref_test, <=, true);
comparison_test!(greater_equal_test, greater_equal_ref_test, >=, true);

randomized_test!(compare_f64_test, |rng: F64Rand| {
    let a = rng();
    let a_two = TwoFloat::try_new(a, 0.0).unwrap();
    assert!(
        a.partial_cmp(&a_two) == Some(Ordering::Equal),
        "Comparison of f64 <=> TwoFloat failed"
    );
    assert!(
        a_two.partial_cmp(&a) == Some(Ordering::Equal),
        "Comparison of TwoFloat <=> f64 failed"
    );
    assert!(a <= a_two);
    assert!(a_two <= a);
    assert!(a >= a_two);
    assert!(a_two >= a);

    if let Ok(ab) = try_get_twofloat_with_hi(rng, a) {
        let b = ab.lo();
        if b < 0.0 {
            assert!(
                a.partial_cmp(&ab) == Some(Ordering::Greater),
                "Comparison of f64 <=> TwoFloat failed"
            );
            assert!(a > ab, "Comparison of f64 > TwoFloat failed");
            assert!(a >= ab, "Comparison of f64 >= TwoFloat failed");

            assert!(
                ab.partial_cmp(&a) == Some(Ordering::Less),
                "Comparison of TwoFloat <=> f64 failed"
            );
            assert!(ab < a, "Comparison of TwoFloat < f64 failed");
            assert!(ab <= a, "Comparison of TwoFloat <= f64 failed");
        } else if b > 0.0 {
            assert!(
                a.partial_cmp(&ab) == Some(Ordering::Less),
                "Comparison of f64 <=> TwoFloat failed"
            );
            assert!(a < ab, "Comparison of f64 < TwoFloat failed");
            assert!(a <= ab, "Comparison of f64 <= TwoFloat failed");

            assert!(
                ab.partial_cmp(&a) == Some(Ordering::Greater),
                "Comparison of TwoFloat <=> f64 failed"
            );
            assert!(ab > a, "Comparison of TwoFloat > f64 failed");
            assert!(ab >= a, "Comparison of TwoFloat >= f64 failed");
        }

        let c = rng();
        if c < a {
            assert!(
                c.partial_cmp(&ab) == Some(Ordering::Less),
                "Comparison of f64 <=> TwoFloat failed"
            );
            assert!(c < ab, "Comparison of f64 < TwoFloat failed");
            assert!(c <= ab, "Comparison of f64 <= TwoFloat failed");

            assert!(
                ab.partial_cmp(&c) == Some(Ordering::Greater),
                "Comparison of TwoFloat <=> f64 failed"
            );
            assert!(ab > c, "Comparison of TwoFloat > f64 failed");
            assert!(ab >= c, "Comparison of TwoFloat >= f64 failed");
        } else if c > a {
            assert!(
                c.partial_cmp(&ab) == Some(Ordering::Greater),
                "Comparison of f64 <=> TwoFloat failed"
            );
            assert!(c > ab, "Comparison of f64 > TwoFloat failed");
            assert!(c >= ab, "Comparison of f64 >= TwoFloat failed");

            assert!(
                ab.partial_cmp(&c) == Some(Ordering::Less),
                "Comparison of TwoFloat <=> f64 failed"
            );
            assert!(ab < c, "Comparison of TwoFloat < f64 failed");
            assert!(ab <= c, "Comparison of TwoFloat <= f64 failed");
        }
    }
});

randomized_test!(data_test, |rng: F64Rand| {
    let source = get_twofloat(rng);
    let expected = (source.hi(), source.lo());
    let result = source.data();
    assert_eq!(expected, result, "Mismatch between data(), hi() and lo()");
});

randomized_test!(try_new_no_overlap_test, |rng: F64Rand| {
    let (a, b) = get_valid_pair(rng, |x, y| no_overlap(x, y));
    let result = TwoFloat::try_new(a, b);
    assert!(
        result.is_ok(),
        "Creation from non-overlapping pair {}, {} resulted in error",
        a,
        b
    );
    assert_eq!(
        result.unwrap().data(),
        (a, b),
        "Value mismatch in creation of TwoFloat"
    );
});

randomized_test!(try_new_overlap_test, |rng: F64Rand| {
    let (a, b) = get_valid_pair(rng, |x, y| !no_overlap(x, y));
    let result = TwoFloat::try_new(a, b);
    assert!(
        result.is_err(),
        "Creation from overlapping pair {}, {} resulted in value",
        a,
        b
    );
});

randomized_test!(min_test, |rng: F64Rand| {
    let a = get_twofloat(rng);
    let b = get_twofloat(rng);
    let expected = if a < b { a } else { b };
    let result = a.min(&b);

    assert_eq!(
        result, expected,
        "min({:?}, {:?}) produced unexpected result",
        a, b
    );
});

randomized_test!(max_test, |rng: F64Rand| {
    let a = get_twofloat(rng);
    let b = get_twofloat(rng);
    let expected = if a > b { a } else { b };
    let result = a.max(&b);

    assert_eq!(
        result, expected,
        "max({:?}, {:?}) produced unexpected result",
        a, b
    );
});
