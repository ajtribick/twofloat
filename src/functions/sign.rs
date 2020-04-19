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
        self.hi.is_sign_positive()
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
        self.hi.is_sign_negative()
    }

    /// Returns a number composed of the magnitude of `self` and the sign of
    /// `sign`.
    ///
    /// Equal to `self` if the sign of `self` and `sign` are the same,
    /// otherwise equal to `-self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::new_add(-1.0, 1.0e-200);
    /// let b = TwoFloat::new_add(1.0, 0.3);
    /// let c = a.copysign(&b);
    ///
    /// assert_eq!(c, -a);
    pub fn copysign(&self, sign: &TwoFloat) -> TwoFloat {
        if self.is_sign_positive() == sign.is_sign_positive() { *self } else { -self }
    }

    /// Returns a number that represents the sign of the value.
    ///
    /// * `1.0` if the number is positive or `+0.0`
    /// * `-1.0` if the number is negative or `-0.0`
    /// * Invalid value otherwise
    ///
    /// # Examples
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(3.5);
    /// let b = TwoFloat::from(-0.0);
    ///
    /// assert_eq!(a.signum(), 1.0);
    /// assert_eq!(b.signum(), -1.0);
    pub fn signum(&self) -> TwoFloat {
        if self.is_valid() {
            if self.is_sign_positive() { TwoFloat::from(1.0) } else { TwoFloat::from(-1.0) }
        } else {
            TwoFloat { hi: std::f64::NAN, lo: std::f64::NAN }
        }
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
        assert!(TwoFloat { hi: 1.0, lo: -1e-300}.is_sign_positive());
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
}
