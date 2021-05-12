use core::{convert::TryFrom, num::FpCategory};

use hexf::hexf64;
use num_traits::{Inv, Pow};

use crate::{consts, TwoFloat, TwoFloatError};

impl num_traits::Num for TwoFloat {
    type FromStrRadixErr = TwoFloatError;

    #[cfg(feature = "string_convert")]
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        if radix == 10 {
            str.parse()
        } else {
            Err(TwoFloatError::ParseError)
        }
    }

    #[cfg(not(feature = "string_convert"))]
    fn from_str_radix(_str: &str, _radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Err(TwoFloatError::ParseError)
    }
}

impl num_traits::Zero for TwoFloat {
    #[inline]
    fn zero() -> Self {
        TwoFloat::default()
    }

    #[inline]
    fn is_zero(&self) -> bool {
        *self == TwoFloat::default()
    }
}

impl num_traits::One for TwoFloat {
    #[inline]
    fn one() -> Self {
        TwoFloat { hi: 1.0, lo: 0.0 }
    }
}

impl num_traits::Bounded for TwoFloat {
    #[inline]
    fn min_value() -> TwoFloat {
        TwoFloat::MIN
    }

    #[inline]
    fn max_value() -> TwoFloat {
        TwoFloat::MAX
    }
}

impl num_traits::Signed for TwoFloat {
    #[inline]
    fn abs(&self) -> Self {
        TwoFloat::abs(self)
    }

    #[inline]
    fn abs_sub(&self, other: &Self) -> Self {
        TwoFloat::abs(&(self - other))
    }

    #[inline]
    fn signum(&self) -> Self {
        TwoFloat::signum(self)
    }

    #[inline]
    fn is_positive(&self) -> bool {
        TwoFloat::is_sign_positive(self)
    }

    #[inline]
    fn is_negative(&self) -> bool {
        TwoFloat::is_sign_negative(self)
    }
}

impl num_traits::FromPrimitive for TwoFloat {
    #[inline]
    fn from_i8(n: i8) -> Option<Self> {
        Some(TwoFloat::from(n))
    }

    #[inline]
    fn from_i16(n: i16) -> Option<Self> {
        Some(TwoFloat::from(n))
    }

    #[inline]
    fn from_i32(n: i32) -> Option<Self> {
        Some(TwoFloat::from(n))
    }

    #[inline]
    fn from_i64(n: i64) -> Option<Self> {
        Some(TwoFloat::from(n))
    }

    #[inline]
    fn from_i128(n: i128) -> Option<Self> {
        Some(TwoFloat::from(n))
    }

    fn from_isize(n: isize) -> Option<Self> {
        match std::mem::size_of::<isize>() {
            1 => Self::from_i8(n as i8),
            2 => Self::from_i16(n as i16),
            4 => Self::from_i32(n as i32),
            8 => Self::from_i64(n as i64),
            16 => Self::from_i128(n as i128),
            _ => None,
        }
    }

    #[inline]
    fn from_u8(n: u8) -> Option<Self> {
        Some(TwoFloat::from(n))
    }

    #[inline]
    fn from_u16(n: u16) -> Option<Self> {
        Some(TwoFloat::from(n))
    }

    #[inline]
    fn from_u32(n: u32) -> Option<Self> {
        Some(TwoFloat::from(n))
    }

    #[inline]
    fn from_u64(n: u64) -> Option<Self> {
        Some(TwoFloat::from(n))
    }

    #[inline]
    fn from_u128(n: u128) -> Option<Self> {
        Some(TwoFloat::from(n))
    }

    fn from_usize(n: usize) -> Option<Self> {
        match std::mem::size_of::<usize>() {
            1 => Self::from_u8(n as u8),
            2 => Self::from_u16(n as u16),
            4 => Self::from_u32(n as u32),
            8 => Self::from_u64(n as u64),
            16 => Self::from_u128(n as u128),
            _ => None,
        }
    }
}

impl num_traits::ToPrimitive for TwoFloat {
    #[inline]
    fn to_i8(&self) -> Option<i8> {
        i8::try_from(self).ok()
    }

    #[inline]
    fn to_i16(&self) -> Option<i16> {
        i16::try_from(self).ok()
    }

    #[inline]
    fn to_i32(&self) -> Option<i32> {
        i32::try_from(self).ok()
    }

    #[inline]
    fn to_i64(&self) -> Option<i64> {
        i64::try_from(self).ok()
    }

    #[inline]
    fn to_i128(&self) -> Option<i128> {
        i128::try_from(self).ok()
    }

    fn to_isize(&self) -> Option<isize> {
        match std::mem::size_of::<isize>() {
            1 => self.to_i8().map(|i| i as isize),
            2 => self.to_i16().map(|i| i as isize),
            4 => self.to_i32().map(|i| i as isize),
            8 => self.to_i64().map(|i| i as isize),
            16 => self.to_i128().map(|i| i as isize),
            _ => None,
        }
    }

    #[inline]
    fn to_u8(&self) -> Option<u8> {
        u8::try_from(self).ok()
    }

    #[inline]
    fn to_u16(&self) -> Option<u16> {
        u16::try_from(self).ok()
    }

    #[inline]
    fn to_u32(&self) -> Option<u32> {
        u32::try_from(self).ok()
    }

    #[inline]
    fn to_u64(&self) -> Option<u64> {
        u64::try_from(self).ok()
    }

    #[inline]
    fn to_u128(&self) -> Option<u128> {
        u128::try_from(self).ok()
    }

    fn to_usize(&self) -> Option<usize> {
        match std::mem::size_of::<usize>() {
            1 => self.to_u8().map(|u| u as usize),
            2 => self.to_u16().map(|u| u as usize),
            4 => self.to_u32().map(|u| u as usize),
            8 => self.to_u64().map(|u| u as usize),
            16 => self.to_u128().map(|u| u as usize),
            _ => None,
        }
    }
}

impl num_traits::NumCast for TwoFloat {
    fn from<T: num_traits::ToPrimitive>(n: T) -> Option<Self> {
        const INT_THRESHOLD: f64 = hexf64!("0x1.0p53");
        if let Some(f) = n.to_f64() {
            if f.abs() <= INT_THRESHOLD {
                Some(f.into())
            } else if let Some(i) = n.to_i128() {
                Some(i.into())
            } else {
                Some(n.to_u128().map_or_else(|| f.into(), |u| u.into()))
            }
        } else if let Some(i) = n.to_i128() {
            Some(i.into())
        } else {
            n.to_u128().map(|u| u.into())
        }
    }
}

impl num_traits::FloatConst for TwoFloat {
    #[inline]
    fn E() -> Self {
        consts::E
    }

    #[inline]
    fn FRAC_1_PI() -> Self {
        consts::FRAC_1_PI
    }

    #[inline]
    fn FRAC_1_SQRT_2() -> Self {
        consts::FRAC_1_SQRT_2
    }

    #[inline]
    fn FRAC_2_PI() -> Self {
        consts::FRAC_2_PI
    }

    #[inline]
    fn FRAC_2_SQRT_PI() -> Self {
        consts::FRAC_2_SQRT_PI
    }

    #[inline]
    fn FRAC_PI_2() -> Self {
        consts::FRAC_PI_2
    }

    #[inline]
    fn FRAC_PI_3() -> Self {
        consts::FRAC_PI_3
    }

    #[inline]
    fn FRAC_PI_4() -> Self {
        consts::FRAC_PI_4
    }

    #[inline]
    fn FRAC_PI_6() -> Self {
        consts::FRAC_PI_6
    }

    #[inline]
    fn FRAC_PI_8() -> Self {
        consts::FRAC_PI_8
    }

    #[inline]
    fn LN_10() -> Self {
        consts::LN_10
    }

    #[inline]
    fn LN_2() -> Self {
        consts::LN_2
    }

    #[inline]
    fn LOG10_E() -> Self {
        consts::LOG10_E
    }

    #[inline]
    fn LOG2_E() -> Self {
        consts::LOG2_E
    }

    #[inline]
    fn PI() -> Self {
        consts::PI
    }

    #[inline]
    fn SQRT_2() -> Self {
        consts::SQRT_2
    }

    #[inline]
    fn TAU() -> Self {
        consts::TAU
    }

    #[inline]
    fn LOG10_2() -> Self {
        consts::LOG10_2
    }

    #[inline]
    fn LOG2_10() -> Self {
        consts::LOG2_10
    }
}

impl num_traits::float::FloatCore for TwoFloat {
    fn infinity() -> Self {
        TwoFloat::INFINITY
    }

    fn neg_infinity() -> Self {
        TwoFloat::NEG_INFINITY
    }

    #[inline]
    fn nan() -> Self {
        TwoFloat::NAN
    }

    #[inline]
    fn neg_zero() -> Self {
        TwoFloat { hi: -0.0, lo: 0.0 }
    }

    #[inline]
    fn min_value() -> Self {
        TwoFloat::MIN
    }

    #[inline]
    fn min_positive_value() -> Self {
        TwoFloat::MIN_POSITIVE
    }

    #[inline]
    fn epsilon() -> Self {
        TwoFloat::EPSILON
    }

    #[inline]
    fn max_value() -> Self {
        TwoFloat::MAX
    }

    #[inline]
    fn classify(self) -> FpCategory {
        self.hi.classify()
    }

    #[inline]
    fn to_degrees(self) -> Self {
        TwoFloat::to_degrees(self)
    }

    #[inline]
    fn to_radians(self) -> Self {
        TwoFloat::to_radians(self)
    }

    fn integer_decode(self) -> (u64, i16, i8) {
        panic!("cannot decode mantissa to u64")
    }

    #[inline]
    fn is_nan(self) -> bool {
        self.hi.is_nan() || self.lo.is_nan()
    }

    #[inline]
    fn is_infinite(self) -> bool {
        self.hi.is_infinite() || self.lo.is_infinite()
    }

    #[inline]
    fn is_finite(self) -> bool {
        self.is_valid()
    }

    #[inline]
    fn is_normal(self) -> bool {
        self.hi.is_normal()
    }

    #[inline]
    fn floor(self) -> Self {
        TwoFloat::floor(self)
    }

    #[inline]
    fn ceil(self) -> Self {
        TwoFloat::ceil(self)
    }

    #[inline]
    fn round(self) -> Self {
        TwoFloat::round(self)
    }

    #[inline]
    fn trunc(self) -> Self {
        TwoFloat::trunc(self)
    }

    #[inline]
    fn fract(self) -> Self {
        TwoFloat::fract(self)
    }

    #[inline]
    fn abs(self) -> Self {
        TwoFloat::abs(&self)
    }

    #[inline]
    fn signum(self) -> Self {
        TwoFloat::signum(&self)
    }

    #[inline]
    fn is_sign_positive(self) -> bool {
        TwoFloat::is_sign_positive(&self)
    }

    #[inline]
    fn is_sign_negative(self) -> bool {
        TwoFloat::is_sign_negative(&self)
    }

    #[inline]
    fn min(self, other: Self) -> Self {
        TwoFloat::min(self, other)
    }

    #[inline]
    fn max(self, other: Self) -> Self {
        TwoFloat::max(self, other)
    }

    #[inline]
    fn recip(self) -> Self {
        TwoFloat::recip(self)
    }

    #[inline]
    fn powi(self, exp: i32) -> Self {
        TwoFloat::powi(self, exp)
    }
}

#[cfg(feature = "math_funcs")]
impl num_traits::Float for TwoFloat {
    fn infinity() -> Self {
        TwoFloat::INFINITY
    }

    fn neg_infinity() -> Self {
        TwoFloat::NEG_INFINITY
    }

    #[inline]
    fn nan() -> Self {
        TwoFloat::NAN
    }

    #[inline]
    fn neg_zero() -> Self {
        TwoFloat { hi: -0.0, lo: 0.0 }
    }

    #[inline]
    fn min_value() -> Self {
        TwoFloat::MIN
    }

    #[inline]
    fn min_positive_value() -> Self {
        TwoFloat::MIN_POSITIVE
    }

    #[inline]
    fn epsilon() -> Self {
        TwoFloat::EPSILON
    }

    #[inline]
    fn max_value() -> Self {
        TwoFloat::MAX
    }

    #[inline]
    fn classify(self) -> FpCategory {
        self.hi.classify()
    }

    #[inline]
    fn to_degrees(self) -> Self {
        TwoFloat::to_degrees(self)
    }

    #[inline]
    fn to_radians(self) -> Self {
        TwoFloat::to_radians(self)
    }

    fn integer_decode(self) -> (u64, i16, i8) {
        panic!("cannot decode mantissa to u64")
    }

    #[inline]
    fn is_nan(self) -> bool {
        self.hi.is_nan() || self.lo.is_nan()
    }

    #[inline]
    fn is_infinite(self) -> bool {
        self.hi.is_infinite() || self.lo.is_infinite()
    }

    #[inline]
    fn is_finite(self) -> bool {
        self.is_valid()
    }

    #[inline]
    fn is_normal(self) -> bool {
        self.hi.is_normal()
    }

    #[inline]
    fn floor(self) -> Self {
        TwoFloat::floor(self)
    }

    #[inline]
    fn ceil(self) -> Self {
        TwoFloat::ceil(self)
    }

    #[inline]
    fn round(self) -> Self {
        TwoFloat::round(self)
    }

    #[inline]
    fn trunc(self) -> Self {
        TwoFloat::trunc(self)
    }

    #[inline]
    fn fract(self) -> Self {
        TwoFloat::fract(self)
    }

    #[inline]
    fn abs(self) -> Self {
        TwoFloat::abs(&self)
    }

    #[inline]
    fn signum(self) -> Self {
        TwoFloat::signum(&self)
    }

    #[inline]
    fn is_sign_positive(self) -> bool {
        TwoFloat::is_sign_positive(&self)
    }

    #[inline]
    fn is_sign_negative(self) -> bool {
        TwoFloat::is_sign_negative(&self)
    }

    #[inline]
    fn min(self, other: Self) -> Self {
        TwoFloat::min(self, other)
    }

    #[inline]
    fn max(self, other: Self) -> Self {
        TwoFloat::max(self, other)
    }

    #[inline]
    fn recip(self) -> Self {
        TwoFloat::recip(self)
    }

    #[inline]
    fn powi(self, exp: i32) -> Self {
        TwoFloat::powi(self, exp)
    }

    #[inline]
    fn mul_add(self, a: Self, b: Self) -> Self {
        (self * a) + b
    }

    #[inline]
    fn powf(self, n: Self) -> Self {
        TwoFloat::powf(self, n)
    }

    #[inline]
    fn sqrt(self) -> Self {
        TwoFloat::sqrt(self)
    }

    #[inline]
    fn exp(self) -> Self {
        TwoFloat::exp(self)
    }

    #[inline]
    fn exp2(self) -> Self {
        TwoFloat::exp2(self)
    }

    #[inline]
    fn ln(self) -> Self {
        TwoFloat::ln(self)
    }

    #[inline]
    fn log(self, base: Self) -> Self {
        TwoFloat::log(self, base)
    }

    #[inline]
    fn log2(self) -> Self {
        TwoFloat::log2(self)
    }

    #[inline]
    fn log10(self) -> Self {
        TwoFloat::log10(self)
    }

    #[inline]
    fn abs_sub(self, other: Self) -> Self {
        TwoFloat::abs(&(self - other))
    }

    #[inline]
    fn cbrt(self) -> Self {
        TwoFloat::cbrt(self)
    }

    #[inline]
    fn hypot(self, other: Self) -> Self {
        TwoFloat::hypot(self, other)
    }

    #[inline]
    fn sin(self) -> Self {
        TwoFloat::sin(self)
    }

    #[inline]
    fn cos(self) -> Self {
        TwoFloat::cos(self)
    }

    #[inline]
    fn tan(self) -> Self {
        TwoFloat::tan(self)
    }

    #[inline]
    fn asin(self) -> Self {
        TwoFloat::asin(self)
    }

    #[inline]
    fn acos(self) -> Self {
        TwoFloat::acos(self)
    }

    #[inline]
    fn atan(self) -> Self {
        TwoFloat::atan(self)
    }

    #[inline]
    fn atan2(self, other: Self) -> Self {
        TwoFloat::atan2(self, other)
    }

    #[inline]
    fn sin_cos(self) -> (Self, Self) {
        TwoFloat::sin_cos(self)
    }

    #[inline]
    fn exp_m1(self) -> Self {
        TwoFloat::exp_m1(self)
    }

    #[inline]
    fn ln_1p(self) -> Self {
        TwoFloat::ln_1p(self)
    }

    #[inline]
    fn sinh(self) -> Self {
        TwoFloat::sinh(self)
    }

    #[inline]
    fn cosh(self) -> Self {
        TwoFloat::cosh(self)
    }

    #[inline]
    fn tanh(self) -> Self {
        TwoFloat::tanh(self)
    }

    #[inline]
    fn asinh(self) -> Self {
        TwoFloat::asinh(self)
    }

    #[inline]
    fn acosh(self) -> Self {
        TwoFloat::acosh(self)
    }

    #[inline]
    fn atanh(self) -> Self {
        TwoFloat::atanh(self)
    }
}

unary_ops! {
    fn Inv::inv(self: &TwoFloat) -> TwoFloat {
        TwoFloat::recip(*self)
    }
}

binary_ops! {
    fn Pow::pow<'a, 'b>(self: &'a TwoFloat, rhs: &'b i8) -> TwoFloat {
        TwoFloat::powi(*self, *rhs as i32)
    }

    fn Pow::pow<'a, 'b>(self: &'a TwoFloat, rhs: &'b i16) -> TwoFloat {
        TwoFloat::powi(*self, *rhs as i32)
    }

    fn Pow::pow<'a, 'b>(self: &'a TwoFloat, rhs: &'b i32) -> TwoFloat {
        TwoFloat::powi(*self, *rhs)
    }

    fn Pow::pow<'a, 'b>(self: &'a TwoFloat, rhs: &'b u8) -> TwoFloat {
        TwoFloat::powi(*self, *rhs as i32)
    }

    fn Pow::pow<'a, 'b>(self: &'a TwoFloat, rhs: &'b u16) -> TwoFloat {
        TwoFloat::powi(*self, *rhs as i32)
    }
}

#[cfg(feature = "math_funcs")]
binary_ops! {
    fn Pow::pow<'a, 'b>(self: &'a TwoFloat, rhs: &'b f64) -> TwoFloat {
        TwoFloat::powf(*self, (*rhs).into())
    }

    fn Pow::pow<'a, 'b>(self: &'a TwoFloat, rhs: &'b TwoFloat) -> TwoFloat {
        TwoFloat::powf(*self, *rhs)
    }
}
