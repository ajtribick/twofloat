#![allow(clippy::extra_unused_lifetimes)]

use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

use crate::TwoFloat;

// MinGW FMA seems to be inaccurate, use libm even if std is enabled.
#[cfg(all(feature = "std", not(all(windows, target_env = "gnu"))))]
#[inline(always)]
fn fma(x: f64, y: f64, z: f64) -> f64 {
    f64::mul_add(x, y, z)
}

#[cfg(not(all(feature = "std", not(all(windows, target_env = "gnu")))))]
#[inline(always)]
fn fma(x: f64, y: f64, z: f64) -> f64 {
    libm::fma(x, y, z)
}

/// Renormalization ensures that the components of the returned tuple are arranged in such a
/// way that the absolute value of the last component is no more than half the ULP of the
/// first.
#[inline]
pub fn renorm3(a: f64, b: f64, c: f64) -> TwoFloat {
    let u = fast_two_sum(a, b);
    let v = fast_two_sum(c, u.hi);
    fast_two_sum(v.hi, u.lo + v.lo)
}

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
            lo: fma(a, b, -p),
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

unary_ops! {
    fn Neg::neg(self: &TwoFloat) -> TwoFloat {
        Self::Output {
            hi: -self.hi,
            lo: -self.lo,
        }
    }
}

binary_ops! {
    /// Implements addition of `TwoFloat` and `f64` using Joldes et al.
    /// (2017) Algorithm 4.
    fn Add::add<'a, 'b>(self: &'a TwoFloat, rhs: &'b f64) -> TwoFloat {
        let (sh, sl) = TwoFloat::new_add(self.hi, *rhs).into();
        let v = self.lo + sl;
        fast_two_sum(sh, v)
    }

    /// Implements addition of `TwoFloat` and `f64` using Joldes et al.
    /// (2017) Algorithm 4.
    fn Add::add<'a, 'b>(self: &'a f64, rhs: &'b TwoFloat) -> TwoFloat {
        let (sh, sl) = TwoFloat::new_add(rhs.hi, *self).into();
        let v = rhs.lo + sl;
        fast_two_sum(sh, v)
    }

    /// Implements addition of two `TwoFloat` values using Joldes et al.
    /// (2017) Algorithm 6.
    fn Add::add<'a, 'b>(self: &'a TwoFloat, rhs: &'b TwoFloat) -> TwoFloat {
        let (sh, sl) = TwoFloat::new_add(self.hi, rhs.hi).into();
        let (th, tl) = TwoFloat::new_add(self.lo, rhs.lo).into();
        let c = sl + th;
        let (vh, vl) = fast_two_sum(sh, c).into();
        let w = tl + vl;
        fast_two_sum(vh, w)
    }

    /// Implements subtraction of `TwoFloat` and `f64` using Joldes et al.
    /// (2017) Algorithm 4 modified for negative right-hand side.
    fn Sub::sub<'a, 'b>(self: &'a TwoFloat, rhs: &'b f64) -> TwoFloat {
        let (sh, sl) = TwoFloat::new_sub(self.hi, *rhs).into();
        let v = self.lo + sl;
        fast_two_sum(sh, v)
    }

    /// Implements subtraction of `f64` and `TwoFloat` using Joldes et al.
    /// (2017) Algorithm 4 modified for negative left-hand side.
    fn Sub::sub<'a, 'b>(self: &'a f64, rhs: &'b TwoFloat) -> TwoFloat {
        let (sh, sl) = TwoFloat::new_sub(*self, rhs.hi).into();
        let v = sl - rhs.lo;
        fast_two_sum(sh, v)
    }

    /// Implements subtraction of two `TwoFloat` values using Joldes et al.
    /// (2017) Algorithm 6 modified for a negative right-hand side.
    fn Sub::sub<'a, 'b>(self: &'a TwoFloat, rhs: &'b TwoFloat) -> TwoFloat {
        let (sh, sl) = TwoFloat::new_sub(self.hi, rhs.hi).into();
        let (th, tl) = TwoFloat::new_sub(self.lo, rhs.lo).into();
        let c = sl + th;
        let (vh, vl) = fast_two_sum(sh, c).into();
        let w = tl + vl;
        fast_two_sum(vh, w)
    }

    /// Implements multiplication of `TwoFloat` and `f64` using Joldes et al.
    /// (2017) Algorithm 9.
    fn Mul::mul<'a, 'b>(self: &'a TwoFloat, rhs: &'b f64) -> TwoFloat {
        let (ch, cl1) = TwoFloat::new_mul(self.hi, *rhs).into();
        let cl3 = fma(self.lo, *rhs, cl1);
        fast_two_sum(ch, cl3)
    }

    /// Implements multiplication of `TwoFloat` and `f64` using Joldes et al.
    /// (2017) Algorithm 9.
    fn Mul::mul<'a, 'b>(self: &'a f64, rhs: &'b TwoFloat) -> TwoFloat {
        let (ch, cl1) = TwoFloat::new_mul(rhs.hi, *self).into();
        let cl3 = fma(rhs.lo, *self, cl1);
        fast_two_sum(ch, cl3)
    }

    /// Implements multiplication of two `TwoFloat` values using Joldes et al.
    /// (2017) Algorithm 12.
    fn Mul::mul<'a, 'b>(self: &'a TwoFloat, rhs: &'b TwoFloat) -> TwoFloat {
        let (ch, cl1) = TwoFloat::new_mul(self.hi, rhs.hi).into();
        let tl0 = self.lo * rhs.lo;
        let tl1 = fma(self.hi, rhs.lo, tl0);
        let cl2 = fma(self.lo, rhs.hi, tl1);
        let cl3 = cl1 + cl2;
        fast_two_sum(ch, cl3)
    }

    /// Implements division of `TwoFloat` and `f64` using Joldes et al. (2017)
    /// Algorithm 15
    fn Div::div<'a, 'b>(self: &'a TwoFloat, rhs: &'b f64) -> TwoFloat {
        let th = self.hi / rhs;
        let (ph, pl) = TwoFloat::new_mul(th, *rhs).into();
        let dh = self.hi - ph;
        let dt = dh - pl;
        let d = dt + self.lo;
        let tl = d / rhs;
        fast_two_sum(th, tl)
    }

    /// Former implements division from Joldes et al. (2017) Algorithm 18
    /// Now taken from qd crate using long division
    fn Div::div<'a, 'b>(self: &'a f64, rhs: &'b TwoFloat) -> TwoFloat {
                let q1 = self / rhs.hi;
                let mut r = self - (rhs* q1);
                let q2 = r.hi / rhs.hi;
                r -=rhs* q2;
                let q3 = r.hi / rhs.hi;
                renorm3(q1, q2, q3)
    }

    /// Former implements division from Joldes et al. (2017) Algorithm 18
    /// Now taken from qd crate using long division
    fn Div::div<'a, 'b>(self: &'a TwoFloat, rhs: &'b TwoFloat) -> TwoFloat {
                let q1 = self.hi / rhs.hi;
                let mut r = self - (rhs* q1);
                let q2 = r.hi / rhs.hi;
                r -=rhs* q2;
                let q3 = r.hi / rhs.hi;
                renorm3(q1, q2, q3)
    }

    fn Rem::rem<'a, 'b>(self: &'a TwoFloat, rhs: &'b f64) -> TwoFloat {
        let quotient = (self / rhs).trunc();
        self - quotient * rhs
    }

    fn Rem::rem<'a, 'b>(self: &'a f64, rhs: &'b TwoFloat) -> TwoFloat {
        let quotient = (self / rhs).trunc();
        self - quotient * rhs
    }

    fn Rem::rem<'a, 'b>(self: &'a TwoFloat, rhs: &'b TwoFloat) -> TwoFloat {
        let quotient = (self / rhs).trunc();
        self - quotient * rhs
    }
}

// Self-assignment operators

assign_ops! {
    /// Implements addition of `TwoFloat` and `f64` using Joldes et al.
    /// (2017) Algorithm 4.
    fn AddAssign::add_assign<'a>(self: &mut TwoFloat, rhs: &'a f64) {
        let (sh, sl) = TwoFloat::new_add(self.hi, *rhs).into();
        let v = self.lo + sl;
        *self = fast_two_sum(sh, v);
    }

    /// Implements addition of two `TwoFloat` values using Joldes et al.
    /// (2017) Algorithm 6.
    fn AddAssign::add_assign<'a>(self: &mut TwoFloat, rhs: &'a TwoFloat) {
        let (sh, sl) = TwoFloat::new_add(self.hi, rhs.hi).into();
        let (th, tl) = TwoFloat::new_add(self.lo, rhs.lo).into();
        let c = sl + th;
        let (vh, vl) = fast_two_sum(sh, c).into();
        let w = tl + vl;
        *self = fast_two_sum(vh, w)
    }

    /// Implements subtraction of `TwoFloat` and `f64` using Joldes et al.
    /// (2017) Algorithm 4 modified for negative right-hand side.
    fn SubAssign::sub_assign<'a>(self: &mut TwoFloat, rhs: &'a f64) {
        let (sh, sl) = TwoFloat::new_sub(self.hi, *rhs).into();
        let v = self.lo + sl;
        *self = fast_two_sum(sh, v);
    }

    /// Implements subtraction of two `TwoFloat` values using Joldes et al.
    /// (2017) Algorithm 6 modified for a negative right-hand side.
    fn SubAssign::sub_assign<'a>(self: &mut TwoFloat, rhs: &'a TwoFloat) {
        let (sh, sl) = TwoFloat::new_sub(self.hi, rhs.hi).into();
        let (th, tl) = TwoFloat::new_sub(self.lo, rhs.lo).into();
        let c = sl + th;
        let (vh, vl) = fast_two_sum(sh, c).into();
        let w = tl + vl;
        *self = fast_two_sum(vh, w)
    }

    /// Implements multiplication of `TwoFloat` and `f64` using Joldes et al.
    /// (2017) Algorithm 9.
    fn MulAssign::mul_assign<'a>(self: &mut TwoFloat, rhs: &'a f64) {
        let (ch, cl1) = TwoFloat::new_mul(self.hi, *rhs).into();
        let cl3 = fma(self.lo, *rhs, cl1);
        *self = fast_two_sum(ch, cl3);
    }

    /// Implements multiplication of two `TwoFloat` values using Joldes et al.
    /// (2017) Algorithm 12.
    fn MulAssign::mul_assign<'a>(self: &mut TwoFloat, rhs: &'a TwoFloat) {
        let (ch, cl1) = TwoFloat::new_mul(self.hi, rhs.hi).into();
        let tl0 = self.lo * rhs.lo;
        let tl1 = fma(self.hi, rhs.lo, tl0);
        let cl2 = fma(self.lo, rhs.hi, tl1);
        let cl3 = cl1 + cl2;
        *self = fast_two_sum(ch, cl3)
    }

    /// Implements division of `TwoFloat` and `f64` using Joldes et al. (2017)
    /// Algorithm 15
    fn DivAssign::div_assign<'a>(self: &mut TwoFloat, rhs: &'a f64) {
        let th = self.hi / rhs;
        let (ph, pl) = TwoFloat::new_mul(th, *rhs).into();
        let dh = self.hi - ph;
        let dt = dh - pl;
        let d = dt + self.lo;
        let tl = d / rhs;
        *self = fast_two_sum(th, tl)
    }

    /// Implements division of two `TwoFloat` values using Joldes et al.
    /// (2017) Algorithm 18.
    fn DivAssign::div_assign<'a>(self: &mut TwoFloat, rhs: &'a TwoFloat) {
        let th = rhs.hi.recip();
        let rh = 1.0 - rhs.hi * th;
        let rl = -(rhs.lo * th);
        let (eh, el) = fast_two_sum(rh, rl).into();
        let e = TwoFloat { hi: eh, lo: el };
        let d = e * th;
        let m = d + th;
        *self *= m;
    }

    fn RemAssign::rem_assign<'b>(self: &mut TwoFloat, rhs: &'b f64) {
        let quotient = (*self / rhs).trunc();
        *self -= quotient * rhs;
    }

    fn RemAssign::rem_assign<'a>(self: &mut TwoFloat, rhs: &'a TwoFloat) {
        let quotient = (*self / rhs).trunc();
        *self -= quotient * rhs;
    }
}

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
    /// ```
    pub fn div_euclid(self, rhs: Self) -> Self {
        let quotient = (self / rhs).trunc();
        if (self - quotient * rhs) < 0.0 {
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
    /// ```
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
    use super::fast_two_sum;
    use crate::test_util::{get_valid_pair, repeated_test};

    #[test]
    fn fast_two_sum_test() {
        repeated_test(|| {
            let (a, b) = get_valid_pair(|x, y| (x + y).is_finite());
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
}
