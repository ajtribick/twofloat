use crate::arithmetic::*;
use crate::base::*;

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
    pub fn fract(&self) -> TwoFloat {
        let hi_fract = self.hi.fract();
        let lo_fract = self.lo.fract();
        let (a, b) = if lo_fract == 0.0 {
            (hi_fract, 0.0)
        } else if hi_fract == 0.0 {
            match (self.hi >= 0.0, self.lo >= 0.0) {
                (true, false) => fast_two_sum(1.0, lo_fract),
                (false, true) => fast_two_sum(-1.0, lo_fract),
                _ => (self.lo.fract(), 0.0),
            }
        } else {
            fast_two_sum(self.hi.fract(), self.lo)
        };

        TwoFloat { hi: a, lo: b }
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
    pub fn trunc(&self) -> TwoFloat {
        let (a, b) = if self.lo.fract() == 0.0 {
            (self.hi.trunc(), self.lo)
        } else if self.hi.fract() == 0.0 {
            match (self.hi >= 0.0, self.lo >= 0.0) {
                (true, false) => fast_two_sum(self.hi, self.lo.trunc() - 1.0),
                (false, true) => fast_two_sum(self.hi, self.lo.trunc() + 1.0),
                _ => (self.hi, self.lo.trunc()),
            }
        } else {
            (self.hi.trunc(), 0.0)
        };

        TwoFloat { hi: a, lo: b }
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
    pub fn ceil(&self) -> TwoFloat {
        let (a, b) = if self.lo.fract() == 0.0 {
            (self.hi.ceil(), self.lo)
        } else if self.hi.fract() == 0.0 {
            fast_two_sum(self.hi, self.lo.ceil())
        } else {
            (self.hi.ceil(), 0.0)
        };

        TwoFloat { hi: a, lo: b }
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
    pub fn floor(&self) -> TwoFloat {
        let (a, b) = if self.lo.fract() == 0.0 {
            (self.hi.floor(), self.lo)
        } else if self.hi.fract() == 0.0 {
            fast_two_sum(self.hi, self.lo.floor())
        } else {
            (self.hi.floor(), 0.0)
        };

        TwoFloat { hi: a, lo: b }
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
    pub fn round(&self) -> TwoFloat {
        let (a, b) = if self.lo.fract() == 0.0 {
            (self.hi.round(), self.lo())
        } else if self.hi.fract() == 0.0 {
            fast_two_sum(self.hi, self.lo.round())
        } else {
            (self.hi.round(), 0.0)
        };

        TwoFloat { hi: a, lo: b }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trunc_test() {
        assert_eq!(0.0, TwoFloat::from(0.25).trunc());
        assert_eq!(0.0, TwoFloat::from(-0.25).trunc());
    }

    #[test]
    fn ceil_test() {
        assert_eq!(1.0, TwoFloat::from(0.25).ceil());
        assert_eq!(0.0, TwoFloat::from(-0.25).ceil());
    }

    #[test]
    fn floor_test() {
        assert_eq!(0.0, TwoFloat::from(0.25).floor());
        assert_eq!(-1.0, TwoFloat::from(-0.25).floor());
    }

    #[test]
    fn round_test() {
        assert_eq!(1.0, TwoFloat::from(0.5).round());
        assert_eq!(2.0, TwoFloat::from(1.5).round());
        assert_eq!(-1.0, TwoFloat::from(-0.5).round());
        assert_eq!(-2.0, TwoFloat::from(-1.5).round());
    }
}
