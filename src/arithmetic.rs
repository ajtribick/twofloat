use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

use crate::base::TwoFloat;

pub(crate) fn fast_two_sum(a: f64, b: f64) -> TwoFloat {
    // Joldes et al. (2017) Algorithm 1
    let s = a + b;
    let z = s - a;
    TwoFloat { hi: s, lo: b - z }
}

impl TwoFloat {
    /// Creates a new `TwoFloat` by adding two `f64` values using Algorithm 2
    /// from Joldes et al. (2017).
    pub fn new_add(a: f64, b: f64) -> Self {
        let s = a + b;
        let aa = s - b;
        let bb = s - aa;
        let da = a - aa;
        let db = b - bb;
        Self { hi: s, lo: da + db }
    }

    /// Creates a new `TwoFloat` by subtracting two `f64` values using
    /// Algorithm 2 from Joldes et al. (2017) modified for negative right-hand
    /// side.
    pub fn new_sub(a: f64, b: f64) -> Self {
        let s = a - b;
        let aa = s + b;
        let bb = s - aa;
        let da = a - aa;
        let db = b + bb;
        Self { hi: s, lo: da - db }
    }

    /// Creates a new `TwoFloat` by multiplying two `f64` values using
    /// Algorithm 3 from Joldes et al. (2017).
    pub fn new_mul(a: f64, b: f64) -> Self {
        let p = a * b;
        Self {
            hi: p,
            lo: a.mul_add(b, -p),
        }
    }

    /// Creates a new `TwoFloat` by dividing two `f64` values using Algorithm
    /// 15 from Joldes et al. (2017) modified for the left-hand-side having a
    /// zero value in the low word.
    pub fn new_div(a: f64, b: f64) -> Self {
        let th = a / b;
        let (ph, pl) = Self::new_mul(th, b).into();
        let dh = a - ph;
        let d = dh - pl;
        let tl = d / b;
        fast_two_sum(th, tl)
    }
}

impl Neg for TwoFloat {
    type Output = TwoFloat;

    /// Returns a new `TwoFloat` with the negated value of `self`.
    fn neg(self) -> Self::Output {
        Self::Output {
            hi: -self.hi,
            lo: -self.lo,
        }
    }
}

impl<'a> Neg for &'a TwoFloat {
    type Output = TwoFloat;

    /// Returns a new `TwoFloat` with the negated value of `self`.
    fn neg(self) -> Self::Output {
        Self::Output {
            hi: -self.hi,
            lo: -self.lo,
        }
    }
}

macro_rules! op_common_impl {
    ($op_assign:ident, $op_assign_fn:ident, $op:ident, $op_fn:ident, $lhs_i:ident, $rhs_i: ident, $rhs:ty, $code:block, $($meta:meta)*) => {
        impl $op_assign<$rhs> for TwoFloat {
            $(#[$meta])*
            fn $op_assign_fn(&mut self, $rhs_i: $rhs) {
                let $lhs_i = *self;
                let (a, b) = $code.into();
                self.hi = a;
                self.lo = b;
            }
        }

        impl $op<$rhs> for TwoFloat {
            type Output = TwoFloat;

            $(#[$meta])*
            fn $op_fn(self, $rhs_i: $rhs) -> Self::Output {
                let $lhs_i = self;
                $code
            }
        }

        impl<'a> $op<$rhs> for &'a TwoFloat {
            type Output = TwoFloat;

            $(#[$meta])*
            fn $op_fn(self, $rhs_i: $rhs) -> Self::Output {
                let $lhs_i = self;
                $code
            }
        }
    };
}

macro_rules! op_impl {
    ($op_assign:ident, $op_assign_fn:ident, $op:ident, $op_fn:ident, $(#[$meta:meta])* |$lhs_i:ident : &TwoFloat, $rhs_i:ident : &TwoFloat| $code:block) => {
        op_common_impl!($op_assign, $op_assign_fn, $op, $op_fn, $lhs_i, $rhs_i, TwoFloat, $code, $($meta)*);

        impl<'a> $op_assign<&'a TwoFloat> for TwoFloat {
            $(#[$meta])*
            fn $op_assign_fn(&mut self, $rhs_i: &'a TwoFloat) {
                let $lhs_i = *self;
                let (a, b) = $code.into();
                self.hi = a;
                self.lo = b;
            }
        }

        impl<'a> $op<&'a TwoFloat> for TwoFloat {
            type Output = TwoFloat;

            $(#[$meta])*
            fn $op_fn(self, $rhs_i: &'a TwoFloat) -> Self::Output {
                let $lhs_i = self;
                $code
            }
        }

        impl<'a, 'b> $op<&'b TwoFloat> for &'a TwoFloat {
            type Output = TwoFloat;

            $(#[$meta])*
            fn $op_fn(self, $rhs_i: &'b TwoFloat) -> Self::Output {
                let $lhs_i = self;
                $code
            }
        }
    };
    ($op_assign:ident, $op_assign_fn:ident, $op:ident, $op_fn:ident, $(#[$meta:meta])* |$lhs_i:ident : &TwoFloat, $rhs_i:ident : $rhs:ty| $code:block) => {
        op_common_impl!($op_assign, $op_assign_fn, $op, $op_fn, $lhs_i, $rhs_i, $rhs, $code, $($meta)*);

        impl $op<TwoFloat> for $rhs {
            type Output = TwoFloat;

            $(#[$meta])*
            fn $op_fn(self, mut $lhs_i: TwoFloat) -> Self::Output {
                $lhs_i.$op_assign_fn(self);
                $lhs_i
            }
        }

        impl<'a> $op<&'a TwoFloat> for $rhs {
            type Output = TwoFloat;

            $(#[$meta])*
            fn $op_fn(self, $lhs_i: &'a TwoFloat) -> Self::Output {
                let $rhs_i = self;
                $code
            }
        }
    };
    ($op_assign:ident, $op_assign_fn:ident, $op:ident, $op_fn:ident,
        $(#[$fwd:meta])* |$lhs_i:ident : &TwoFloat, $rhs_i: ident : $rhs:ty| $code:block,
        $(#[$rev:meta])* |$lhs_rev_i:ident : $lhs_rev:ty, $rhs_rev_i:ident : &TwoFloat| $code_rev:block) => {
        op_common_impl!($op_assign, $op_assign_fn, $op, $op_fn, $lhs_i, $rhs_i, $rhs, $code, $($fwd)*);

        impl $op<TwoFloat> for $lhs_rev {
            type Output = TwoFloat;

            $(#[$rev])*
            fn $op_fn(self, $rhs_rev_i: TwoFloat) -> Self::Output {
                let $lhs_rev_i = self;
                $code_rev
            }
        }

        impl<'a> $op<&'a TwoFloat> for $lhs_rev {
            type Output = TwoFloat;

            $(#[$rev])*
            fn $op_fn(self, $rhs_rev_i: &'a TwoFloat) -> Self::Output {
                let $lhs_rev_i = self;
                $code_rev
            }
        }
    };
}

op_impl!(
    AddAssign,
    add_assign,
    Add,
    add,
    /// Implements addition of `TwoFloat` and `f64` using Joldes et al.
    /// (2017) Algorithm 4.
    |lhs: &TwoFloat, rhs: f64| {
        let (sh, sl) = TwoFloat::new_add(lhs.hi, rhs).into();
        let v = lhs.lo + sl;
        fast_two_sum(sh, v)
    }
);

op_impl!(
    SubAssign,
    sub_assign,
    Sub,
    sub,
    /// Implements subtraction of `TwoFloat` and `f64` using Joldes et al.
    /// (2017) Algorithm 4 modified for negative right-hand side.
    |lhs: &TwoFloat, rhs: f64| {
        let (sh, sl) = TwoFloat::new_sub(lhs.hi, rhs).into();
        let v = lhs.lo + sl;
        fast_two_sum(sh, v)
    },
    /// Implements subtraction of `f64` and `TwoFloat` using Joldes et al.
    /// (2017) Algorithm 4 modified for negative left-hand side.
    |lhs: f64, rhs: &TwoFloat| {
        let (sh, sl) = TwoFloat::new_sub(lhs, rhs.hi).into();
        let v = sl - rhs.lo;
        fast_two_sum(sh, v)
    }
);

op_impl!(
    MulAssign,
    mul_assign,
    Mul,
    mul,
    /// Implements multiplication of `TwoFloat` and `f64` using Joldes et al.
    /// (2017) Algorithm 9.
    |lhs: &TwoFloat, rhs: f64| {
        let (ch, cl1) = TwoFloat::new_mul(lhs.hi, rhs).into();
        let cl3 = lhs.lo.mul_add(rhs, cl1);
        fast_two_sum(ch, cl3)
    }
);

op_impl!(
    DivAssign,
    div_assign,
    Div,
    div,
    /// Implements division of `TwoFloat` and `f64` using Joldes et al. (2017)
    /// Algorithm 15
    |lhs: &TwoFloat, rhs: f64| {
        let th = lhs.hi / rhs;
        let (ph, pl) = TwoFloat::new_mul(th, rhs).into();
        let dh = lhs.hi - ph;
        let dt = dh - pl;
        let d = dt + lhs.lo;
        let tl = d / rhs;
        fast_two_sum(th, tl)
    },
    /// Implements division of `f64` and `TwoFloat` using Joldes et al. (2017)
    /// Algorithm 18 modified for the left-hand side having a zero value in
    /// the low word.
    |lhs: f64, rhs: &TwoFloat| {
        let th = rhs.hi.recip();
        let rh = 1.0 - rhs.hi * th;
        let rl = -(rhs.lo * th);
        let (eh, el) = fast_two_sum(rh, rl).into();
        let e = TwoFloat { hi: eh, lo: el };
        let d = &e * th;
        let m = &d + th;
        let (ch, cl1) = TwoFloat::new_mul(m.hi, lhs).into();
        let cl3 = m.lo.mul_add(lhs, cl1);
        fast_two_sum(ch, cl3)
    }
);

op_impl!(
    RemAssign,
    rem_assign,
    Rem,
    rem,
    |lhs: &TwoFloat, rhs: f64| {
        let quotient = (lhs / rhs).trunc();
        lhs - quotient * rhs
    },
    |lhs: f64, rhs: &TwoFloat| {
        let quotient = (lhs / rhs).trunc();
        lhs - quotient * rhs
    }
);

op_impl!(
    AddAssign,
    add_assign,
    Add,
    add,
    /// Implements addition of two `TwoFloat` values using Joldes et al.
    /// (2017) Algorithm 6.
    |lhs: &TwoFloat, rhs: &TwoFloat| {
        let (sh, sl) = TwoFloat::new_add(lhs.hi, rhs.hi).into();
        let (th, tl) = TwoFloat::new_add(lhs.lo, rhs.lo).into();
        let c = sl + th;
        let (vh, vl) = fast_two_sum(sh, c).into();
        let w = tl + vl;
        fast_two_sum(vh, w)
    }
);

op_impl!(
    SubAssign,
    sub_assign,
    Sub,
    sub,
    /// Implements subtraction of two `TwoFloat` values using Joldes et al.
    /// (2017) Algorithm 6 modified for a negative right-hand side.
    |lhs: &TwoFloat, rhs: &TwoFloat| {
        let (sh, sl) = TwoFloat::new_sub(lhs.hi, rhs.hi).into();
        let (th, tl) = TwoFloat::new_sub(lhs.lo, rhs.lo).into();
        let c = sl + th;
        let (vh, vl) = fast_two_sum(sh, c).into();
        let w = tl + vl;
        fast_two_sum(vh, w)
    }
);

op_impl!(
    MulAssign,
    mul_assign,
    Mul,
    mul,
    /// Implements multiplication of two `TwoFloat` values using Joldes et al.
    /// (2017) Algorithm 12.
    |lhs: &TwoFloat, rhs: &TwoFloat| {
        let (ch, cl1) = TwoFloat::new_mul(lhs.hi, rhs.hi).into();
        let tl0 = lhs.lo * rhs.lo;
        let tl1 = lhs.hi.mul_add(rhs.lo, tl0);
        let cl2 = lhs.lo.mul_add(rhs.hi, tl1);
        let cl3 = cl1 + cl2;
        fast_two_sum(ch, cl3)
    }
);

op_impl!(
    DivAssign,
    div_assign,
    Div,
    div,
    /// Implements division of two `TwoFloat` values using Joldes et al.
    /// (2017) Algorithm 18.
    |lhs: &TwoFloat, rhs: &TwoFloat| {
        let th = rhs.hi.recip();
        let rh = 1.0 - rhs.hi * th;
        let rl = -(rhs.lo * th);
        let (eh, el) = fast_two_sum(rh, rl).into();
        let e = TwoFloat { hi: eh, lo: el };
        let d = e * th;
        let m = d + th;
        lhs * &m
    }
);

op_impl!(
    RemAssign,
    rem_assign,
    Rem,
    rem,
    |lhs: &TwoFloat, rhs: &TwoFloat| {
        let quotient = (lhs / rhs).trunc();
        lhs - &quotient * rhs
    }
);

impl TwoFloat {
    /// Calculates Euclidean division, the matching method for `rem_euclid`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(9.0);
    /// let b = TwoFloat::from(5.0);
    ///
    /// assert_eq!(a.div_euclid(b), TwoFloat::from(1.0));
    /// assert_eq!((-a).div_euclid(b), TwoFloat::from(-2.0));
    /// assert_eq!(a.div_euclid(-b), TwoFloat::from(-1.0));
    /// assert_eq!((-a).div_euclid(-b), TwoFloat::from(2.0));
    pub fn div_euclid(self, rhs: Self) -> Self {
        let quotient = (self / rhs).trunc();
        if (self - &quotient * rhs) < 0.0 {
            if rhs > 0.0 {
                quotient - 1.0
            } else {
                quotient + 1.0
            }
        } else {
            quotient
        }
    }

    /// Calculates the least nonnegative remainder of `self (mod rhs)`.
    ///
    /// The return value `r` usually satisfies `0.0 <= r < rhs.abs()`,
    /// although the errors in numerical computation may result in violations
    /// of this constraint.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(9.0);
    /// let b = TwoFloat::from(5.0);
    ///
    /// assert_eq!(a.rem_euclid(b), TwoFloat::from(4.0));
    /// assert_eq!((-a).rem_euclid(b), TwoFloat::from(1.0));
    /// assert_eq!(a.rem_euclid(-b), TwoFloat::from(4.0));
    /// assert_eq!((-a).rem_euclid(-b), TwoFloat::from(1.0));
    pub fn rem_euclid(self, rhs: Self) -> Self {
        let remainder = self % rhs;
        if remainder < 0.0 {
            remainder + rhs.abs()
        } else {
            remainder
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::*;

    randomized_test!(fast_two_sum_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |x, y| (x + y).is_finite());
        let result = if a.abs() >= b.abs() {
            fast_two_sum(a, b)
        } else {
            fast_two_sum(b, a)
        };

        assert_eq_ulp!(
            result.hi(),
            a + b,
            1,
            "Incorrect result of fast_two_sum({}, {})",
            a,
            b
        );
        assert!(
            result.is_valid(),
            "Invalid result of fast_two_sum({}, {})",
            a,
            b
        );
    });
}
