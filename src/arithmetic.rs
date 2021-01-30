use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

use crate::TwoFloat;

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
/*
impl Neg for TwoFloat {
    type Output = TwoFloat;

    /// Returns a new `TwoFloat` with the negated value of `self`.
    fn neg(self) -> Self::Output {
        -&self
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
}*/

macro_rules! op_trait_impl {
    (
        $trait:ident, $name:ident, $($ab:lifetime,)*
        $slf:ident, $lt:ty, $rhs:ident, $rt:ty,
        $ot:ty, $($meta:meta,)* $code:block
    ) => {
        impl<$($ab,)*> $trait<$rt> for $lt {
            type Output = $ot;

            $(#[$meta])*
            fn $name($slf, $rhs:$rt) -> Self::Output $code
        }
    };
    (
        $trait:ident, $name:ident, $($ab:lifetime,)*
        $slf:ident, $lt:ty, $rhs:ident, $rt:ty,
        $($meta:meta,)* $code:block
    ) => {
        impl<$($ab,)*> $trait<$rt> for $lt {
            $(#[$meta])*
            fn $name(&mut $slf, $rhs:$rt) $code
        }
    };
    (
        $trait:ident, $name:ident, $($a:lifetime,)?
        $slf:ident, $t:ty, $ot:ty,
        $($meta:meta,)* $code:block
    ) => {
        impl<$($a)?> $trait for $t {
            type Output = $ot;

            $(#[$meta])*
            fn $name($slf) -> Self::Output $code
        }
    };
}

macro_rules! binary_ops {
    (
        $(#[$meta:meta])*
        fn $trait:ident::$name:ident<$($ab:lifetime),+>(
            $slf:ident: &$a:lifetime $lt:ty, $rhs:ident: &$b:lifetime $rt:ty) -> $ot:ty
        $code:block
    ) => {
        op_trait_impl!($trait, $name, $($ab),+, $slf, &$a $lt, $rhs, &$b $rt, $ot, $($meta,)* $code);
        op_trait_impl!($trait, $name, $a, $slf, &$a $lt, $rhs, $rt, $ot, $($meta,)* { $slf.$name(&$rhs) });
        op_trait_impl!($trait, $name, $b, $slf, $lt, $rhs, &$b $rt, $ot, $($meta,)* { (&$slf).$name($rhs) });
        op_trait_impl!($trait, $name, $slf, $lt, $rhs, $rt, $ot, $($meta,)* { (&$slf).$name(&$rhs) });
    };
    (
        $(#[$meta:meta])*
        fn $trait:ident::$name:ident<$($ab:lifetime),+>(
            $slf:ident: &$a:lifetime $lt:ty, $rhs:ident: &$b:lifetime $rt:ty) -> $ot:ty
        $code:block
        $(
            $(#[$metas:meta])*
            fn $traits:ident::$names:ident<$($abs:lifetime),+>(
                $slfs:ident: &$as:lifetime $lts:ty, $rhss:ident: &$bs:lifetime $rts:ty) -> $ots:ty
            $codes:block
        )+
    ) => {
        binary_ops! {
            $(#[$meta])*
            fn $trait::$name<$($ab),+>($slf: &$a $lt, $rhs: &$b $rt) -> $ot
            $code
        }

        binary_ops! {
            $(
                $(#[$metas])*
                fn $traits::$names<$($abs),+>($slfs: &$as $lts, $rhss: &$bs $rts) -> $ots
                $codes
            )+
        }
    };
}

macro_rules! assign_ops {
    (
        $(#[$meta:meta])*
        fn $trait:ident::$name:ident<$a:lifetime>(
            $slf:ident: &mut $lt:ty, $rhs:ident: &$aa:lifetime $rt:ty) $code:block
    ) => {
        op_trait_impl!($trait, $name, $a, $slf, $lt, $rhs, &$aa $rt, $($meta,)* $code);
        op_trait_impl!($trait, $name, $a, $slf, $lt, $rhs, $rt, $($meta,)* { $slf.$name(&$rhs); });
    };
    (
        $(#[$meta:meta])*
        fn $trait:ident::$name:ident<$a:lifetime>(
            $slf:ident: &mut $lt:ty, $rhs:ident: &$aa:lifetime $rt:ty) $code:block
        $(
            $(#[$metas:meta])*
            fn $traits:ident::$names:ident<$as:lifetime>(
                $slfs:ident: &mut $lts:ty, $rhss:ident: &$aas:lifetime $rts:ty) $codes:block
        )+
    ) => {
        assign_ops! {
            $(#[$meta])*
            fn $trait::$name<$a>($slf: &mut $lt, $rhs: &$aa $rt) $code
        }

        assign_ops! {
            $(
                $(#[$metas])*
                fn $traits::$names<$as>($slfs: &mut $lts, $rhss: &$aas $rts) $codes
            )+
        }
    };
}

macro_rules! unary_ops {
    (
        $(#[$meta:meta])*
        fn $trait:ident::$name:ident($slf:ident: &$t:ty) -> $ot:ty $code:block
    ) => {
        op_trait_impl!($trait, $name, 'a, $slf, &'a $t, $ot, $($meta,)* $code);
        op_trait_impl!($trait, $name, $slf, $t, $ot, $($meta,)* { (&$slf).$name() });
    };
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
        let cl3 = self.lo.mul_add(*rhs, cl1);
        fast_two_sum(ch, cl3)
    }

    /// Implements multiplication of `TwoFloat` and `f64` using Joldes et al.
    /// (2017) Algorithm 9.
    fn Mul::mul<'a, 'b>(self: &'a f64, rhs: &'b TwoFloat) -> TwoFloat {
        let (ch, cl1) = TwoFloat::new_mul(rhs.hi, *self).into();
        let cl3 = rhs.lo.mul_add(*self, cl1);
        fast_two_sum(ch, cl3)
    }

    /// Implements multiplication of two `TwoFloat` values using Joldes et al.
    /// (2017) Algorithm 12.
    fn Mul::mul<'a, 'b>(self: &'a TwoFloat, rhs: &'b TwoFloat) -> TwoFloat {
        let (ch, cl1) = TwoFloat::new_mul(self.hi, rhs.hi).into();
        let tl0 = self.lo * rhs.lo;
        let tl1 = self.hi.mul_add(rhs.lo, tl0);
        let cl2 = self.lo.mul_add(rhs.hi, tl1);
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

    /// Implements division of `f64` and `TwoFloat` using Joldes et al. (2017)
    /// Algorithm 18 modified for the left-hand side having a zero value in
    /// the low word.
    fn Div::div<'a, 'b>(self: &'a f64, rhs: &'b TwoFloat) -> TwoFloat {
        let th = rhs.hi.recip();
        let rh = 1.0 - rhs.hi * th;
        let rl = -(rhs.lo * th);
        let (eh, el) = fast_two_sum(rh, rl).into();
        let e = TwoFloat { hi: eh, lo: el };
        let d = e * th;
        let m = d + th;
        let (ch, cl1) = TwoFloat::new_mul(m.hi, *self).into();
        let cl3 = m.lo.mul_add(*self, cl1);
        fast_two_sum(ch, cl3)
    }

    /// Implements division of two `TwoFloat` values using Joldes et al.
    /// (2017) Algorithm 18.
    fn Div::div<'a, 'b>(self: &'a TwoFloat, rhs: &'b TwoFloat) -> TwoFloat {
        let th = rhs.hi.recip();
        let rh = 1.0 - rhs.hi * th;
        let rl = -(rhs.lo * th);
        let (eh, el) = fast_two_sum(rh, rl).into();
        let e = TwoFloat { hi: eh, lo: el };
        let d = e * th;
        let m = d + th;
        self * m
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
        let cl3 = self.lo.mul_add(*rhs, cl1);
        *self = fast_two_sum(ch, cl3);
    }

    /// Implements multiplication of two `TwoFloat` values using Joldes et al.
    /// (2017) Algorithm 12.
    fn MulAssign::mul_assign<'a>(self: &mut TwoFloat, rhs: &'a TwoFloat) {
        let (ch, cl1) = TwoFloat::new_mul(self.hi, rhs.hi).into();
        let tl0 = self.lo * rhs.lo;
        let tl1 = self.hi.mul_add(rhs.lo, tl0);
        let cl2 = self.lo.mul_add(rhs.hi, tl1);
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
