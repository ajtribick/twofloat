use crate::arithmetic::*;
use crate::base::TwoFloat;

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
    pub fn fract(self) -> TwoFloat {
        let hi_fract = self.hi.fract();
        let lo_fract = self.lo.fract();
        if lo_fract == 0.0 {
            hi_fract.into()
        } else if hi_fract == 0.0 {
            match (self.hi >= 0.0, self.lo >= 0.0) {
                (true, false) => fast_two_sum(1.0, lo_fract),
                (false, true) => fast_two_sum(-1.0, lo_fract),
                _ => self.lo.fract().into(),
            }
        } else {
            fast_two_sum(self.hi.fract(), self.lo)
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
    pub fn trunc(self) -> TwoFloat {
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
    pub fn ceil(self) -> TwoFloat {
        if self.lo.fract() == 0.0 {
            TwoFloat {
                hi: self.hi.ceil(),
                lo: self.lo,
            }
        } else if self.hi.fract() == 0.0 {
            fast_two_sum(self.hi, self.lo.ceil())
        } else {
            self.hi.ceil().into()
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
    pub fn floor(self) -> TwoFloat {
        if self.lo.fract() == 0.0 {
            TwoFloat {
                hi: self.hi.floor(),
                lo: self.lo,
            }
        } else if self.hi.fract() == 0.0 {
            fast_two_sum(self.hi, self.lo.floor())
        } else {
            self.hi.floor().into()
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
    pub fn round(self) -> TwoFloat {
        if self.lo.fract() == 0.0 {
            TwoFloat {
                hi: self.hi.round(),
                lo: self.lo(),
            }
        } else if self.hi.fract() == 0.0 {
            if self.lo.fract().abs() == 0.5 {
                if self.is_sign_positive() {
                    fast_two_sum(self.hi, self.lo.ceil())
                } else {
                    fast_two_sum(self.hi, self.lo.floor())
                }
            } else {
                fast_two_sum(self.hi, self.lo.round())
            }
        } else if self.hi.fract().abs() == 0.5 {
            if self.hi.is_sign_positive() == self.lo.is_sign_positive() {
                self.hi.round().into()
            } else {
                self.hi.trunc().into()
            }
        } else {
            self.hi.round().into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
