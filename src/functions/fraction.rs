use crate::arithmetic::*;
use crate::base::*;

impl TwoFloat {
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
                _ => (self.hi, self.lo.trunc()),
            }
        } else {
            (self.hi.trunc(), 0.0)
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
            (self.hi.ceil(), 0.0)
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
            (self.hi.floor(), 0.0)
        };

        TwoFloat { hi: a, lo: b }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::*;

    randomized_test!(fract_hi_fract_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |x, y| x.fract() != 0.0 && no_overlap(x, y));
        let source = TwoFloat { hi: a, lo: b };
        let expected = a.fract() + b.fract();
        let result = source.fract();
        assert!(
            no_overlap(result.hi, result.lo),
            "Overlap in fract({:?})",
            source
        );
        assert!(
            result.hi.trunc() == 0.0
                || (result.hi.trunc().abs() == 1.0 && ((result.hi >= 0.0) != (result.lo >= 0.0))),
            "Fractional part of {:?} is greater than one",
            source
        );
        assert!(
            result.lo.trunc() == 0.0,
            "Non-zero integer part of low word of fract({:?}",
            source
        );
        assert_eq_ulp!(
            result.hi,
            expected,
            1,
            "Mismatch in fractional part of {:?}",
            source
        );
    });

    randomized_test!(fract_lo_fract_test, |rng: F64Rand| {
        let (a_fract, b) = get_valid_pair(rng, |x, y| y.fract() != 0.0 && no_overlap(x.trunc(), y));
        let a = a_fract.trunc();
        let source = TwoFloat { hi: a, lo: b };
        let expected = match (a >= 0.0, b >= 0.0) {
            (true, false) => 1.0 + b.fract(),
            (false, true) => -1.0 + b.fract(),
            _ => b.fract(),
        };
        let result = source.fract();
        assert!(
            no_overlap(result.hi, result.lo),
            "Overlap in fract({:?})",
            source
        );
        println!("{:?}", result);
        assert!(
            result.hi.trunc() == 0.0
                || (result.hi.trunc().abs() == 1.0 && ((result.hi >= 0.0) != (result.lo >= 0.0))),
            "Fractional part of {:?} is greater than one",
            source
        );
        assert!(
            result.lo.trunc() == 0.0,
            "Non-zero integer part of low word of fract({:?}",
            source
        );
        assert_eq_ulp!(
            result.hi,
            expected,
            1,
            "Mismatch in fractional part of {:?}",
            source
        );
    });

    randomized_test!(fract_no_fract_test, |rng: F64Rand| {
        let (a_fract, b_fract) = get_valid_pair(rng, |x, y| no_overlap(x.trunc(), y.trunc()));
        let source = TwoFloat {
            hi: a_fract.trunc(),
            lo: b_fract.trunc(),
        };
        let expected = TwoFloat::from(0.0);
        let result = source.fract();
        assert_eq!(
            result, expected,
            "Non-zero fractional part of integer {:?}",
            source
        );
    });

    randomized_test!(trunc_hi_fract_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |x, y| x.fract() != 0.0 && no_overlap(x, y));
        let source = TwoFloat { hi: a, lo: b };
        let expected = TwoFloat {
            hi: a.trunc(),
            lo: 0.0,
        };
        let result = source.trunc();

        assert!(
            no_overlap(result.hi, result.lo),
            "Overlap in trunc({:?})",
            source
        );
        assert!(
            result.hi.fract() == 0.0,
            "Fractional part remains in high word after truncating {:?}",
            source
        );
        assert!(
            result.lo.fract() == 0.0,
            "Fractional part remains in low word after truncating {:?}",
            source
        );
        assert_eq!(result, expected, "Incorrect value of trunc({:?})", source);
    });

    randomized_test!(trunc_lo_fract_test, |rng: F64Rand| {
        let (a_fract, b) = get_valid_pair(rng, |x, y| y.fract() != 0.0 && no_overlap(x.trunc(), y));
        let a = a_fract.trunc();
        let source = TwoFloat { hi: a, lo: b };
        let (expected_a, expected_b) = match (a >= 0.0, b >= 0.0) {
            (true, false) => two_sum(a, b.trunc() - 1.0),
            (false, true) => two_sum(a, b.trunc() + 1.0),
            _ => (a, b.trunc()),
        };
        let expected = TwoFloat {
            hi: expected_a,
            lo: expected_b,
        };
        let result = source.trunc();
        assert!(
            no_overlap(result.hi, result.lo),
            "Overlap in trunc({:?})",
            source
        );
        assert!(
            result.hi.fract() == 0.0,
            "Fractional part remains in high word after truncating {:?}",
            source
        );
        assert!(
            result.lo.fract() == 0.0,
            "Fractional part remains in low word after truncating {:?}",
            source
        );
        assert_eq!(result, expected, "Incorrect value in trunc({:?})", source);
    });

    randomized_test!(trunc_no_fract_test, |rng: F64Rand| {
        let (a_fract, b_fract) = get_valid_pair(rng, |x, y| no_overlap(x.trunc(), y.trunc()));
        let source = TwoFloat {
            hi: a_fract.trunc(),
            lo: b_fract.trunc(),
        };
        let expected = source;
        let result = source.trunc();
        assert_eq!(
            result, expected,
            "Truncation of integer {:?} returned different value",
            source
        );
    });

    randomized_test!(ceil_hi_fract_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |x, y| x.fract() != 0.0 && no_overlap(x, y));
        let source = TwoFloat { hi: a, lo: b };
        let expected = TwoFloat {
            hi: a.ceil(),
            lo: 0.0,
        };
        let result = source.ceil();

        assert!(
            no_overlap(result.hi, result.lo),
            "ceil({:?}) contained overlap",
            source
        );
        assert_eq!(result, expected, "Incorrect value of ceil({:?})", source);
    });

    randomized_test!(ceil_lo_fract_test, |rng: F64Rand| {
        let (a_fract, b) = get_valid_pair(rng, |x, y| y.fract() != 0.0 && no_overlap(x.trunc(), y));
        let a = a_fract.trunc();
        let source = TwoFloat { hi: a, lo: b };
        let expected = TwoFloat::new_add(a, b.ceil());
        let result = source.ceil();

        assert!(
            no_overlap(result.hi, result.lo),
            "ceil({:?}) contained overlap",
            source
        );
        assert_eq!(result, expected, "Incorrect value of ceil({:?})", source);
    });

    randomized_test!(ceil_no_fract_test, |rng: F64Rand| {
        let (a_fract, b_fract) = get_valid_pair(rng, |x, y| no_overlap(x.trunc(), y.trunc()));
        let source = TwoFloat {
            hi: a_fract.trunc(),
            lo: b_fract.trunc(),
        };
        let expected = source;
        let result = source.ceil();
        assert!(
            no_overlap(result.hi, result.lo),
            "ceil({:?}) contained overlap",
            source
        );
        assert_eq!(
            result, expected,
            "Ceil of integer {:?} returned different value",
            source
        );
    });

    randomized_test!(floor_hi_fract_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |x, y| x.fract() != 0.0 && no_overlap(x, y));
        let source = TwoFloat { hi: a, lo: b };
        let expected = TwoFloat {
            hi: a.floor(),
            lo: 0.0,
        };
        let result = source.floor();

        assert!(
            no_overlap(result.hi, result.lo),
            "floor({:?}) contained overlap",
            source
        );
        assert_eq!(result, expected, "Incorrect value of floor({:?})", source);
    });

    randomized_test!(floor_lo_fract_test, |rng: F64Rand| {
        let (a_fract, b) = get_valid_pair(rng, |x, y| y.fract() != 0.0 && no_overlap(x.trunc(), y));
        let a = a_fract.trunc();
        let source = TwoFloat { hi: a, lo: b };
        let expected = TwoFloat::new_add(a, b.floor());
        let result = source.floor();

        assert!(
            no_overlap(result.hi, result.lo),
            "floor({:?}) contained overlap",
            source
        );
        assert_eq!(result, expected, "Incorrect value of floor({:?})", source);
    });

    randomized_test!(floor_no_fract_test, |rng: F64Rand| {
        let (a_fract, b_fract) = get_valid_pair(rng, |x, y| no_overlap(x.trunc(), y.trunc()));
        let source = TwoFloat {
            hi: a_fract.trunc(),
            lo: b_fract.trunc(),
        };
        let expected = source;
        let result = source.floor();
        assert!(
            no_overlap(result.hi, result.lo),
            "floor({:?}) contained overlap",
            source
        );
        assert_eq!(
            result, expected,
            "Floor of integer value {:?} returned different value",
            source
        );
    });
}
