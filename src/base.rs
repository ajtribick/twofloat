use core::{cmp::Ordering, num::FpCategory};

use hexf::hexf64;

use crate::TwoFloat;

const DEG_PER_RAD: TwoFloat = TwoFloat {
    hi: hexf64!("0x1.ca5dc1a63c1f8p5"),
    lo: hexf64!("-0x1.1e7ab456405f9p-49"),
};

const RAD_PER_DEG: TwoFloat = TwoFloat {
    hi: hexf64!("0x1.1df46a2529d39p-6"),
    lo: hexf64!("0x1.5c1d8becdd291p-62"),
};

const EXPONENT_MASK: u64 = 0x7ff;
const MANTISSA_MASK: u64 = (1 << 52) - 1;

/// Checks if two `f64` values do not overlap, with the first value being the
/// more significant. This matches definition 1.4 in Joldes et al. (2017).
///
/// # Examples
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
/// ```
pub fn no_overlap(a: f64, b: f64) -> bool {
    match a.classify() {
        FpCategory::Normal => {
            if b == 0.0 {
                return true;
            }
            let bits = a.to_bits();
            let biased_exponent = ((bits >> 52) & EXPONENT_MASK) as i16;
            let offset = if (bits & MANTISSA_MASK) == 0
                && libm::copysign(1.0, a) != libm::copysign(1.0, b)
            {
                1077
            } else {
                1076
            };
            let limit = libm::exp2((biased_exponent - offset) as f64);
            match libm::fabs(b).partial_cmp(&limit) {
                Some(Ordering::Less) => true,
                Some(Ordering::Equal) => (bits & 1) == 0,
                _ => false,
            }
        }
        FpCategory::Subnormal | FpCategory::Zero => b == 0.0,
        _ => false,
    }
}

impl TwoFloat {
    /// Mantissa size of the double-double structure of TwoFloat
    /// aka the number of significant digits in base 2
    pub const MANTISSA_DIGITS: u32 = 106;

    /// Smallest finite `TwoFloat` value.
    pub const MIN: Self = Self {
        hi: f64::MIN,
        lo: hexf64!("-0x1.fffffffffffffp+969"),
    };

    /// Smallest positive normal `TwoFloat` value.
    pub const MIN_POSITIVE: Self = Self {
        hi: f64::MIN_POSITIVE,
        lo: 0.0,
    };

    /// Largest finite `TwoFloat` value.
    pub const MAX: Self = Self {
        hi: f64::MAX,
        lo: hexf64!("0x1.fffffffffffffp+969"),
    };

    /// Represents an error value equivalent to `f64::NAN`.
    pub const NAN: Self = Self {
        hi: f64::NAN,
        lo: f64::NAN,
    };

    /// Represents the difference between 1.0 and the next representable normal value.
    pub const EPSILON: Self = Self {
        hi: f64::MIN_POSITIVE,
        lo: 0.0,
    };

    /// A positive infinite value
    pub const INFINITY: Self = Self {
        hi: f64::INFINITY,
        lo: f64::INFINITY,
    };

    /// A negative infinite value
    pub const NEG_INFINITY: Self = Self {
        hi: f64::NEG_INFINITY,
        lo: f64::NEG_INFINITY,
    };

    /// Creates a new TwoFloat from a constant `f64` value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// const value: TwoFloat = TwoFloat::from_f64(1.0);
    /// assert_eq!(value.hi(), 1.0);
    /// ```
    pub const fn from_f64(value: f64) -> Self {
        TwoFloat { hi: value, lo: 0.0 }
    }

    /// Returns the high word of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let value = TwoFloat::new_add(1.0, -1.0e-200);
    /// assert_eq!(value.hi(), 1.0);
    /// ```
    pub fn hi(&self) -> f64 {
        self.hi
    }

    /// Returns the low word of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let value = TwoFloat::new_add(1.0, -1.0e-200);
    /// assert_eq!(value.lo(), -1.0e-200);
    /// ```
    pub fn lo(&self) -> f64 {
        self.lo
    }

    /// Returns `true` if `self` is a valid value, where both components are
    /// finite (not infinity or `NAN`).
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::new_add(1.0, 1.0e-300).is_valid();
    /// let b = TwoFloat::new_mul(1.0e300, 1.0e300).is_valid();
    ///
    /// assert!(a);
    /// assert!(!b);
    /// ```
    pub fn is_valid(&self) -> bool {
        self.hi.is_finite() && self.lo.is_finite() && no_overlap(self.hi, self.lo)
    }

    /// Returns the minimum of two numbers. If one of the arguments is `NAN`,
    /// the other is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::new_add(35.2, 1e-84);
    /// let b = TwoFloat::new_add(35.2, -1e-93);
    ///
    /// assert_eq!(a.min(b), b);
    /// ```
    pub fn min(self, other: Self) -> Self {
        if !self.is_valid() {
            other
        } else if !other.is_valid() || self <= other {
            self
        } else {
            other
        }
    }

    /// Returns the maximum of two numbers. If one of the arguments is `NAN`,
    /// the other is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::new_add(35.2, 1e-84);
    /// let b = TwoFloat::new_add(35.2, -1e-93);
    ///
    /// assert_eq!(a.max(b), a);
    /// ```
    pub fn max(self, other: Self) -> Self {
        if !self.is_valid() {
            other
        } else if !other.is_valid() || self >= other {
            self
        } else {
            other
        }
    }

    /// Converts degrees to radians.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(90.0);
    /// let b = a.to_radians();
    ///
    /// assert!((b - twofloat::consts::FRAC_PI_2).abs() < 1e-16);
    /// ```
    pub fn to_radians(self) -> Self {
        self * RAD_PER_DEG
    }

    /// Converts radians to degrees.
    ///
    /// # Examples
    ///
    /// ```
    /// let a = twofloat::consts::PI;
    /// let b = a.to_degrees();
    ///
    /// assert!((b - 180.0).abs() < 1e-16);
    /// ```
    pub fn to_degrees(self) -> Self {
        self * DEG_PER_RAD
    }

    /// Takes the reciprocal (inverse) of the number, `1/x`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::new_add(67.2, 5.7e-53);
    /// let b = a.recip();
    /// let difference = b.recip() - a;
    ///
    /// assert!(difference.abs() < 1e-16);
    /// ```
    pub fn recip(self) -> Self {
        1.0 / self
    }

    /// Raises the number to an integer power. Returns a NAN value for 0^0.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(2.0).powi(3);
    /// let b = TwoFloat::from(0.0).powi(0);
    ///
    /// assert!(a - TwoFloat::from(8.0) <= 1e-16);
    /// assert!(!b.is_valid());
    /// ```
    pub fn powi(self, n: i32) -> Self {
        match n {
            0 => {
                if self.hi == 0.0 && self.lo == 0.0 {
                    Self::NAN
                } else {
                    Self::from(1.0)
                }
            }
            1 => self,
            -1 => self.recip(),
            _ => {
                let mut result = Self::from(1.0);
                let mut n_pos = n.abs();
                let mut value = self;
                while n_pos > 0 {
                    if (n_pos & 1) != 0 {
                        result *= &value;
                    }
                    value *= value;
                    n_pos >>= 1;
                }
                if n > 0 {
                    result
                } else {
                    result.recip()
                }
            }
        }
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

impl PartialEq<TwoFloat> for TwoFloat {
    fn eq(&self, other: &TwoFloat) -> bool {
        if self.is_valid() != other.is_valid()
            || self.hi.is_nan()
            || self.lo.is_nan()
            || other.hi.is_nan()
            || self.lo.is_nan()
        {
            false
        } else if self.is_valid() {
            self.hi == other.hi && self.lo == other.lo
        } else {
            // all infinities compare equal
            true
        }
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

impl PartialOrd<TwoFloat> for TwoFloat {
    fn partial_cmp(&self, other: &TwoFloat) -> Option<Ordering> {
        if self.hi.is_nan() || self.lo.is_nan() || other.hi.is_nan() || other.lo.is_nan() {
            return None;
        }

        match (self.is_valid(), other.is_valid()) {
            (true, true) => {
                let hi_cmp = self.hi.partial_cmp(&other.hi);
                if matches!(hi_cmp, Some(Ordering::Equal)) {
                    self.lo.partial_cmp(&other.lo)
                } else {
                    hi_cmp
                }
            }
            (true, false) => Some(Ordering::Less),
            (false, true) => Some(Ordering::Greater),
            (false, false) => Some(Ordering::Equal),
        }
    }
}

#[cfg(test)]
mod tests {
    use hexf::hexf64;

    use super::{no_overlap, TwoFloat};

    const ONE: f64 = 1.0;
    const ONE_NEXT: f64 = hexf64!("0x1.0000000000001p+0");
    const ONE_NEXT_2: f64 = hexf64!("0x1.0000000000002p+0");
    const ONE_PREV: f64 = hexf64!("0x1.fffffffffffffp-1");
    const LOWER_MID_DIFF: f64 = hexf64!("0x1p-54");
    const LOWER_MID_DIFF_NEXT: f64 = hexf64!("0x1.0000000000001p-54");
    const UPPER_MID_DIFF: f64 = hexf64!("0x1p-53");
    const UPPER_MID_DIFF_NEXT: f64 = hexf64!("0x1.0000000000001p-53");
    const OFFSET_1_4: f64 = hexf64!("0x1p-54");
    const OFFSET_3_4: f64 = hexf64!("0x1.8p-53");

    #[test]
    fn no_overlap_test() {
        assert!(!no_overlap(1.0, hexf64!("0x1p-52")));
        assert!(!no_overlap(-1.0, hexf64!("-0x1p-52")));
        assert!(no_overlap(1.0, UPPER_MID_DIFF));
        assert!(!no_overlap(1.0, UPPER_MID_DIFF_NEXT));
        assert!(no_overlap(1.0, -LOWER_MID_DIFF));
        assert!(!no_overlap(1.0, -LOWER_MID_DIFF_NEXT));
        assert!(!no_overlap(1.0, -UPPER_MID_DIFF));
        assert!(!no_overlap(ONE_NEXT, UPPER_MID_DIFF));
        assert!(!no_overlap(ONE_NEXT, -UPPER_MID_DIFF));
        assert!(no_overlap(ONE_NEXT_2, UPPER_MID_DIFF));
        assert!(no_overlap(ONE_NEXT_2, -UPPER_MID_DIFF));
        assert!(no_overlap(-1.0, LOWER_MID_DIFF));
        assert!(!no_overlap(-1.0, LOWER_MID_DIFF_NEXT));
        assert!(!no_overlap(-1.0, UPPER_MID_DIFF));
        assert!(no_overlap(-1.0, -UPPER_MID_DIFF));
        assert!(!no_overlap(-1.0, -UPPER_MID_DIFF_NEXT));
        assert!(!no_overlap(-ONE_NEXT, hexf64!("0x1p-53")));
        assert!(!no_overlap(-ONE_NEXT, hexf64!("-0x1p-53")));
        assert!(no_overlap(-ONE_NEXT_2, hexf64!("-0x1p-53")));
        assert!(no_overlap(-ONE_NEXT_2, hexf64!("0x1p-53")));
        assert!(no_overlap(1.0, hexf64!("0x1p-1023")));
        assert!(no_overlap(1.0, hexf64!("-0x1p-1023")));
        assert!(no_overlap(1.0, 0.0));
        assert!(no_overlap(-1.0, -0.0));

        assert!(!no_overlap(hexf64!("0x1p-970"), hexf64!("0x1p-1022")));
        assert!(no_overlap(hexf64!("0x1p-970"), hexf64!("0x1p-1023")));
        assert!(!no_overlap(hexf64!("0x1p-971"), hexf64!("0x1p-1023")));
        assert!(no_overlap(hexf64!("0x1p-971"), hexf64!("0x1p-1024")));

        assert!(no_overlap(hexf64!("0x1p-1023"), 0.0));
        assert!(!no_overlap(hexf64!("0x1p-1023"), f64::MIN));

        assert!(!no_overlap(f64::INFINITY, 1.0));
        assert!(!no_overlap(f64::NAN, 1.0));

        assert!(!no_overlap(0.0, 1.0));
        assert!(!no_overlap(0.0, f64::MIN));
        assert!(no_overlap(0.0, 0.0));
    }

    #[test]
    fn default_test() {
        let value: TwoFloat = Default::default();
        assert_eq!(value, TwoFloat::from(0));
    }

    #[test]
    fn min_test() {
        assert!(TwoFloat::MIN.is_valid());
    }

    #[test]
    fn max_test() {
        assert!(TwoFloat::MAX.is_valid());
    }

    #[test]
    fn midpoint_eq_test() {
        let values = [
            TwoFloat::new_add(ONE, UPPER_MID_DIFF),
            TwoFloat::new_add(ONE_NEXT, -UPPER_MID_DIFF),
            TwoFloat::new_sub(ONE, -UPPER_MID_DIFF),
            TwoFloat::new_sub(ONE_NEXT, UPPER_MID_DIFF),
            TwoFloat {
                hi: ONE,
                lo: UPPER_MID_DIFF,
            },
        ];

        assert!(values.iter().all(|v| v.is_valid()));
        values
            .iter()
            .for_each(|&a| values.iter().for_each(|&b| assert_eq!(a, b)));
    }

    #[test]
    fn midpoint_eq_test_next() {
        let values = [
            TwoFloat::new_add(ONE_NEXT, UPPER_MID_DIFF),
            TwoFloat::new_add(ONE_NEXT_2, -UPPER_MID_DIFF),
            TwoFloat::new_sub(ONE_NEXT, -UPPER_MID_DIFF),
            TwoFloat::new_sub(ONE_NEXT_2, UPPER_MID_DIFF),
            TwoFloat {
                hi: ONE_NEXT_2,
                lo: -UPPER_MID_DIFF,
            },
        ];

        assert!(values.iter().all(|v| v.is_valid()));
        values
            .iter()
            .for_each(|&a| values.iter().for_each(|&b| assert_eq!(a, b)));
    }

    #[test]
    fn midpoint_eq_test_prev() {
        let values = [
            TwoFloat::new_add(ONE, -LOWER_MID_DIFF),
            TwoFloat::new_add(ONE_PREV, LOWER_MID_DIFF),
            TwoFloat::new_sub(ONE, LOWER_MID_DIFF),
            TwoFloat::new_sub(ONE_PREV, -LOWER_MID_DIFF),
        ];

        assert!(values.iter().all(|v| v.is_valid()));
        values
            .iter()
            .for_each(|&a| values.iter().for_each(|&b| assert_eq!(a, b)));
    }

    #[test]
    fn quarter_eq_test() {
        let values = [
            TwoFloat::new_add(ONE, OFFSET_3_4),
            TwoFloat::new_add(ONE_NEXT, -OFFSET_1_4),
            TwoFloat::new_sub(ONE, -OFFSET_3_4),
            TwoFloat::new_sub(ONE_NEXT, OFFSET_1_4),
            TwoFloat {
                hi: ONE_NEXT,
                lo: -OFFSET_1_4,
            },
        ];

        assert!(values.iter().all(|v| v.is_valid()));
        values
            .iter()
            .for_each(|&a| values.iter().for_each(|&b| assert_eq!(a, b)));
    }

    #[test]
    fn ord_test() {
        let lower_values = [
            TwoFloat::new_add(ONE, OFFSET_1_4),
            TwoFloat::new_add(ONE_NEXT, -OFFSET_3_4),
            TwoFloat::new_sub(ONE, -OFFSET_1_4),
            TwoFloat::new_sub(ONE_NEXT, OFFSET_3_4),
            TwoFloat {
                hi: ONE,
                lo: OFFSET_1_4,
            },
        ];
        assert!(lower_values.iter().all(|v| v.is_valid()));

        let mid_values = [
            TwoFloat::new_add(ONE, UPPER_MID_DIFF),
            TwoFloat::new_add(ONE_NEXT, -UPPER_MID_DIFF),
            TwoFloat::new_sub(ONE, -UPPER_MID_DIFF),
            TwoFloat::new_sub(ONE_NEXT, UPPER_MID_DIFF),
            TwoFloat {
                hi: ONE,
                lo: UPPER_MID_DIFF,
            },
        ];
        assert!(mid_values.iter().all(|v| v.is_valid()));

        let upper_values = [
            TwoFloat::new_add(ONE, OFFSET_3_4),
            TwoFloat::new_add(ONE_NEXT, -OFFSET_1_4),
            TwoFloat::new_sub(ONE, -OFFSET_3_4),
            TwoFloat::new_sub(ONE_NEXT, OFFSET_1_4),
            TwoFloat {
                hi: ONE_NEXT,
                lo: -OFFSET_1_4,
            },
        ];
        assert!(upper_values.iter().all(|v| v.is_valid()));

        lower_values.iter().for_each(|&a| {
            mid_values.iter().for_each(|&b| assert!(a < b));
            upper_values.iter().for_each(|&b| assert!(a < b));
        });

        mid_values.iter().for_each(|&a| {
            lower_values.iter().for_each(|&b| assert!(a > b));
            upper_values.iter().for_each(|&b| assert!(a < b));
        });

        upper_values.iter().for_each(|&a| {
            lower_values.iter().for_each(|&b| assert!(a > b));
            mid_values.iter().for_each(|&b| assert!(a > b));
        });
    }
}
