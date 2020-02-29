use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::twofloat::TwoFloat;

fn fast_two_sum(a: f64, b: f64) -> (f64, f64) {
    // Joldes et al. (2017) Algorithm 1
    let s = a + b;
    let z = s - a;
    (s, b - z)
}

fn two_sum(a: f64, b: f64) -> (f64, f64) {
    // Joldes et al. (2017) Algorithm 2
    let s = a + b;
    let aa = s - b;
    let bb = s - aa;
    let da = a - aa;
    let db = b - bb;
    (s, da + db)
}

fn two_diff(a: f64, b: f64) -> (f64, f64) {
    // Joldes et al. (2017) Algorithm 2 for negative rhs
    let s = a - b;
    let aa = s + b;
    let bb = s - aa;
    let da = a - aa;
    let db = b + bb;
    (s, da - db)
}

fn two_prod(a: f64, b: f64) -> (f64, f64) {
    // Joldes et al. (2017) Algorithm 3
    let p = a * b;
    (p, a.mul_add(b, -p))
}

impl TwoFloat {
    /// Creates a TwoFloat by adding two f64 values
    pub fn new_add(x: f64, y: f64) -> TwoFloat {
        let (a, b) = two_sum(x, y);
        TwoFloat { hi: a, lo: b }
    }

    /// Creates a TwoFloat by subtracting two f64 values
    pub fn new_sub(x: f64, y: f64) -> TwoFloat {
        let (a, b) = two_diff(x, y);
        TwoFloat { hi: a, lo: b }
    }

    /// Creates a TwoFloat by multiplying two f64 values
    pub fn new_mul(x: f64, y: f64) -> TwoFloat {
        let (a, b) = two_prod(x, y);
        TwoFloat { hi: a, lo: b }
    }

    /// Creates a TwoFloat by dividing two f64 values
    pub fn new_div(x: f64, y: f64) -> TwoFloat {
        // Joldes et al. (2017) Algorithm 15 with xl=0
        let th = x / y;
        let (ph, pl) = two_prod(th, y);
        let dh = x - ph;
        let d = dh - pl;
        let tl = d / y;
        let (a, b) = fast_two_sum(th, tl);
        TwoFloat { hi: a, lo: b }
    }
}

impl Neg for TwoFloat {
    type Output = TwoFloat;

    fn neg(self) -> TwoFloat {
        TwoFloat { hi: -self.hi, lo: -self.lo }
    }
}

macro_rules! op_impl {
    ($op_assign:ident, $op_assign_fn:ident, $op:ident, $op_fn:ident, $code:expr) => {
        impl $op_assign<f64> for TwoFloat {
            fn $op_assign_fn(&mut self, other: f64) {
                let (a, b) = $code(self, other);
                self.hi = a;
                self.lo = b;
            }
        }

        impl $op<f64> for TwoFloat {
            type Output = TwoFloat;

            fn $op_fn(mut self, other: f64) -> TwoFloat {
                let (a, b) = $code(&self, other);
                self.hi = a;
                self.lo = b;
                self
            }
        }

        impl<'a> $op<f64> for &'a TwoFloat {
            type Output = TwoFloat;

            fn $op_fn(self, other: f64) -> TwoFloat {
                let (a, b) = $code(self, other);
                TwoFloat { hi: a, lo: b }
            }
        }

        impl $op<TwoFloat> for f64 {
            type Output = TwoFloat;

            fn $op_fn(self, mut other: TwoFloat) -> TwoFloat {
                other.$op_assign_fn(self);
                other
            }
        }

        impl<'a> $op<&'a TwoFloat> for f64 {
            type Output = TwoFloat;

            fn $op_fn(self, other: &'a TwoFloat) -> TwoFloat {
                let (a, b) = $code(other, self);
                TwoFloat { hi: a, lo: b }
            }
        }
    };
    ($op_assign:ident, $op_assign_fn:ident, $op:ident, $op_fn:ident, $code:expr, $code_rev:expr) => {
        impl $op_assign<f64> for TwoFloat {
            fn $op_assign_fn(&mut self, other: f64) {
                let (a, b) = $code(self, other);
                self.hi = a;
                self.lo = b;
            }
        }

        impl $op<f64> for TwoFloat {
            type Output = TwoFloat;

            fn $op_fn(mut self, other: f64) -> TwoFloat {
                let (a, b) = $code(&self, other);
                self.hi = a;
                self.lo = b;
                self
            }
        }

        impl<'a> $op<f64> for &'a TwoFloat {
            type Output = TwoFloat;

            fn $op_fn(self, other: f64) -> TwoFloat {
                let (a, b) = $code(self, other);
                TwoFloat { hi: a, lo: b }
            }
        }

        impl $op<TwoFloat> for f64 {
            type Output = TwoFloat;

            fn $op_fn(self, mut other: TwoFloat) -> TwoFloat {
                let (a, b) = $code_rev(self, &other);
                other.hi = a;
                other.lo = b;
                other
            }
        }

        impl<'a> $op<&'a TwoFloat> for f64 {
            type Output = TwoFloat;

            fn $op_fn(self, other: &'a TwoFloat) -> TwoFloat {
                let (a, b) = $code_rev(self, other);
                TwoFloat { hi: a, lo: b }
            }
        }
    };
}

op_impl!(AddAssign, add_assign, Add, add, |lhs: &TwoFloat, rhs: f64| {
    // Joldes et al. (2017) Algorithm 4
    let (sh, sl) = two_sum(lhs.hi, rhs);
    let v = lhs.lo + sl;
    fast_two_sum(sh, v)
});

op_impl!(SubAssign, sub_assign, Sub, sub, |lhs: &TwoFloat, rhs: f64| {
    // Joldes et al. (2017) Algorithm 4 for negative rhs
    let (sh, sl) = two_diff(lhs.hi, rhs);
    let v = lhs.lo + sl;
    fast_two_sum(sh, v)
}, |lhs: f64, rhs: &TwoFloat| {
    let (sh, sl) = two_diff(lhs, rhs.hi);
    let v = sl - rhs.lo;
    fast_two_sum(sh, v)
});

op_impl!(MulAssign, mul_assign, Mul, mul, |lhs: &TwoFloat, rhs: f64| {
    // Joldes et al. (2017) Algorithm 9
    let (ch, cl1) = two_prod(lhs.hi, rhs);
    let cl3 = lhs.lo.mul_add(rhs, cl1);
    fast_two_sum(ch, cl3)
});

op_impl!(DivAssign, div_assign, Div, div, |lhs: &TwoFloat, rhs: f64| {
    // Joldes et al. (2017) Algorithm 15
    let th = lhs.hi / rhs;
    let (ph, pl) = two_prod(th, rhs);
    let dh = lhs.hi - ph;
    let dt = dh - pl;
    let d = dt + lhs.lo;
    let tl = d / rhs;
    fast_two_sum(th, tl)
}, |lhs: f64, rhs: &TwoFloat| {
    // Joldes et al. (2017) Algorithm 18 with xl = 0
    let th = 1.0 / rhs.hi;
    let rh = 1.0 - rhs.hi * th;
    let rl = -(rhs.lo * th);
    let (eh, el) = fast_two_sum(rh, rl);
    let e = TwoFloat { hi: eh, lo: el };
    let d = &e * th;
    let m = &d + th;
    let (ch, cl1) = two_prod(m.hi, lhs);
    let cl3 = m.lo.mul_add(lhs, cl1);
    fast_two_sum(ch, cl3)
});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::*;

    fn get_valid_pair<F : Fn(f64, f64) -> bool>(rng: F64Rand, pred: F) -> (f64, f64) {
        loop {
            let a = rng();
            let b = rng();
            if pred(a, b) { return (a, b); };
        }
    }

    randomized_test!(fast_two_sum_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { (a + b).is_finite() });
        let (hi, lo) = if a.abs() >= b.abs() { fast_two_sum(a, b) } else { fast_two_sum(b, a) };

        assert_eq_ulp!(hi, a + b, 1, "Incorrect result of fast_two_sum({}, {})", a, b);
        assert!(no_overlap(hi, lo).unwrap_or(false), "Overlapping bits in two_sum({}, {})", a, b);
    });

    randomized_test!(two_sum_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { (a + b).is_finite() });
        let (hi, lo) = two_sum(a, b);
        
        assert_eq_ulp!(hi, a + b, 1, "Incorrect result of two_sum({}, {})", a, b);
        assert!(no_overlap(hi, lo).unwrap_or(false), "Overlapping bits in two_sum({}, {})", a, b);
    });

    randomized_test!(two_diff_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { (a - b).is_finite() });
        let (hi, lo) = two_diff(a, b);

        assert_eq_ulp!(hi, a - b, 1, "Incorrect resut of two_diff({}, {})", a, b);
        assert!(no_overlap(hi, lo).unwrap_or(false), "Overlapping bits in two_diff({}, {})", a, b);
    });

    randomized_test!(two_prod_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { (a * b).is_finite() });
        let (hi, lo) = two_prod(a, b);

        assert_eq_ulp!(hi, a * b, 1, "Incorrect result of two_prod({}, {})", a, b);
        assert!(no_overlap(hi, lo).unwrap_or(false), "Overlapping bits in two_prod({}, {})", a, b);
    });

    randomized_test!(new_add_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { (a + b).is_finite() });
        let expected = two_sum(a, b);
        let actual = TwoFloat::new_add(a, b);
        assert_eq!(actual.hi, expected.0, "Incorrect result of new_add({}, {})", a, b);
        assert_eq!(actual.lo, expected.1, "Incorrect result of new_add({}, {})", a, b);
    });

    randomized_test!(new_sub_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { (a - b).is_finite() });
        let expected = two_diff(a, b);
        let actual = TwoFloat::new_sub(a, b);
        assert_eq!(actual.hi, expected.0, "Incorrect result of new_sub({}, {})", a, b);
        assert_eq!(actual.lo, expected.1, "Incorrect result of new_sub({}, {})", a, b);
    });

    randomized_test!(new_mul_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { (a * b).is_finite() });
        let expected = two_prod(a, b);
        let actual = TwoFloat::new_mul(a, b);
        assert_eq!(actual.hi, expected.0, "Incorrect result of new_mul({}, {})", a, b);
        assert_eq!(actual.lo, expected.1, "Incorrect result of new_mul({}, {})", a, b);
    });

    randomized_test!(new_div_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { (a / b).is_finite() });
        let actual = TwoFloat::new_div(a, b);
        let ef = |a: f64, b: f64| -> u64 { let ab = a.to_bits(); let bb = b.to_bits(); if ab > bb { ab - bb } else { bb - ab }};
        assert_eq_ulp!(actual.hi, a / b, 10, "Incorrect result of new_div({}, {}) - {}", a, b, ef(actual.hi, a / b));
        assert!(no_overlap(actual.hi, actual.lo).unwrap_or(false), "Overlapping bits in new_div({}, {})", a, b);
    });

    randomized_test!(neg_test, |rng: F64Rand| {
        let a = TwoFloat { hi: rng(), lo: rng() };
        let b = -a;
        assert_eq!(b.hi, -a.hi);
        assert_eq!(b.lo, -a.lo);

        let c = -b;
        assert_eq!(c.hi, a.hi);
        assert_eq!(c.lo, a.lo);
    });

    macro_rules! op_test {
        ($test_name:ident, $op:tt, $op_assign:tt, $reversible:expr) => {
            randomized_test!($test_name, |rng: F64Rand| {
                let is_reversible = $reversible;

                let c = rng();

                let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { ((a + b) $op c).is_finite() && (c $op (a + b)).is_finite() && no_overlap(a, b).unwrap_or(false) });
                let value = TwoFloat { hi: a, lo: b };

                let result1 = value $op c;
                let result2 = &value $op c;
                let mut result3 = value;
                result3 $op_assign c;
                assert_eq!(result1, result2, "Mismatch between TwoFloat {0} f64 and &TwoFloat {0} f64", stringify!($op));
                assert_eq!(result1, result3, "Mismatch between TwoFloat {} f64 and TwoFloat {} f64", stringify!($op), stringify!($op_assign));

                let result4 = c $op value;
                let result5 = c $op &value;
                if is_reversible { assert_eq!(result1, result4, "Mismatch between TwoFloat {0} f64 and f64 {0} TwoFloat", stringify!($op)); }
                assert_eq!(result4, result5, "Mismatch between f64 {0} TwoFloat and f64 {0} &TwoFloat", stringify!($op));

                let check1 = TwoFloat::from(a) $op c;
                assert_eq_ulp!(check1.hi, a $op c, 10);

                if !is_reversible {
                    let check2 = c $op TwoFloat::from(a);
                    assert_eq_ulp!(check2.hi, c $op a, 10);
                }
            });
        };
    }

    op_test!(add_f64_test, +, +=, true);
    op_test!(sub_f64_test, -, -=, false);
    op_test!(mul_f64_test, *, *=, true);
    op_test!(div_f64_test, /, /=, false);
}
