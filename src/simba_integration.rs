use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use num_traits::{Float, Signed, Zero};
use simba::{
    scalar::{ComplexField, Field, RealField, SubsetOf},
    simd::SimdValue,
};

use crate::TwoFloat;

impl SimdValue for TwoFloat {
    const LANES: usize = 1;

    type Element = Self;
    type SimdBool = bool;

    #[inline]
    fn splat(val: Self::Element) -> Self {
        val
    }

    fn extract(&self, i: usize) -> Self::Element {
        if i >= Self::LANES {
            panic!("Out of bounds");
        }

        *self
    }

    #[inline]
    unsafe fn extract_unchecked(&self, _: usize) -> Self::Element {
        *self
    }

    fn replace(&mut self, i: usize, val: Self::Element) {
        if i >= Self::LANES {
            panic!("Out of bounds");
        }

        *self = val
    }

    #[inline]
    unsafe fn replace_unchecked(&mut self, _: usize, val: Self::Element) {
        *self = val
    }

    fn select(self, cond: Self::SimdBool, other: Self) -> Self {
        if cond {
            self
        } else {
            other
        }
    }
}

impl Field for TwoFloat {}

impl ComplexField for TwoFloat {
    type RealField = TwoFloat;

    #[inline]
    fn from_real(re: Self::RealField) -> Self {
        re
    }

    #[inline]
    fn real(self) -> Self::RealField {
        self
    }

    #[inline]
    fn imaginary(self) -> Self::RealField {
        Default::default()
    }

    #[inline]
    fn modulus(self) -> Self::RealField {
        Self::abs(&self)
    }

    #[inline]
    fn modulus_squared(self) -> Self::RealField {
        self * self
    }

    fn argument(self) -> Self::RealField {
        if self >= Self::zero() {
            Default::default()
        } else {
            crate::consts::PI
        }
    }

    #[inline]
    fn norm1(self) -> Self::RealField {
        Self::abs(&self)
    }

    #[inline]
    fn scale(self, factor: Self::RealField) -> Self {
        self * factor
    }

    #[inline]
    fn unscale(self, factor: Self::RealField) -> Self {
        self / factor
    }

    #[inline]
    fn floor(self) -> Self {
        self.floor()
    }

    #[inline]
    fn ceil(self) -> Self {
        self.ceil()
    }

    #[inline]
    fn round(self) -> Self {
        self.round()
    }

    #[inline]
    fn trunc(self) -> Self {
        self.trunc()
    }

    #[inline]
    fn fract(self) -> Self {
        self.fract()
    }

    #[inline]
    fn mul_add(self, a: Self, b: Self) -> Self {
        self * a + b
    }

    #[inline]
    fn abs(self) -> Self::RealField {
        Self::abs(&self)
    }

    #[inline]
    fn hypot(self, other: Self) -> Self::RealField {
        self.hypot(other)
    }

    #[inline]
    fn recip(self) -> Self {
        self.recip()
    }

    #[inline]
    fn conjugate(self) -> Self {
        self
    }

    #[inline]
    fn sin(self) -> Self {
        self.sin()
    }

    #[inline]
    fn cos(self) -> Self {
        self.cos()
    }

    #[inline]
    fn sin_cos(self) -> (Self, Self) {
        self.sin_cos()
    }

    #[inline]
    fn tan(self) -> Self {
        self.tan()
    }

    #[inline]
    fn asin(self) -> Self {
        self.asin()
    }

    #[inline]
    fn acos(self) -> Self {
        self.acos()
    }

    #[inline]
    fn atan(self) -> Self {
        self.atan()
    }

    #[inline]
    fn sinh(self) -> Self {
        self.sinh()
    }

    #[inline]
    fn cosh(self) -> Self {
        self.cosh()
    }

    #[inline]
    fn tanh(self) -> Self {
        self.tanh()
    }

    #[inline]
    fn asinh(self) -> Self {
        self.asinh()
    }

    #[inline]
    fn acosh(self) -> Self {
        self.acosh()
    }

    #[inline]
    fn atanh(self) -> Self {
        self.atanh()
    }

    #[inline]
    fn log(self, base: Self::RealField) -> Self {
        self.log(base)
    }

    #[inline]
    fn log2(self) -> Self {
        self.log2()
    }

    #[inline]
    fn log10(self) -> Self {
        self.log10()
    }

    #[inline]
    fn ln(self) -> Self {
        self.ln()
    }

    #[inline]
    fn ln_1p(self) -> Self {
        self.ln_1p()
    }

    #[inline]
    fn sqrt(self) -> Self {
        self.sqrt()
    }

    #[inline]
    fn exp(self) -> Self {
        self.exp()
    }

    #[inline]
    fn exp2(self) -> Self {
        self.exp2()
    }

    #[inline]
    fn exp_m1(self) -> Self {
        self.exp_m1()
    }

    #[inline]
    fn powi(self, n: i32) -> Self {
        self.powi(n)
    }

    #[inline]
    fn powf(self, n: Self::RealField) -> Self {
        self.powf(n)
    }

    #[inline]
    fn powc(self, n: Self) -> Self {
        self.powf(n)
    }

    #[inline]
    fn cbrt(self) -> Self {
        self.cbrt()
    }

    #[inline]
    fn is_finite(&self) -> bool {
        num_traits::Float::is_finite(*self)
    }

    #[inline]
    fn try_sqrt(self) -> Option<Self> {
        Some(self.sqrt())
    }
}

impl AbsDiffEq for TwoFloat {
    type Epsilon = Self;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        Self::EPSILON
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        TwoFloat::abs(&(self - other)) <= epsilon
    }
}

impl UlpsEq for TwoFloat {
    fn default_max_ulps() -> u32 {
        f64::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        if self.abs_diff_eq(other, epsilon) {
            return true;
        }

        if self.signum() != other.signum() {
            return false;
        }

        // Incomplete - fix this before merging
        self.hi == other.hi && self.lo.ulps_eq(&other.lo, epsilon.into(), max_ulps)
    }
}

impl RelativeEq for TwoFloat {
    #[inline]
    fn default_max_relative() -> Self::Epsilon {
        Self::EPSILON
    }

    fn relative_eq(&self, other: &TwoFloat, epsilon: TwoFloat, max_relative: TwoFloat) -> bool {
        // Handle same infinities
        if self == other {
            return true;
        }

        // Handle remaining infinities
        if TwoFloat::is_infinite(*self) || TwoFloat::is_infinite(*other) {
            return false;
        }

        let abs_diff = TwoFloat::abs(&(self - other));

        // For when the numbers are really close together
        if abs_diff <= epsilon {
            return true;
        }

        let abs_self = TwoFloat::abs(self);
        let abs_other = TwoFloat::abs(other);

        let largest = if abs_other > abs_self {
            abs_other
        } else {
            abs_self
        };

        // Use a relative difference comparison
        abs_diff <= largest * max_relative
    }
}

impl RealField for TwoFloat {
    #[inline]
    fn is_sign_positive(&self) -> bool {
        self.is_positive()
    }

    #[inline]
    fn is_sign_negative(&self) -> bool {
        self.is_negative()
    }

    #[inline]
    fn copysign(self, sign: Self) -> Self {
        if sign.is_positive() {
            TwoFloat::abs(&self)
        } else {
            -TwoFloat::abs(&self)
        }
    }

    #[inline]
    fn max(self, other: Self) -> Self {
        self.max(other)
    }

    #[inline]
    fn min(self, other: Self) -> Self {
        self.min(other)
    }

    #[inline]
    fn clamp(self, min: Self, max: Self) -> Self {
        self.min(min).max(max)
    }

    #[inline]
    fn atan2(self, other: Self) -> Self {
        self.atan2(other)
    }

    #[inline]
    fn min_value() -> Option<Self> {
        Some(Self::MIN)
    }

    #[inline]
    fn max_value() -> Option<Self> {
        Some(Self::MAX)
    }

    #[inline]
    fn pi() -> Self {
        crate::consts::PI
    }

    #[inline]
    fn two_pi() -> Self {
        crate::consts::TAU
    }

    #[inline]
    fn frac_pi_2() -> Self {
        crate::consts::FRAC_PI_2
    }

    #[inline]
    fn frac_pi_3() -> Self {
        crate::consts::FRAC_PI_3
    }

    #[inline]
    fn frac_pi_4() -> Self {
        crate::consts::FRAC_PI_4
    }

    #[inline]
    fn frac_pi_6() -> Self {
        crate::consts::FRAC_PI_6
    }

    #[inline]
    fn frac_pi_8() -> Self {
        crate::consts::FRAC_PI_8
    }

    #[inline]
    fn frac_1_pi() -> Self {
        crate::consts::FRAC_1_PI
    }

    #[inline]
    fn frac_2_pi() -> Self {
        crate::consts::FRAC_2_PI
    }

    #[inline]
    fn frac_2_sqrt_pi() -> Self {
        crate::consts::FRAC_2_SQRT_PI
    }

    #[inline]
    fn e() -> Self {
        crate::consts::E
    }

    #[inline]
    fn log2_e() -> Self {
        crate::consts::LOG2_E
    }

    #[inline]
    fn log10_e() -> Self {
        crate::consts::LOG10_E
    }

    #[inline]
    fn ln_2() -> Self {
        crate::consts::LN_2
    }

    #[inline]
    fn ln_10() -> Self {
        crate::consts::LN_10
    }
}

impl SubsetOf<f64> for TwoFloat {
    #[inline]
    fn to_superset(&self) -> f64 {
        self.into()
    }

    #[inline]
    fn from_superset_unchecked(element: &f64) -> Self {
        (*element).into()
    }

    #[inline]
    fn is_in_subset(_: &f64) -> bool {
        true
    }
}

impl SubsetOf<f32> for TwoFloat {
    #[inline]
    fn to_superset(&self) -> f32 {
        self.into()
    }

    #[inline]
    fn from_superset_unchecked(element: &f32) -> Self {
        (*element).into()
    }

    #[inline]
    fn is_in_subset(_: &f32) -> bool {
        true
    }
}

impl SubsetOf<Self> for TwoFloat {
    #[inline]
    fn to_superset(&self) -> Self {
        *self
    }

    #[inline]
    fn from_superset_unchecked(element: &Self) -> Self {
        *element
    }

    #[inline]
    fn is_in_subset(_: &Self) -> bool {
        true
    }
}

impl SubsetOf<TwoFloat> for f64 {
    #[inline]
    fn to_superset(&self) -> TwoFloat {
        (*self).into()
    }

    #[inline]
    fn from_superset_unchecked(element: &TwoFloat) -> Self {
        element.into()
    }

    #[inline]
    fn is_in_subset(_: &TwoFloat) -> bool {
        true
    }
}

impl SubsetOf<TwoFloat> for f32 {
    #[inline]
    fn to_superset(&self) -> TwoFloat {
        (*self).into()
    }

    #[inline]
    fn from_superset_unchecked(element: &TwoFloat) -> Self {
        element.into()
    }

    #[inline]
    fn is_in_subset(_: &TwoFloat) -> bool {
        true
    }
}
