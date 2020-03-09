use crate::base::*;
use crate::arithmetic::*;

impl TwoFloat {
    /// Returns the absolute value root of `self`.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::new_add(1.0, 1.0e-300).abs();
    /// let b = TwoFloat::new_add(-1.0, 1.0e-300).abs();
    ///
    /// assert_eq!(a, TwoFloat::new_add(1.0, 1.0e-300));
    /// assert_eq!(b, TwoFloat::new_add(1.0, -1.0e-300));
    pub fn abs(&self) -> TwoFloat {
        if self.hi > 0.0 || (self.hi == 0.0 && self.hi.is_sign_positive() && self.lo.is_sign_positive()) { self.clone() } else { -self }
    }

    /// Returns `true` if `self` has a positive sign, including `+0.0`.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::new_add(0.0, 0.0).is_sign_positive();
    /// let b = TwoFloat::new_add(1.0, 1.0e-300).is_sign_positive();
    /// let c = TwoFloat::new_add(-1.0, 1.0e-300).is_sign_positive();
    ///
    /// assert!(a);
    /// assert!(b);
    /// assert!(!c);
    pub fn is_sign_positive(&self) -> bool {
        self.hi > 0.0 || (self.hi == 0.0 && self.hi.is_sign_positive())
    }

    /// Returns `true` if `self` has a negative sign, including `-0.0`.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::new_add(-1.0, 1.0e-300).is_sign_negative();
    /// let b = TwoFloat::new_add(0.0, 0.0).is_sign_negative();
    /// let c = TwoFloat::new_add(1.0, 1.0e-300).is_sign_negative();
    ///
    /// assert!(a);
    /// assert!(!b);
    /// assert!(!c);
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
    /// let a = TwoFloat::new_add(1.0, 1.0e-300).is_valid();
    /// let b = TwoFloat::new_mul(1.0e300, 1.0e300).is_valid();
    ///
    /// assert!(a);
    /// assert!(!b);
    pub fn is_valid(&self) -> bool {
        self.hi.is_finite() && self.lo.is_finite()
    }

    /// Returns the fractional part of the number.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::new_add(1.0, 1e-200).fract();
    /// let b = TwoFloat::new_add(-1.0, 1e-200).fract();
    ///
    /// assert_eq!(a, TwoFloat::from(1e-200));
    /// assert_eq!(b, TwoFloat::new_add(-1.0, 1e-200));
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
    /// let a = TwoFloat::new_add(1.0, 1e-200).trunc();
    /// let b = TwoFloat::new_add(1.0, -1e-200).trunc();
    ///
    /// assert_eq!(a, TwoFloat::from(1.0));
    /// assert_eq!(b, TwoFloat::from(0.0));
    pub fn trunc(&self) -> TwoFloat {
        let (a, b) = if self.lo.fract() == 0.0 {
            (self.hi, self.lo)
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

    /// Returns the smallest integer greater than or equal to the number.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::new_add(1.0, 1e-200).ceil();
    /// let b = TwoFloat::new_add(1.0, -1e-200).ceil();
    /// let c = TwoFloat::new_add(-1.0, 1e-200).ceil();
    ///
    /// assert_eq!(a, TwoFloat::from(2.0));
    /// assert_eq!(b, TwoFloat::from(1.0));
    /// assert_eq!(c, TwoFloat::from(0.0));
    pub fn ceil(&self) -> TwoFloat {
        let (a, b) = if self.lo.fract() == 0.0 {
            (self.hi, self.lo)
        } else if self.hi.fract() == 0.0 {
            fast_two_sum(self.hi, self.lo.ceil())
        } else {
            (self.hi.ceil(), 0f64)
        };

        TwoFloat { hi: a, lo: b }
    }

    /// Returns the smallest integer less than or equal to the number.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::new_add(1.0, 1e-200).floor();
    /// let b = TwoFloat::new_add(1.0, -1e-200).floor();
    /// let c = TwoFloat::new_add(-1.0, 1e-200).floor();
    ///
    /// assert_eq!(a, TwoFloat::from(1.0));
    /// assert_eq!(b, TwoFloat::from(0.0));
    /// assert_eq!(c, TwoFloat::from(-1.0));
    pub fn floor(&self) -> TwoFloat {
        let (a, b) = if self.lo.fract() == 0.0 {
            (self.hi, self.lo)
        } else if self.hi.fract() == 0.0 {
            fast_two_sum(self.hi, self.lo.floor())
        } else {
            (self.hi.floor(), 0f64)
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
    ///
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

    /// Raises the number to an integer power. Returns a NAN value for 0^0.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(2f64).powi(3);
    /// let b = TwoFloat::from(0f64).powi(0);
    ///
    /// assert!(a - TwoFloat::from(8f64) <= 1e-16);
    /// assert!(!b.is_valid());
    pub fn powi(&self, n: i32) -> TwoFloat {
        match n {
            0 => {
                if self.hi == 0.0 && self.lo == 0.0 {
                    TwoFloat { hi: std::f64::NAN, lo: std::f64::NAN }
                } else {
                    1f64.into()
                }
            },
            1 => { self.clone() },
            -1 => { 1f64 / self },
            _ => {
                let mut result: TwoFloat = 1f64.into();
                let mut n_pos = n.abs();
                let mut value = self.clone();
                while n_pos > 0 {
                    if (n_pos & 1) != 0 { result *= &value; }
                    value *= value;
                    n_pos >>= 1;
                }
                if n > 0 { result } else { 1f64 / result }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::*;

    use rand::Rng;

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

    randomized_test!(ceil_hi_fract_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { a.fract() != 0.0 && no_overlap(a, b) });
        let source = TwoFloat { hi: a, lo: b };
        let expected = TwoFloat { hi: a.ceil(), lo: 0f64 };
        let result = source.ceil();

        assert!(no_overlap(result.hi, result.lo), "ceil({:?}) contained overlap", source);
        assert_eq!(result, expected, "Incorrect value of ceil({:?})", source);
    });

    randomized_test!(ceil_lo_fract_test, |rng: F64Rand| {
        let (a_fract, b) = get_valid_pair(rng, |a: f64, b: f64| { b.fract() != 0.0 && no_overlap(a.trunc(), b) });
        let a = a_fract.trunc();
        let source = TwoFloat { hi: a, lo: b };
        let expected = TwoFloat::new_add(a, b.ceil());
        let result = source.ceil();

        assert!(no_overlap(result.hi, result.lo), "ceil({:?}) contained overlap", source);
        assert_eq!(result, expected, "Incorrect value of ceil({:?})", source);
    });

    randomized_test!(ceil_no_fract_test, |rng: F64Rand| {
        let (a_fract, b_fract) = get_valid_pair(rng, |a: f64, b: f64| { no_overlap(a.trunc(), b.trunc()) });
        let source = TwoFloat { hi: a_fract.trunc(), lo: b_fract.trunc() };
        let expected = source;
        let result = source.ceil();
        assert!(no_overlap(result.hi, result.lo), "ceil({:?}) contained overlap", source);
        assert_eq!(result, expected, "Ceil of integer {:?} returned different value", source);
    });

    randomized_test!(floor_hi_fract_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { a.fract() != 0.0 && no_overlap(a, b) });
        let source = TwoFloat { hi: a, lo: b };
        let expected = TwoFloat { hi: a.floor(), lo: 0f64 };
        let result = source.floor();

        assert!(no_overlap(result.hi, result.lo), "floor({:?}) contained overlap", source);
        assert_eq!(result, expected, "Incorrect value of floor({:?})", source);
    });

    randomized_test!(floor_lo_fract_test, |rng: F64Rand| {
        let (a_fract, b) = get_valid_pair(rng, |a: f64, b: f64| { b.fract() != 0.0 && no_overlap(a.trunc(), b) });
        let a = a_fract.trunc();
        let source = TwoFloat { hi: a, lo: b };
        let expected = TwoFloat::new_add(a, b.floor());
        let result = source.floor();

        assert!(no_overlap(result.hi, result.lo), "floor({:?}) contained overlap", source);
        assert_eq!(result, expected, "Incorrect value of floor({:?})", source);
    });

    randomized_test!(floor_no_fract_test, |rng: F64Rand| {
        let (a_fract, b_fract) = get_valid_pair(rng, |a: f64, b: f64| { no_overlap(a.trunc(), b.trunc()) });
        let source = TwoFloat { hi: a_fract.trunc(), lo: b_fract.trunc() };
        let expected = source;
        let result = source.floor();
        assert!(no_overlap(result.hi, result.lo), "floor({:?}) contained overlap", source);
        assert_eq!(result, expected, "Floor of integer value {:?} returned different value", source);
    });

    randomized_test!(sqrt_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { a > 0.0 && no_overlap(a, b) });
        let source = TwoFloat { hi: a, lo: b };
        let result = source.sqrt();
        assert!(no_overlap(result.hi, result.lo), "Square root of {:?} gave overlap", source);
        let difference = (&result * &result - &source).abs() / &source;
        assert!(difference < 1e-16, "Square root of {:?} ({:?}) squared gives high relative difference {}", source, result, difference.hi);
    });

    randomized_test!(sqrt_negative_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { a < 0.0 && no_overlap(a, b) });
        let source = TwoFloat { hi: a, lo: b };
        let result = source.sqrt();
        assert!(!result.is_valid(), "Square root of negative number {:?} gave non-error result", source);
    });

    randomized_test!(powi_0_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { a != 0.0 && no_overlap(a, b) });
        let source = TwoFloat { hi: a, lo: b };
        let expected = TwoFloat { hi: 1f64, lo: 0f64 };
        let result = source.powi(0);

        assert!(no_overlap(result.hi, result.lo), "Result of {:?}.powi(0) contained overlap", source);
        assert_eq!(result, expected, "{:?}.powi(0) did not return 1", source);
    });

    randomized_test!(powi_1_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { no_overlap(a, b) });
        let source = TwoFloat { hi: a, lo: b };
        let result = source.powi(1);
        assert!(no_overlap(result.hi, result.lo), "{:?}.powi(1) contained overlap", source);
        assert_eq!(result, source, "{:?}.powi(1) did not return same number", source);
    });

    #[test]
    fn powi_value_test() {
        let mut rng = rand::thread_rng();
        for _ in 0..TEST_ITERS {
            let source = TwoFloat::new_add(rng.gen_range(-128f64, 128f64), rng.gen_range(-1.0f64, 1.0f64));
            let exponent = rng.gen_range(1i32, 20i32);
            let mut expected = TwoFloat::from(1f64);
            for _ in 0..exponent {
                expected *= &source;
            }

            let result = source.powi(exponent);
            assert!(no_overlap(result.hi, result.lo), "{:?}.powi({}) contained overlap", source, exponent);

            let difference = (&result - &expected) / &expected;
            assert!(difference.abs() < 1e-10, "Value mismatch in {:?}.powi({})", source, exponent);
        }
    }

    #[test]
    fn powi_reciprocal_test() {
        let mut rng = rand::thread_rng();
        for _ in 0..TEST_ITERS {
            let source = TwoFloat::new_add(rng.gen_range(-128f64, 128f64), rng.gen_range(-1.0f64, 1.0f64));
            let exponent = rng.gen_range(1i32, 20i32);
            let expected = 1.0f64 / source.powi(exponent);
            let result = source.powi(-exponent);
            assert!(no_overlap(result.hi, result.lo), "{:?}.powi({}) contained overlap", source, -exponent);
            assert_eq!(result, expected, "{0:?}.powi({1}) was not reciprocal of {0:?}.powi({2})", source, -exponent, exponent);
        }
    }
}
