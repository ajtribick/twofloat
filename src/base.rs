use std::cmp::Ordering;
use std::fmt;

/// Represents a two-word floating point type, represented as the sum of two
/// non-overlapping f64 values.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TwoFloat {
    pub(crate) hi: f64,
    pub(crate) lo: f64,
}

/// Returns the rightmost included bit of a floating point number
pub(crate) fn right_bit(f: f64) -> Option<i16> {
    let fbits = f.to_bits();
    let exponent = ((fbits >> 52) & 0x7ff) as i16 - 1023;
    match exponent {
        -1023 => {
            let mantissa = fbits & ((1 << 52) - 1);
            if mantissa == 0 {
                Some(std::i16::MIN)
            } else {
                Some(-1074)
            }
        }
        1024 => None,
        _ => {
            Some(exponent - 52)
        },
    }
}

/// Returns the leftmost set bit of a floating point number
pub(crate) fn left_bit(f: f64) -> Option<i16> {
    let fbits = f.to_bits();
    let exponent = ((fbits >> 52) & 0x7ff) as i16 - 1023;
    match exponent {
        -1023 => {
            let mantissa = fbits & ((1 << 52) - 1);
            if mantissa == 0 {
                Some(std::i16::MIN)
            } else {
                Some(-1011 - mantissa.leading_zeros() as i16)
            }
        }
        1024 => None,
        _ => Some(exponent),
    }
}

/// Checks if two `f64` values do not overlap, with the first value being the
/// more significant.
///
/// # Examples:
///
/// ```
/// # use twofloat::no_overlap;
/// let a = no_overlap(1.0, -1e-200);
/// let b = no_overlap(1e-200, 1.0);
/// let c = no_overlap(1.0, 0.25);
///
/// assert!(a);
/// assert!(!b);
/// assert!(!c);
pub fn no_overlap(a: f64, b: f64) -> bool {
    (a == 0.0 && b == 0.0) || match (right_bit(a), left_bit(b)) {
        (Some(r), Some(l)) => r > l,
        _ => false,
    }
}

impl TwoFloat {
    /// Attempts to construct a new `TwoFloat` from two `f64` values. This can
    /// be used to reconstitute a `TwoFloat` from the values returned by the
    /// `data` method.
    ///
    /// # Errors:
    ///
    /// An error will be returned if the supplied `f64` values overlap.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// #
    /// # fn main() -> Result<(), ()> {
    /// let a = 1.0;
    /// let b = 1.0e-200;
    /// let result1 = TwoFloat::try_new(a, b)?;
    /// let result2 = TwoFloat::try_new(1.0, 2.0);
    ///
    /// assert_eq!(result1.data(), (a, b));
    /// assert!(result2.is_err());
    /// #     Ok(())
    /// # }
    pub fn try_new(a: f64, b: f64) -> Result<TwoFloat, ()> {
        if no_overlap(a, b) { Ok(TwoFloat { hi: a, lo: b }) } else { Err(()) }
    }

    /// Returns the high and low words of `self` as a tuple.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let value = TwoFloat::new_add(1.0, 1.0e-200);
    /// assert_eq!(value.data(), (1.0, 1.0e-200));
    pub fn data(&self) -> (f64, f64) {
        (self.hi, self.lo)
    }
}

impl fmt::Display for TwoFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} ({:+})]", self.hi, self.lo)
    }
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

    #[test]
    fn right_bit_test() {
        assert_eq!(right_bit(std::f64::INFINITY), None);
        assert_eq!(right_bit(std::f64::NEG_INFINITY), None);
        assert_eq!(right_bit(std::f64::NAN), None);
        assert_eq!(right_bit(1.0), Some(-52));
        assert_eq!(right_bit(2.0), Some(-51));
        assert_eq!(right_bit(0.5), Some(-53));
        assert_eq!(right_bit(2.2250738585072014e-308), Some(-1074));
        assert_eq!(right_bit(2.2250738585072009e-308), Some(-1074));
        assert_eq!(right_bit(4.9406564584124654e-324), Some(-1074));
        assert!(right_bit(0.0).unwrap_or(0) < -1074);
    }

    #[test]
    fn left_bit_test() {
        assert_eq!(left_bit(std::f64::INFINITY), None);
        assert_eq!(left_bit(std::f64::NEG_INFINITY), None);
        assert_eq!(left_bit(std::f64::NAN), None);
        assert_eq!(left_bit(1.0), Some(0));
        assert_eq!(left_bit(2.0), Some(1));
        assert_eq!(left_bit(0.5), Some(-1));
        assert_eq!(left_bit(2.2250738585072014e-308), Some(-1022));
        assert_eq!(left_bit(2.2250738585072009e-308), Some(-1023));
        assert_eq!(left_bit(4.9406564584124654e-324), Some(-1074));
        assert!(left_bit(0.0).unwrap_or(0) < -1074);
    }

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

    randomized_test!(data_test, |rng: F64Rand| {
        let a = rng();
        let b = rng();
        let source = TwoFloat { hi: a, lo: b };
        let result = source.data();
        assert_eq!(result, (a, b));
    });

    randomized_test!(try_new_no_overlap_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |x, y| { no_overlap(x, y) });
        let expected = TwoFloat { hi: a, lo: b };
        let result = TwoFloat::try_new(a, b);
        assert!(result.is_ok(), "Creation from non-overlapping pair {}, {} resulted in error", a, b);
        assert_eq!(result.unwrap(), expected, "Value mismatch in creation of TwoFloat");
    });

    randomized_test!(try_new_overlap_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |x, y| { !no_overlap(x, y) });
        let result = TwoFloat::try_new(a, b);
        assert!(result.is_err(), "Creation from overlapping pair {}, {} resulted in value", a, b);
    });
}
