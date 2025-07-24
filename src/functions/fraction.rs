use crate::{arithmetic::fast_two_sum, TwoFloat};

impl TwoFloat {
    /// Returns the fractional part of the number.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::new_add(1.0, 1e-200).fract();
    /// let b = TwoFloat::new_add(-1.0, 1e-200).fract();
    ///
    /// assert_eq!(a, TwoFloat::from(1e-200));
    /// assert_eq!(b, TwoFloat::new_add(-1.0, 1e-200));
    /// ```
    pub fn fract(self) -> Self {
        let hi_fract = libm::modf(self.hi).0;
        let lo_fract = libm::modf(self.lo).0;
        if lo_fract == 0.0 {
            hi_fract.into()
        } else if hi_fract == 0.0 {
            match (self.hi >= 0.0, self.lo >= 0.0) {
                (true, false) => fast_two_sum(1.0, lo_fract),
                (false, true) => fast_two_sum(-1.0, lo_fract),
                _ => libm::modf(self.lo).0.into(),
            }
        } else {
            fast_two_sum(libm::modf(self.hi).0, self.lo)
        }
    }

    /// Returns the integer part of the number.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::new_add(1.0, 1e-200).trunc();
    /// let b = TwoFloat::new_add(1.0, -1e-200).trunc();
    ///
    /// assert_eq!(a, TwoFloat::from(1.0));
    /// assert_eq!(b, TwoFloat::from(0.0));
    /// ```
    pub fn trunc(self) -> Self {
        if self.is_sign_positive() {
            self.floor()
        } else {
            self.ceil()
        }
    }

    /// Returns the smallest integer greater than or equal to the number.
    ///
    /// # Examples
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
    /// ```
    pub fn ceil(self) -> Self {
        if libm::modf(self.lo).0 == 0.0 {
            Self {
                hi: libm::ceil(self.hi),
                lo: self.lo,
            }
        } else if libm::modf(self.hi).0 == 0.0 {
            fast_two_sum(self.hi, libm::ceil(self.lo))
        } else {
            libm::ceil(self.hi).into()
        }
    }

    /// Returns the smallest integer less than or equal to the number.
    ///
    /// # Examples
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
    /// ```
    pub fn floor(self) -> Self {
        if libm::modf(self.lo).0 == 0.0 {
            Self {
                hi: libm::floor(self.hi),
                lo: self.lo,
            }
        } else if libm::modf(self.hi).0 == 0.0 {
            fast_two_sum(self.hi, libm::floor(self.lo))
        } else {
            libm::floor(self.hi).into()
        }
    }

    /// Returns the nearest integer to the value. Round half-way cases away
    /// from `0.0`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::new_add(1.0, 1e-200).round();
    /// let b = TwoFloat::new_add(1.0, -1e-200).round();
    /// let c = TwoFloat::from(-0.5).round();
    ///
    /// assert_eq!(a, TwoFloat::from(1.0));
    /// assert_eq!(b, TwoFloat::from(1.0));
    /// assert_eq!(c, TwoFloat::from(-1.0));
    /// ```
    pub fn round(self) -> Self {
        if libm::modf(self.lo).0 == 0.0 {
            Self {
                hi: libm::round(self.hi),
                lo: self.lo(),
            }
        } else if libm::modf(self.hi).0 == 0.0 {
            if libm::fabs(libm::modf(self.lo).0) == 0.5 {
                if self.is_sign_positive() {
                    fast_two_sum(self.hi, libm::ceil(self.lo))
                } else {
                    fast_two_sum(self.hi, libm::floor(self.lo))
                }
            } else {
                fast_two_sum(self.hi, libm::round(self.lo))
            }
        } else if libm::fabs(libm::modf(self.hi).0) == 0.5 {
            if self.hi.is_sign_positive() == self.lo.is_sign_positive() {
                libm::round(self.hi).into()
            } else {
                libm::trunc(self.hi).into()
            }
        } else {
            libm::round(self.hi).into()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::TwoFloat;

    const EXP2_60: f64 = 1152921504606846976.0; // 2^60

    #[test]
    fn trunc_test() {
        assert_eq!(TwoFloat::from(1.25).trunc(), 1.0);
        assert_eq!(TwoFloat::from(-1.25).trunc(), -1.0);

        assert_eq!(TwoFloat::new_add(5.0, 1e-200).trunc(), 5.0);
        assert_eq!(TwoFloat::new_add(5.0, -1e-200).trunc(), 4.0);
        assert_eq!(TwoFloat::new_add(-5.0, 1e-200).trunc(), -4.0);
        assert_eq!(TwoFloat::new_add(-5.0, -1e-200).trunc(), -5.0);

        assert_eq!(
            TwoFloat::new_add(EXP2_60, 1.5).trunc(),
            TwoFloat::new_add(EXP2_60, 1.0)
        );
        assert_eq!(
            TwoFloat::new_add(EXP2_60, -1.5).trunc(),
            TwoFloat::new_add(EXP2_60, -2.0)
        );
        assert_eq!(
            TwoFloat::new_add(-EXP2_60, 1.5).trunc(),
            TwoFloat::new_add(-EXP2_60, 2.0)
        );
        assert_eq!(
            TwoFloat::new_add(-EXP2_60, -1.5).trunc(),
            TwoFloat::new_add(-EXP2_60, -1.0)
        );
    }

    #[test]
    fn ceil_test() {
        assert_eq!(1.0, TwoFloat::from(0.25).ceil());
        assert_eq!(0.0, TwoFloat::from(-0.25).ceil());

        assert_eq!(TwoFloat::new_add(5.0, 1e-200).ceil(), 6.0);
        assert_eq!(TwoFloat::new_add(5.0, -1e-200).ceil(), 5.0);
        assert_eq!(TwoFloat::new_add(-5.0, 1e-200).ceil(), -4.0);
        assert_eq!(TwoFloat::new_add(-5.0, -1e-200).ceil(), -5.0);

        assert_eq!(
            TwoFloat::new_add(EXP2_60, 1.5).ceil(),
            TwoFloat::new_add(EXP2_60, 2.0)
        );
        assert_eq!(
            TwoFloat::new_add(EXP2_60, -1.5).ceil(),
            TwoFloat::new_add(EXP2_60, -1.0)
        );
        assert_eq!(
            TwoFloat::new_add(-EXP2_60, 1.5).ceil(),
            TwoFloat::new_add(-EXP2_60, 2.0)
        );
        assert_eq!(
            TwoFloat::new_add(-EXP2_60, -1.5).ceil(),
            TwoFloat::new_add(-EXP2_60, -1.0)
        );
    }

    #[test]
    fn floor_test() {
        assert_eq!(0.0, TwoFloat::from(0.25).floor());
        assert_eq!(-1.0, TwoFloat::from(-0.25).floor());

        assert_eq!(TwoFloat::new_add(5.0, 1e-200).floor(), 5.0);
        assert_eq!(TwoFloat::new_add(5.0, -1e-200).floor(), 4.0);
        assert_eq!(TwoFloat::new_add(-5.0, 1e-200).floor(), -5.0);
        assert_eq!(TwoFloat::new_add(-5.0, -1e-200).floor(), -6.0);

        assert_eq!(
            TwoFloat::new_add(EXP2_60, 1.5).floor(),
            TwoFloat::new_add(EXP2_60, 1.0)
        );
        assert_eq!(
            TwoFloat::new_add(EXP2_60, -1.5).floor(),
            TwoFloat::new_add(EXP2_60, -2.0)
        );
        assert_eq!(
            TwoFloat::new_add(-EXP2_60, 1.5).floor(),
            TwoFloat::new_add(-EXP2_60, 1.0)
        );
        assert_eq!(
            TwoFloat::new_add(-EXP2_60, -1.5).floor(),
            TwoFloat::new_add(-EXP2_60, -2.0)
        );
    }

    #[test]
    fn round_test() {
        assert_eq!(1.0, TwoFloat::from(0.5).round());
        assert_eq!(2.0, TwoFloat::from(1.5).round());
        assert_eq!(-1.0, TwoFloat::from(-0.5).round());
        assert_eq!(-2.0, TwoFloat::from(-1.5).round());

        assert_eq!(1.0, TwoFloat::from(0.9).round());
        assert_eq!(1.0, TwoFloat::from(1.1).round());
        assert_eq!(-1.0, TwoFloat::from(-0.9).round());
        assert_eq!(-1.0, TwoFloat::from(-1.1).round());

        assert_eq!(TwoFloat::new_add(5.0, 1e-200).round(), 5.0);
        assert_eq!(TwoFloat::new_add(5.0, -1e-200).round(), 5.0);
        assert_eq!(TwoFloat::new_add(-5.0, 1e-200).round(), -5.0);
        assert_eq!(TwoFloat::new_add(-5.0, -1e-200).round(), -5.0);

        assert_eq!(TwoFloat::new_add(1.5, 1e-200).round(), 2.0);
        assert_eq!(TwoFloat::new_add(1.5, -1e-200).round(), 1.0);
        assert_eq!(TwoFloat::new_add(-1.5, 1e-200).round(), -1.0);
        assert_eq!(TwoFloat::new_add(-1.5, -1e-200).round(), -2.0);

        assert_eq!(
            TwoFloat::new_add(EXP2_60, 0.9).round(),
            TwoFloat::new_add(EXP2_60, 1.0)
        );
        assert_eq!(
            TwoFloat::new_add(EXP2_60, 1.1).round(),
            TwoFloat::new_add(EXP2_60, 1.0)
        );
        assert_eq!(
            TwoFloat::new_add(EXP2_60, -0.9).round(),
            TwoFloat::new_add(EXP2_60, -1.0)
        );
        assert_eq!(
            TwoFloat::new_add(EXP2_60, -1.1).round(),
            TwoFloat::new_add(EXP2_60, -1.0)
        );
        assert_eq!(
            TwoFloat::new_add(-EXP2_60, 0.9).round(),
            TwoFloat::new_add(-EXP2_60, 1.0)
        );
        assert_eq!(
            TwoFloat::new_add(-EXP2_60, 1.1).round(),
            TwoFloat::new_add(-EXP2_60, 1.0)
        );
        assert_eq!(
            TwoFloat::new_add(-EXP2_60, -0.9).round(),
            TwoFloat::new_add(-EXP2_60, -1.0)
        );
        assert_eq!(
            TwoFloat::new_add(-EXP2_60, -1.1).round(),
            TwoFloat::new_add(-EXP2_60, -1.0)
        );

        assert_eq!(
            TwoFloat::new_add(EXP2_60, 1.5).round(),
            TwoFloat::new_add(EXP2_60, 2.0)
        );
        assert_eq!(
            TwoFloat::new_add(EXP2_60, -1.5).round(),
            TwoFloat::new_add(EXP2_60, -1.0)
        );
        assert_eq!(
            TwoFloat::new_add(-EXP2_60, 1.5).round(),
            TwoFloat::new_add(-EXP2_60, 1.0)
        );
        assert_eq!(
            TwoFloat::new_add(-EXP2_60, -1.5).round(),
            TwoFloat::new_add(-EXP2_60, -2.0)
        );
    }
}
