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
fn right_bit(f: f64) -> Option<i16> {
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
        _ => Some(exponent - 52),
    }
}

/// Returns the leftmost set bit of a floating point number
fn left_bit(f: f64) -> Option<i16> {
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
pub fn no_overlap(a: f64, b: f64) -> bool {
    (a == 0.0 && b == 0.0)
        || match (right_bit(a), left_bit(b)) {
            (Some(r), Some(l)) => r > l,
            _ => false,
        }
}

impl TwoFloat {
    /// Returns the high word of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let value = TwoFloat::new_add(1.0, -1.0e-200);
    /// assert_eq!(value.hi(), 1.0);
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
    pub fn lo(&self) -> f64 {
        self.lo
    }

    /// Returns `true` if `self` is a valid value, where both components are
    /// finite (not infinity or NaN).
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
    pub fn is_valid(&self) -> bool {
        self.hi.is_finite() && self.lo.is_finite() && no_overlap(self.hi, self.lo)
    }

    /// Returns the minimum of two numbers. If one of the arguments is NAN,
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
    pub fn min(self, other: TwoFloat) -> TwoFloat {
        if !self.is_valid() {
            other.clone()
        } else if !other.is_valid() || self <= other {
            self.clone()
        } else {
            other.clone()
        }
    }

    /// Returns the maximum of two numbers. If one of the arguments is NAN,
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
    pub fn max(self, other: TwoFloat) -> TwoFloat {
        if !self.is_valid() {
            other.clone()
        } else if !other.is_valid() || self >= other {
            self.clone()
        } else {
            other.clone()
        }
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
}
