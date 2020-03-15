use crate::base::*;

impl TwoFloat {
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
    pub fn recip(&self) -> TwoFloat {
        1.0 / self
    }

    /// Returns the square root of the number, using equation 4 from Karp &
    /// Markstein (1997).
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(2.0);
    /// let b = a.sqrt();
    ///
    /// assert!(b * b - a < 1e-16);
    pub fn sqrt(&self) -> TwoFloat {
        if self.hi < 0.0 || (self.hi == 0.0 && self.lo < 0.0) {
            TwoFloat {
                hi: std::f64::NAN,
                lo: std::f64::NAN,
            }
        } else if self.hi == 0.0 && self.lo == 0.0 {
            TwoFloat { hi: 0.0, lo: 0.0 }
        } else {
            let x = self.hi.sqrt().recip();
            let y = self.hi * x;
            TwoFloat::new_add(y, (self - TwoFloat::new_mul(y, y)).hi * (x * 0.5))
        }
    }

    /// Returns the cube root of the number, using Newton-Raphson iteration.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::new_add(1.4e53, 0.21515);
    /// let b = a.cbrt();
    ///
    /// assert!(b.powi(3) - a < 1e-16);
    pub fn cbrt(&self) -> TwoFloat {
        let mut x = TwoFloat::from(self.hi.cbrt());
        let mut x2 = x * x;
        x -= (&x2 * &x - self) / (3.0 * &x2);
        x2 = x * x;
        x - (&x2 * &x - self) / (3.0 * &x2)
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
    pub fn powi(&self, n: i32) -> TwoFloat {
        match n {
            0 => {
                if self.hi == 0.0 && self.lo == 0.0 {
                    TwoFloat {
                        hi: std::f64::NAN,
                        lo: std::f64::NAN,
                    }
                } else {
                    TwoFloat::from(1.0)
                }
            }
            1 => self.clone(),
            -1 => self.recip(),
            _ => {
                let mut result = TwoFloat::from(1.0);
                let mut n_pos = n.abs();
                let mut value = self.clone();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::*;

    use rand::Rng;

    randomized_test!(recip_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| no_overlap(a, b));
        let source = TwoFloat { hi: a, lo: b };
        let result = source.recip();

        assert!(
            no_overlap(a, b),
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
        let (a, b) = get_valid_pair(rng, |x, y| x > 0.0 && no_overlap(x, y));
        let source = TwoFloat { hi: a, lo: b };
        let result = source.sqrt();
        assert!(
            no_overlap(result.hi, result.lo),
            "Square root of {:?} gave overlap",
            source
        );
        let difference = (&result * &result - &source).abs() / &source;
        assert!(
            difference < 1e-16,
            "Square root of {:?} ({:?}) squared gives high relative difference {}",
            source,
            result,
            difference.hi
        );
    });

    randomized_test!(sqrt_negative_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |x, y| x < 0.0 && no_overlap(x, y));
        let source = TwoFloat { hi: a, lo: b };
        let result = source.sqrt();
        assert!(
            !result.is_valid(),
            "Square root of negative number {:?} gave non-error result",
            source
        );
    });

    randomized_test!(cbrt_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |x, y| no_overlap(x, y));
        let source = TwoFloat { hi: a, lo: b };
        let result = source.cbrt();
        assert!(
            no_overlap(result.hi, result.lo),
            "Cube root of {:?} gave overlap",
            source
        );
        let difference = (result.powi(3) - &source).abs() / &source;
        assert!(
            difference < 1e-16,
            "Cube root of {:?} ({:?}) squared gives high relative difference {}",
            source,
            result,
            difference.hi
        );
    });

    randomized_test!(powi_0_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |x, y| x != 0.0 && no_overlap(x, y));
        let source = TwoFloat { hi: a, lo: b };
        let expected = TwoFloat { hi: 1.0, lo: 0.0 };
        let result = source.powi(0);

        assert!(
            no_overlap(result.hi, result.lo),
            "Result of {:?}.powi(0) contained overlap",
            source
        );
        assert_eq!(result, expected, "{:?}.powi(0) did not return 1", source);
    });

    randomized_test!(powi_1_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |x, y| no_overlap(x, y));
        let source = TwoFloat { hi: a, lo: b };
        let result = source.powi(1);
        assert!(
            no_overlap(result.hi, result.lo),
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
                no_overlap(result.hi, result.lo),
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
                no_overlap(result.hi, result.lo),
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
}
