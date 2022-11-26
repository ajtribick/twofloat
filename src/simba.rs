use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use num_traits::{Float, Signed, Zero};
use simba::{
    scalar::{ComplexField, Field, RealField, SubsetOf},
    simd::SimdValue,
};

use crate::TwoFloat;

impl SimdValue for TwoFloat {
    type Element = Self;

    type SimdBool = bool;

    #[inline(always)]
    fn lanes() -> usize {
        1
    }

    fn splat(val: Self::Element) -> Self {
        val
    }

    fn extract(&self, _: usize) -> Self::Element {
        *self
    }

    unsafe fn extract_unchecked(&self, _: usize) -> Self::Element {
        *self
    }

    fn replace(&mut self, _: usize, val: Self::Element) {
        *self = val
    }

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

    fn from_real(re: Self::RealField) -> Self {
        re
    }

    fn real(self) -> Self::RealField {
        self
    }

    fn imaginary(self) -> Self::RealField {
        0.0.into()
    }

    fn modulus(self) -> Self::RealField {
        Self::abs(&self)
    }

    fn modulus_squared(self) -> Self::RealField {
        self * self
    }

    fn argument(self) -> Self::RealField {
        if self >= Self::zero() {
            Self::zero()
        } else {
            Self::pi()
        }
    }

    fn norm1(self) -> Self::RealField {
        Self::abs(&self)
    }

    fn scale(self, factor: Self::RealField) -> Self {
        self * factor
    }

    fn unscale(self, factor: Self::RealField) -> Self {
        self / factor
    }

    fn floor(self) -> Self {
        self.floor()
    }

    fn ceil(self) -> Self {
        self.ceil()
    }

    fn round(self) -> Self {
        self.round()
    }

    fn trunc(self) -> Self {
        self.trunc()
    }

    fn fract(self) -> Self {
        self.fract()
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        num_traits::real::Real::mul_add(self, a, b)
    }

    fn abs(self) -> Self::RealField {
        Self::abs(&self)
    }

    fn hypot(self, other: Self) -> Self::RealField {
        self.hypot(other)
    }

    fn recip(self) -> Self {
        self.recip()
    }

    fn conjugate(self) -> Self {
        self
    }

    fn sin(self) -> Self {
        self.sin()
    }

    fn cos(self) -> Self {
        self.cos()
    }

    fn sin_cos(self) -> (Self, Self) {
        self.sin_cos()
    }

    fn tan(self) -> Self {
        self.tan()
    }

    fn asin(self) -> Self {
        self.asin()
    }

    fn acos(self) -> Self {
        self.acos()
    }

    fn atan(self) -> Self {
        self.atan()
    }

    fn sinh(self) -> Self {
        self.sinh()
    }

    fn cosh(self) -> Self {
        self.cosh()
    }

    fn tanh(self) -> Self {
        self.tanh()
    }

    fn asinh(self) -> Self {
        self.asinh()
    }

    fn acosh(self) -> Self {
        self.acosh()
    }

    fn atanh(self) -> Self {
        self.atanh()
    }

    fn log(self, base: Self::RealField) -> Self {
        self.log(base)
    }

    fn log2(self) -> Self {
        self.log2()
    }

    fn log10(self) -> Self {
        self.log10()
    }

    fn ln(self) -> Self {
        self.ln()
    }

    fn ln_1p(self) -> Self {
        self.ln_1p()
    }

    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn exp(self) -> Self {
        self.exp()
    }

    fn exp2(self) -> Self {
        self.exp2()
    }

    fn exp_m1(self) -> Self {
        self.exp_m1()
    }

    fn powi(self, n: i32) -> Self {
        self.powi(n)
    }

    fn powf(self, n: Self::RealField) -> Self {
        self.powf(n)
    }

    fn powc(self, n: Self) -> Self {
        self.powf(n)
    }

    fn cbrt(self) -> Self {
        todo!()
    }

    fn is_finite(&self) -> bool {
        num_traits::Float::is_finite(*self)
    }

    fn try_sqrt(self) -> Option<Self> {
        Some(self.sqrt())
    }
}

impl AbsDiffEq for TwoFloat {
    type Epsilon = Self;

    fn default_epsilon() -> Self::Epsilon {
        Self::EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        num_traits::Float::abs(self - other) <= epsilon
    }
}

impl UlpsEq for TwoFloat {
    fn default_max_ulps() -> u32 {
        4
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        if self.abs_diff_eq(other, epsilon) {
            return true;
        }
        if self.signum() != other.signum() {
            return false;
        }

        self.hi().ulps_eq(&other.hi(), epsilon.into(), max_ulps)
            && self.lo().ulps_eq(&other.lo(), epsilon.into(), max_ulps)
    }
}

impl RelativeEq for TwoFloat {
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
    fn is_sign_positive(&self) -> bool {
        self.is_positive()
    }

    fn is_sign_negative(&self) -> bool {
        self.is_negative()
    }

    fn copysign(self, sign: Self) -> Self {
        if sign.is_positive() {
            TwoFloat::abs(&self)
        } else {
            -TwoFloat::abs(&self)
        }
    }

    fn max(self, other: Self) -> Self {
        self.max(other)
    }

    fn min(self, other: Self) -> Self {
        self.min(other)
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        self.min(min).max(max)
    }

    fn atan2(self, other: Self) -> Self {
        self.atan2(other)
    }

    fn min_value() -> Option<Self> {
        Some(Self::MIN)
    }

    fn max_value() -> Option<Self> {
        Some(Self::MAX)
    }

    fn pi() -> Self {
        f64::pi().into()
    }

    fn two_pi() -> Self {
        Self::from(f64::two_pi())
    }

    fn frac_pi_2() -> Self {
        Self::from(f64::frac_pi_2())
    }

    fn frac_pi_3() -> Self {
        Self::from(f64::frac_pi_3())
    }

    fn frac_pi_4() -> Self {
        Self::from(f64::frac_pi_4())
    }

    fn frac_pi_6() -> Self {
        Self::from(f64::frac_pi_6())
    }

    fn frac_pi_8() -> Self {
        Self::from(f64::frac_pi_8())
    }

    fn frac_1_pi() -> Self {
        Self::from(f64::frac_1_pi())
    }

    fn frac_2_pi() -> Self {
        Self::from(f64::frac_2_pi())
    }

    fn frac_2_sqrt_pi() -> Self {
        Self::from(f64::frac_2_sqrt_pi())
    }

    fn e() -> Self {
        Self::from(f64::e())
    }

    fn log2_e() -> Self {
        Self::from(f64::log2_e())
    }

    fn log10_e() -> Self {
        Self::from(f64::log10_e())
    }

    fn ln_2() -> Self {
        Self::from(f64::ln_2())
    }

    fn ln_10() -> Self {
        Self::from(f64::ln_10())
    }
}

impl SubsetOf<f64> for TwoFloat {
    fn to_superset(&self) -> f64 {
        self.into()
    }

    fn from_superset_unchecked(element: &f64) -> Self {
        (*element).into()
    }

    fn is_in_subset(_: &f64) -> bool {
        true
    }
}

impl SubsetOf<f32> for TwoFloat {
    fn to_superset(&self) -> f32 {
        self.into()
    }

    fn from_superset_unchecked(element: &f32) -> Self {
        (*element).into()
    }

    fn is_in_subset(_: &f32) -> bool {
        true
    }
}

impl SubsetOf<Self> for TwoFloat {
    fn to_superset(&self) -> Self {
        *self
    }

    fn from_superset_unchecked(element: &Self) -> Self {
        *element
    }

    fn is_in_subset(_: &Self) -> bool {
        true
    }
}

impl SubsetOf<TwoFloat> for f64 {
    fn to_superset(&self) -> TwoFloat {
        (*self).into()
    }

    fn from_superset_unchecked(element: &TwoFloat) -> Self {
        (*element).into()
    }

    fn is_in_subset(_: &TwoFloat) -> bool {
        true
    }
}
