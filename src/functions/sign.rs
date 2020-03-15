use crate::base::*;

impl TwoFloat {
    /// Returns the absolute value root of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::new_add(1.0, 1.0e-300).abs();
    /// let b = TwoFloat::new_add(-1.0, 1.0e-300).abs();
    ///
    /// assert_eq!(a, TwoFloat::new_add(1.0, 1.0e-300));
    /// assert_eq!(b, TwoFloat::new_add(1.0, -1.0e-300));
    pub fn abs(&self) -> TwoFloat {
        if self.hi > 0.0
            || (self.hi == 0.0 && self.hi.is_sign_positive() && self.lo.is_sign_positive())
        {
            self.clone()
        } else {
            -self
        }
    }

    /// Returns `true` if `self` has a positive sign, including `+0.0`.
    ///
    /// # Examples
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
    /// # Examples
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abs_test() {
        assert_eq!(
            TwoFloat { hi: 0.0, lo: 0.0 }.abs(),
            TwoFloat { hi: 0.0, lo: 0.0 }
        );
        assert!(TwoFloat { hi: 0.0, lo: -0.0 }.abs().lo.is_sign_positive());
        assert!(TwoFloat { hi: -0.0, lo: 0.0 }.abs().lo.is_sign_negative());
    }

    #[test]
    fn is_sign_positive_test() {
        assert!(TwoFloat { hi: 0.0, lo: -0.0 }.is_sign_positive());
        assert!(!TwoFloat { hi: -0.0, lo: 0.0 }.is_sign_positive());
        assert!(!TwoFloat { hi: -0.0, lo: -0.0 }.is_sign_positive());
        assert!(TwoFloat {
            hi: 1.0,
            lo: -1e-300
        }
        .is_sign_positive());
        assert!(!TwoFloat {
            hi: -1.0,
            lo: -1e-300
        }
        .is_sign_positive());
    }

    #[test]
    fn is_sign_negative_test() {
        assert!(!TwoFloat { hi: 0.0, lo: -0.0 }.is_sign_negative());
        assert!(TwoFloat { hi: -0.0, lo: 0.0 }.is_sign_negative());
        assert!(TwoFloat { hi: -0.0, lo: -0.0 }.is_sign_negative());
        assert!(!TwoFloat {
            hi: 1.0,
            lo: -1e-300
        }
        .is_sign_negative());
        assert!(TwoFloat {
            hi: -1.0,
            lo: -1e-300
        }
        .is_sign_negative());
    }
}
