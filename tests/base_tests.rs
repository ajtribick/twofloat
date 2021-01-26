#![allow(clippy::float_cmp)]

use core::cmp::Ordering;
use twofloat::TwoFloat;

pub mod common;
use common::*;

#[test]
fn equality_f64_test() {
    let a = random_float();
    let a_twofloat = TwoFloat::from(a);
    assert_eq!(
        a_twofloat, a,
        "LHS equality check failed for f64 value {}",
        a
    );
    assert_eq!(
        a, a_twofloat,
        "RHS equality check failed for f64 value {}",
        a
    );

    if let Ok(b_twofloat) = try_get_twofloat_with_hi(a) {
        assert_ne!(a_twofloat, b_twofloat);
    }

    let c_twofloat = get_valid_twofloat(|x, _| x != a);

    assert_ne!(a_twofloat, c_twofloat);
}

macro_rules! comparison_test {
    ($val_test: ident, $ref_test: ident, $op: tt, $allow_equal: expr) => {
        #[test]
        fn $val_test() {
            repeated_test(|| {
                let a = get_twofloat();

                assert_eq!($allow_equal, a $op a, "Self-comparison using {} failed", stringify!($op));

                if let Ok(b) = try_get_twofloat_with_hi(a.hi()) {
                    assert_eq!(a.lo() $op b.lo(), a $op b, "Comparison using {} failed", stringify!($op));
                }

                if let Ok(c) = try_get_twofloat_with_lo(a.lo()) {
                    assert_eq!(a.hi() $op c.hi(), a $op c, "Comparison using {} failed", stringify!($op));
                }
            });
        }

        #[test]
        fn $ref_test() {
            repeated_test(|| {
                let a = get_twofloat();

                assert_eq!($allow_equal, &a $op &a, "Self-comparison using {} failed", stringify!($op));

                if let Ok(b) = try_get_twofloat_with_hi(a.hi()) {
                    assert_eq!(a.lo() $op b.lo(), &a $op &b, "Comparison using {} failed", stringify!($op));
                }

                if let Ok(c) = try_get_twofloat_with_lo(a.lo()) {
                    assert_eq!(a.hi() $op c.hi(), &a $op &c, "Comparison using {} failed", stringify!($op));
                }
            });
        }
    };
}

comparison_test!(less_than_test, less_than_ref_test, <, false);
comparison_test!(greater_than_test, greater_than_ref_test, >, false);
comparison_test!(less_equal_test, less_equal_ref_test, <=, true);
comparison_test!(greater_equal_test, greater_equal_ref_test, >=, true);

#[test]
fn compare_f64_test() {
    repeated_test(|| {
        let a = random_float();
        let a_two = TwoFloat::from(a);
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

        if let Ok(ab) = try_get_twofloat_with_hi(a) {
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

            let c = random_float();
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
    })
}

#[test]
fn min_test() {
    repeated_test(|| {
        let a = get_twofloat();
        let b = get_twofloat();
        let expected = if a < b { a } else { b };
        let result = a.min(b);

        assert_eq!(
            result, expected,
            "min({:?}, {:?}) produced unexpected result",
            a, b
        );
    })
}

#[test]
fn max_test() {
    repeated_test(|| {
        let a = get_twofloat();
        let b = get_twofloat();
        let expected = if a > b { a } else { b };
        let result = a.max(b);

        assert_eq!(
            result, expected,
            "max({:?}, {:?}) produced unexpected result",
            a, b
        );
    });
}
