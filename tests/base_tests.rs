#![allow(clippy::float_cmp)]

use core::cmp::Ordering;
use twofloat::TwoFloat;

#[macro_use]
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

// Helper type for comparison tests

enum Comparison {
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
}

impl Comparison {
    fn symbol(&self) -> &str {
        match self {
            Self::Less => "<",
            Self::Greater => ">",
            Self::LessEqual => "<=",
            Self::GreaterEqual => ">=",
        }
    }

    fn allows_equal(&self) -> bool {
        matches!(self, Self::LessEqual | Self::GreaterEqual)
    }

    fn apply<T>(&self, lhs: T, rhs: T) -> bool
    where
        T: PartialOrd,
    {
        match self {
            Self::Less => lhs < rhs,
            Self::Greater => lhs > rhs,
            Self::LessEqual => lhs <= rhs,
            Self::GreaterEqual => lhs >= rhs,
        }
    }
}

fn compare(op: Comparison) {
    let a = get_twofloat();

    assert_eq!(
        op.allows_equal(),
        op.apply(a, a),
        "Self-comparison using {} failed",
        op.symbol()
    );

    if let Ok(b) = try_get_twofloat_with_hi(a.hi()) {
        assert_eq!(
            op.apply(a.lo(), b.lo()),
            op.apply(a, b),
            "Comparison using {} failed",
            op.symbol()
        );
    }

    if let Ok(c) = try_get_twofloat_with_lo(a.lo()) {
        assert_eq!(
            op.apply(a.hi(), c.hi()),
            op.apply(a, c),
            "Comparison using {} failed",
            op.symbol()
        );
    }
}

#[test]
fn less_than_test() {
    compare(Comparison::Less);
}

#[test]
fn greater_than_test() {
    compare(Comparison::Greater);
}

#[test]
fn less_than_equal_test() {
    compare(Comparison::LessEqual);
}

#[test]
fn greater_than_equal_test() {
    compare(Comparison::GreaterEqual);
}

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
