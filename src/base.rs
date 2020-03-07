use std::cmp::Ordering;

/// Represents a two-word floating point type, represented as the sum of two
/// non-overlapping f64 values.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TwoFloat {
    pub(crate) hi: f64,
    pub(crate) lo: f64,
}

impl PartialEq<f64> for TwoFloat {
    fn eq(&self, other: &f64) -> bool {
        self.hi.eq(other) && self.lo == 0.0
    }
}

impl PartialEq<TwoFloat> for f64 {
    fn eq(&self, other: &TwoFloat) -> bool {
        self.eq(&other.hi) && other.lo == 0.0
    }
}

impl PartialOrd<f64> for TwoFloat {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        let hi_cmp = self.hi.partial_cmp(other);
        if hi_cmp == Some(Ordering::Equal) {
            self.lo.partial_cmp(&0.0)
        } else {
            hi_cmp
        }
    }
}

impl PartialOrd<TwoFloat> for f64 {
    fn partial_cmp(&self, other: &TwoFloat) -> Option<Ordering> {
        let hi_cmp = self.partial_cmp(&other.hi);
        if hi_cmp == Some(Ordering::Equal) {
            0.0.partial_cmp(&other.lo)
        } else {
            hi_cmp
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::*;

    randomized_test!(copy_test, |rng: F64Rand| {
        let a = TwoFloat { hi: rng(), lo: rng() };
        let b = a;
        assert_eq!(a.hi, b.hi, "Copy failed for {:?}", a);
        assert_eq!(a.lo, b.lo, "Copy failed for {:?}", a);
    });

    randomized_test!(clone_test, |rng: F64Rand| {
        let a = TwoFloat { hi: rng(), lo: rng() };
        let b = a.clone();
        assert_eq!(a.hi, b.hi, "Clone failed for {:?}", a);
        assert_eq!(a.lo, b.lo, "Clone failed for {:?}", a);
    });

    randomized_test!(equality_test, |rng: F64Rand| {
        let a = TwoFloat { hi: rng(), lo: rng() };
        assert_eq!(a, a);
        assert_eq!(&a, &a);

        let b = TwoFloat { hi: rng(), lo: rng() };
        if a.hi != b.hi || a.lo != b.lo {
            assert_ne!(a, b);
            assert_ne!(&a, &b);
        }
    });

    randomized_test!(equality_f64_test, |rng: F64Rand| {
        let a = rng();
        assert_eq!(TwoFloat { hi: a, lo: 0.0 }, a);
        assert_eq!(a, TwoFloat { hi: a, lo: 0.0 });

        let b = rng();
        if b != 0.0 {
            assert_ne!(TwoFloat { hi: a, lo: b }, a);
            assert_ne!(a, TwoFloat { hi: a, lo: b });
        }
    });

    macro_rules! comparison_test {
        ($val_test: ident, $ref_test: ident, $op: tt, $allow_equal: expr) => {
            randomized_test!($val_test, |rng: F64Rand| {
                let a = TwoFloat { hi: rng(), lo: rng() };
                assert_eq!($allow_equal, a $op a, "Self-comparison using {} failed", stringify!($op));

                let b = TwoFloat { hi: a.hi, lo: rng() };
                assert_eq!(a.lo $op b.lo, a $op b, "Comparison using {} failed", stringify!($op));

                let c = TwoFloat { hi: rng(), lo: a.lo };
                assert_eq!(a.hi $op c.hi, a $op c, "Comparison using {} failed", stringify!($op));
            });

            randomized_test!($ref_test, |rng: F64Rand| {
                let a = TwoFloat { hi: rng(), lo: rng() };
                assert_eq!($allow_equal, &a $op &a, "Self-comparison using {} failed", stringify!($op));

                let b = TwoFloat { hi: a.hi, lo: rng() };
                assert_eq!(a.lo $op b.lo, &a $op &b, "Comparison using {} failed", stringify!($op));

                let c = TwoFloat { hi: rng(), lo: a.lo };
                assert_eq!(a.hi $op c.hi, &a $op &c, "Comparison using {} failed", stringify!($op));
            });
        };
    }

    comparison_test!(less_than_test, less_than_ref_test, <, false);
    comparison_test!(greater_than_test, greater_than_ref_test, >, false);
    comparison_test!(less_equal_test, less_equal_ref_test, <=, true);
    comparison_test!(greater_equal_test, greater_equal_ref_test, >=, true);

    randomized_test!(compare_f64_test, |rng: F64Rand| {
        let a = rng();
        let a_two = TwoFloat { hi: a, lo: 0.0 };
        assert!(a.partial_cmp(&a_two) == Some(Ordering::Equal), "Comparison of f64 <=> TwoFloat failed");
        assert!(a_two.partial_cmp(&a) == Some(Ordering::Equal), "Comparison of TwoFloat <=> f64 failed");
        assert!(a <= a_two);
        assert!(a_two <= a);
        assert!(a >= a_two);
        assert!(a_two >= a);

        let b = rng();
        let ab = TwoFloat { hi: a, lo: b };
        if b < 0.0 {
            assert!(a.partial_cmp(&ab) == Some(Ordering::Greater), "Comparison of f64 <=> TwoFloat failed");
            assert!(a > ab, "Comparison of f64 > TwoFloat failed");
            assert!(a >= ab, "Comparison of f64 >= TwoFloat failed");

            assert!(ab.partial_cmp(&a) == Some(Ordering::Less), "Comparison of TwoFloat <=> f64 failed");
            assert!(ab < a, "Comparison of TwoFloat < f64 failed");
            assert!(ab <= a, "Comparison of TwoFloat <= f64 failed");
        } else if b > 0.0 {
            assert!(a.partial_cmp(&ab) == Some(Ordering::Less), "Comparison of f64 <=> TwoFloat failed");
            assert!(a < ab, "Comparison of f64 < TwoFloat failed");
            assert!(a <= ab, "Comparison of f64 <= TwoFloat failed");

            assert!(ab.partial_cmp(&a) == Some(Ordering::Greater), "Comparison of TwoFloat <=> f64 failed");
            assert!(ab > a, "Comparison of TwoFloat > f64 failed");
            assert!(ab >= a, "Comparison of TwoFloat >= f64 failed");
        }

        let c = rng();
        if c < a {
            assert!(c.partial_cmp(&ab) == Some(Ordering::Less), "Comparison of f64 <=> TwoFloat failed");
            assert!(c < ab, "Comparison of f64 < TwoFloat failed");
            assert!(c <= ab, "Comparison of f64 <= TwoFloat failed");

            assert!(ab.partial_cmp(&c) == Some(Ordering::Greater), "Comparison of TwoFloat <=> f64 failed");
            assert!(ab > c, "Comparison of TwoFloat > f64 failed");
            assert!(ab >= c, "Comparison of TwoFloat >= f64 failed");
        } else if c > a {
            assert!(c.partial_cmp(&ab) == Some(Ordering::Greater), "Comparison of f64 <=> TwoFloat failed");
            assert!(c > ab, "Comparison of f64 > TwoFloat failed");
            assert!(c >= ab, "Comparison of f64 >= TwoFloat failed");

            assert!(ab.partial_cmp(&c) == Some(Ordering::Less), "Comparison of TwoFloat <=> f64 failed");
            assert!(ab < c, "Comparison of TwoFloat < f64 failed");
            assert!(ab <= c, "Comparison of TwoFloat <= f64 failed");
        }
    });
}
