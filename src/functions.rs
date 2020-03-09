use crate::base::*;
use crate::arithmetic::*;

impl TwoFloat {
    /// Returns the absolute value root of `self`.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// assert_eq!(TwoFloat::new_add(1.0, 1.0e-300).abs(), TwoFloat::new_add(1.0, 1.0e-300));
    /// assert_eq!(TwoFloat::new_add(-1.0, 1.0e-300).abs(), TwoFloat::new_add(1.0, -1.0e-300));
    pub fn abs(&self) -> TwoFloat {
        if self.hi > 0.0 || (self.hi == 0.0 && self.hi.is_sign_positive() && self.lo.is_sign_positive()) { self.clone() } else { -self }
    }

    /// Returns `true` if `self` has a positive sign, including `+0.0`.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// assert!(TwoFloat::new_add(0.0, 0.0).is_sign_positive());
    /// assert!(TwoFloat::new_add(1.0, 1.0e-300).is_sign_positive());
    /// assert!(!TwoFloat::new_add(-1.0, 1.0e-300).is_sign_positive());
    pub fn is_sign_positive(&self) -> bool {
        self.hi > 0.0 || (self.hi == 0.0 && self.hi.is_sign_positive())
    }

    /// Returns `true` if `self` has a negative sign, including `-0.0`.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// assert!(TwoFloat::new_add(-1.0, 1.0e-300).is_sign_negative());
    /// assert!(!TwoFloat::new_add(0.0, 0.0).is_sign_negative());
    /// assert!(!TwoFloat::new_add(1.0, 1.0e-300).is_sign_negative());
    pub fn is_sign_negative(&self) -> bool {
        self.hi < 0.0 || (self.hi == 0.0 && self.hi.is_sign_negative())
    }

    /// Returns `true` if `self` is a valid value, where both components are
    /// finite (not infinity or NaN).
    ///
    /// # Examples:
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// assert!(TwoFloat::new_add(1.0, 1.0e-300).is_valid());
    /// assert!(!TwoFloat::new_mul(1.0e300, 1.0e300).is_valid());
    pub fn is_valid(&self) -> bool {
        self.hi.is_finite() && self.lo.is_finite()
    }

    /// Returns the fractional part of the number.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::new_add(1.0, 1e-200);
    /// let b = TwoFloat::new_add(-1.0, 1e-200);
    /// assert_eq!(a.fract(), TwoFloat::from(1e-200));
    /// assert_eq!(b.fract(), b);
    pub fn fract(&self) -> TwoFloat {
        let hi_fract = self.hi.fract();
        let lo_fract = self.lo.fract();
        let (a, b) = if lo_fract == 0.0 {
            (hi_fract, 0f64)
        } else if hi_fract == 0.0 {
            match (self.hi >= 0.0, self.lo >= 0.0) {
                (true, false) => fast_two_sum(1.0, lo_fract),
                (false, true) => fast_two_sum(-1.0, lo_fract),
                _ => (self.lo.fract(), 0f64)
            }
        } else {
            fast_two_sum(self.hi.fract(), self.lo)
        };

        TwoFloat { hi: a, lo: b }
    }

    /// Returns the integer part of the number.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::new_add(1.0, 1e-200);
    /// let b = TwoFloat::new_add(1.0, -1e-200);
    /// assert_eq!(a.trunc(), TwoFloat::from(1.0));
    /// assert_eq!(b.trunc(), TwoFloat::from(0.0));
    pub fn trunc(&self) -> TwoFloat {
        let (a, b) = if self.lo.fract() == 0.0 {
            (self.hi.trunc(), self.lo.trunc())
        } else if self.hi.fract() == 0.0 {
            match (self.hi >= 0.0, self.lo >= 0.0) {
                (true, false) => fast_two_sum(self.hi, self.lo.trunc() - 1.0),
                (false, true) => fast_two_sum(self.hi, self.lo.trunc() + 1.0),
                _ => (self.hi, self.lo.trunc())
            }
        } else {
            (self.hi.trunc(), 0f64)
        };

        TwoFloat { hi: a, lo: b }
    }

    /// Returns the square root of the number, using equation 4 from Karp &
    /// Markstein (1997).
    ///
    /// # Examples:
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(2.0f64);
    /// let b = a.sqrt();
    /// assert!(b * b - a < 1e-16);
    pub fn sqrt(&self) -> TwoFloat {
        if self.hi < 0.0 || (self.hi == 0.0 && self.lo < 0.0) {
            TwoFloat { hi: std::f64::NAN, lo: std::f64::NAN }
        } else if self.hi == 0.0 && self.lo == 0.0 {
            TwoFloat { hi: 0f64, lo: 0f64 }
        } else {
            let x = 1.0 / self.hi.sqrt();
            let y = self.hi * x;
            TwoFloat::new_add(y, (self - TwoFloat::new_mul(y, y)).hi * (x * 0.5))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::*;

    #[test]
    fn abs_test() {
        assert_eq!(TwoFloat { hi: 0.0, lo: 0.0 }.abs(), TwoFloat {hi: 0.0, lo: 0.0});
        assert!(TwoFloat { hi: 0.0, lo: -0.0 }.abs().lo.is_sign_positive());
        assert!(TwoFloat { hi: -0.0, lo: 0.0 }.abs().lo.is_sign_negative());
    }

    #[test]
    fn is_sign_positive_test() {
        assert!(TwoFloat { hi: 0.0, lo: -0.0 }.is_sign_positive());
        assert!(!TwoFloat { hi: -0.0, lo: 0.0 }.is_sign_positive());
        assert!(!TwoFloat { hi: -0.0, lo: -0.0 }.is_sign_positive());
        assert!(TwoFloat { hi: 1.0, lo: -1e-300 }.is_sign_positive());
        assert!(!TwoFloat { hi: -1.0, lo: -1e-300 }.is_sign_positive());
    }

    #[test]
    fn is_sign_negative_test() {
        assert!(!TwoFloat { hi: 0.0, lo: -0.0 }.is_sign_negative());
        assert!(TwoFloat { hi: -0.0, lo: 0.0 }.is_sign_negative());
        assert!(TwoFloat { hi: -0.0, lo: -0.0 }.is_sign_negative());
        assert!(!TwoFloat { hi: 1.0, lo: -1e-300 }.is_sign_negative());
        assert!(TwoFloat { hi: -1.0, lo: -1e-300 }.is_sign_negative());
    }

    randomized_test!(fract_hi_fract_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { a.fract() != 0.0 && no_overlap(a, b) });
        let source = TwoFloat { hi: a, lo: b };
        let expected = a.fract() + b.fract();
        let result = source.fract();
        assert!(no_overlap(result.hi, result.lo), "Overlap in fract({:?})", source);
        assert!(result.hi.trunc() == 0.0
            || (result.hi.trunc().abs() == 1.0 && ((result.hi >= 0.0) != (result.lo >= 0.0))),
            "Fractional part of {:?} is greater than one", source);
        assert!(result.lo.trunc() == 0.0, "Non-zero integer part of low word of fract({:?}", source);
        assert_eq_ulp!(result.hi, expected, 1, "Mismatch in fractional part of {:?}", source);
    });

    randomized_test!(fract_lo_fract_test, |rng: F64Rand| {
        let (a_fract, b) = get_valid_pair(rng, |a: f64, b: f64| { b.fract() != 0.0 && no_overlap(a.trunc(), b) });
        let a = a_fract.trunc();
        let source = TwoFloat { hi: a, lo: b };
        let expected = match (a >= 0.0, b >= 0.0) {
            (true, false) => 1.0 + b.fract(),
            (false, true) => -1.0 + b.fract(),
            _ => b.fract()
        };
        let result = source.fract();
        assert!(no_overlap(result.hi, result.lo), "Overlap in fract({:?})", source);
        println!("{:?}", result);
        assert!(result.hi.trunc() == 0.0
            || (result.hi.trunc().abs() == 1.0 && ((result.hi >= 0.0) != (result.lo >= 0.0))),
            "Fractional part of {:?} is greater than one", source);
        assert!(result.lo.trunc() == 0.0, "Non-zero integer part of low word of fract({:?}", source);
        assert_eq_ulp!(result.hi, expected, 1, "Mismatch in fractional part of {:?}", source);
    });

    randomized_test!(fract_no_fract_test, |rng: F64Rand| {
        let (a_fract, b_fract) = get_valid_pair(rng, |a: f64, b: f64| { no_overlap(a.trunc(), b.trunc()) });
        let source = TwoFloat { hi: a_fract.trunc(), lo: b_fract.trunc() };
        let expected = TwoFloat::from(0.0);
        let result = source.fract();
        assert_eq!(result, expected, "Non-zero fractional part of integer {:?}", source);
    });

    randomized_test!(trunc_hi_fract_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { a.fract() != 0.0 && no_overlap(a, b) });
        let source = TwoFloat { hi: a, lo: b };
        let expected = TwoFloat { hi: a.trunc(), lo: 0f64 };
        let result = source.trunc();

        assert!(no_overlap(result.hi, result.lo), "Overlap in trunc({:?})", source);
        assert!(result.hi.fract() == 0.0, "Fractional part remains in high word after truncating {:?}", source);
        assert!(result.lo.fract() == 0.0, "Fractional part remains in low word after truncating {:?}", source);
        assert_eq!(result, expected, "Incorrect value of trunc({:?})", source);
    });

    randomized_test!(trunc_lo_fract_test, |rng: F64Rand| {
        let (a_fract, b) = get_valid_pair(rng, |a: f64, b: f64| { b.fract() != 0.0 && no_overlap(a.trunc(), b) });
        let a = a_fract.trunc();
        let source = TwoFloat { hi: a, lo: b };
        let (expected_a, expected_b) = match (a >= 0.0, b >= 0.0) {
            (true, false) => two_sum(a, b.trunc() - 1.0),
            (false, true) => two_sum(a, b.trunc() + 1.0),
            _ => (a, b.trunc())
        };
        let expected = TwoFloat { hi: expected_a, lo: expected_b };
        let result = source.trunc();
        assert!(no_overlap(result.hi, result.lo), "Overlap in trunc({:?})", source);
        assert!(result.hi.fract() == 0.0, "Fractional part remains in high word after truncating {:?}", source);
        assert!(result.lo.fract() == 0.0, "Fractional part remains in low word after truncating {:?}", source);
        assert_eq!(result, expected, "Incorrect value in trunc({:?})", source);
    });

    randomized_test!(trunc_no_fract_test, |rng: F64Rand| {
        let (a_fract, b_fract) = get_valid_pair(rng, |a: f64, b: f64| { no_overlap(a.trunc(), b.trunc()) });
        let source = TwoFloat { hi: a_fract.trunc(), lo: b_fract.trunc() };
        let expected = source;
        let result = source.trunc();
        assert_eq!(result, expected, "Truncation of integer {:?} returned different value", source);
    });

    randomized_test!(sqrt_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { a > 0.0 && no_overlap(a, b) });
        let source = TwoFloat { hi: a, lo: b };
        let result = source.sqrt();
        assert!(no_overlap(result.hi, result.lo), "Square root of {:?} gave overlap", source);
        let difference = (result * result - source).abs() / source;
        assert!(difference < 1e-16, "Square root of {:?} ({:?}) squared gives high relative difference {}", source, result, difference.hi);
    });

    randomized_test!(sqrt_test_negative, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { a < 0.0 && no_overlap(a, b) });
        let source = TwoFloat { hi: a, lo: b };
        let result = source.sqrt();
        assert!(!result.is_valid(), "Square root of negative number {:?} gave non-error result", source);
    });
}
