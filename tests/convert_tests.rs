#![allow(clippy::float_cmp)]

use core::{convert::TryFrom, fmt::Debug, mem::discriminant, ops::Range};

use num_traits::{one, zero, ToPrimitive};
use rand::{distributions::uniform::SampleUniform, Rng};

use twofloat::{no_overlap, TwoFloat, TwoFloatError};

#[macro_use]
pub mod common;

use common::*;

fn right_bit(f: f64) -> Option<i16> {
    let fbits = f.to_bits();
    let exponent = ((fbits >> 52) & 0x7ff) as i16 - 1023;
    match exponent {
        -1023 => {
            let mantissa = fbits & ((1 << 52) - 1);
            if mantissa == 0 {
                Some(i16::MIN)
            } else {
                Some(-1074)
            }
        }
        1024 => None,
        _ => Some(exponent - 52),
    }
}

#[test]
fn try_from_tuple_no_overlap_test() {
    repeated_test(|| {
        let (a, b) = get_valid_pair(no_overlap);
        let result = TwoFloat::try_from((a, b));
        assert!(
            result.is_ok(),
            "Creation from non-overlapping pair {}, {} resulted in error",
            a,
            b
        );
        let unwrapped = result.unwrap();
        assert_eq!(
            (unwrapped.hi(), unwrapped.lo()),
            (a, b),
            "Value mismatch in creation of TwoFloat"
        );
    });
}

#[test]
fn try_from_tuple_overlap_test() {
    repeated_test(|| {
        let (a, b) = get_valid_pair(|x, y| !no_overlap(x, y));
        let result = TwoFloat::try_from((a, b));
        assert!(
            result.is_err(),
            "Creation from overlapping pair {}, {} resulted in value",
            a,
            b
        );
    });
}

#[test]
fn try_from_array_no_overlap_test() {
    repeated_test(|| {
        let (a, b) = get_valid_pair(no_overlap);
        let result = TwoFloat::try_from([a, b]);
        assert!(
            result.is_ok(),
            "Creation from non-overlapping pair {}, {} resulted in error",
            a,
            b
        );
        let unwrapped = result.unwrap();
        assert_eq!(
            (unwrapped.hi(), unwrapped.lo()),
            (a, b),
            "Value mismatch in creation of TwoFloat"
        );
    })
}

#[test]
fn try_from_array_overlap_test() {
    repeated_test(|| {
        let (a, b) = get_valid_pair(|x, y| !no_overlap(x, y));
        let result = TwoFloat::try_from([a, b]);
        assert!(
            result.is_err(),
            "Creation from overlapping pair {}, {} resulted in value",
            a,
            b
        );
    });
}

fn from_float<F>()
where
    F: num_traits::float::FloatCore + Into<TwoFloat>,
{
    repeated_test(|| {
        let source = loop {
            let source = F::from(random_float()).unwrap();
            if source.is_finite() {
                break source;
            };
        };

        let result: TwoFloat = source.into();

        assert_eq!(
            result.hi(),
            source.to_f64().unwrap(),
            "Float conversion failed: mismatch in high word"
        );
        assert_eq!(
            result.lo(),
            0.0,
            "Float conversion failed: non-zero low word"
        );
    });
}

fn into_float<F>()
where
    F: PartialEq<F> + num_traits::FromPrimitive + From<TwoFloat> + Debug,
{
    repeated_test(|| {
        let source = get_twofloat();

        let result: F = source.into();
        assert_eq!(
            result,
            F::from_f64(source.hi()).unwrap(),
            "Float conversion from TwoFloat failed"
        );
    });
}

#[test]
fn from_f32_test() {
    from_float::<f32>();
}

#[test]
fn into_f32_test() {
    into_float::<f32>();
}

#[test]
fn from_f64_test() {
    from_float::<f64>();
}

#[test]
fn into_f64_test() {
    into_float::<f64>();
}

#[test]
fn to_f64_test() {
    repeated_test(|| {
        let source = get_twofloat();
        let result = source.to_f64().unwrap();
        assert_eq!(result, source.hi(), "Float conversion from TwoFloat failed");
    });
}

fn check_try_from_result<T>(
    expected: &Result<T, TwoFloatError>,
    result: &Result<T, TwoFloatError>,
    source: TwoFloat,
) where
    T: core::fmt::Debug + PartialEq,
{
    assert_eq!(
        discriminant(expected),
        discriminant(result),
        "Conversion of {source:?} produced unexpected Err/Ok state"
    );
    match (expected, result) {
        (Ok(expected_value), Ok(result_value)) => assert_eq!(
            expected_value, result_value,
            "Conversion of {source:?} produced incorrect result"
        ),
        (Err(expected_err), Err(result_err)) => assert_eq!(
            discriminant(expected_err),
            discriminant(result_err),
            "Conversion of {source:?} produced mismatched error types"
        ),
        _ => unreachable!(),
    }
}

// Helper trait for integer conversions

trait ConvertBounds:
    num_traits::PrimInt
    + num_traits::NumAssign
    + TryFrom<TwoFloat, Error = TwoFloatError>
    + Into<TwoFloat>
    + Debug
    + SampleUniform
{
    fn lower_bound() -> f64 {
        Self::min_value().to_f64().unwrap() - 1.0
    }

    fn upper_bound() -> f64 {
        Self::max_value().to_f64().unwrap() + 1.0
    }
}

impl ConvertBounds for i8 {}
impl ConvertBounds for i16 {}
impl ConvertBounds for i32 {}

impl ConvertBounds for i64 {
    fn upper_bound() -> f64 {
        i64::MAX as f64
    }
}

impl ConvertBounds for i128 {
    fn upper_bound() -> f64 {
        i128::MAX as f64
    }
}

impl ConvertBounds for u8 {}
impl ConvertBounds for u16 {}
impl ConvertBounds for u32 {}

impl ConvertBounds for u64 {
    fn upper_bound() -> f64 {
        u64::MAX as f64
    }
}

impl ConvertBounds for u128 {
    fn upper_bound() -> f64 {
        u128::MAX as f64
    }
}

// Tests for conversions of integers up to 32 bits

fn from_twofloat_lower_bound<T>()
where
    T: ConvertBounds,
{
    repeated_test(|| {
        let source = loop {
            if let Ok(source) = try_get_twofloat_with_hi(T::lower_bound()) {
                break source;
            }
        };

        let expected = if source.lo() > 0.0 {
            Ok(T::min_value())
        } else {
            Err(TwoFloatError::ConversionError {})
        };
        let result = T::try_from(source);

        check_try_from_result(&expected, &result, source);
    });
}

fn from_twofloat_upper_bound<T>()
where
    T: ConvertBounds,
{
    repeated_test(|| {
        let source = loop {
            if let Ok(source) = try_get_twofloat_with_hi(T::upper_bound()) {
                break source;
            }
        };

        let expected = if source.lo() < 0.0 {
            Ok(T::max_value())
        } else {
            Err(TwoFloatError::ConversionError {})
        };

        let result = T::try_from(source);
        check_try_from_result(&expected, &result, source);
    })
}

fn from_twofloat_split_fract<T>()
where
    T: ConvertBounds,
{
    let mut rng = rand::thread_rng();
    let valid_dist = rand::distributions::Uniform::new(
        f64::from_bits(T::lower_bound().to_bits() - 1),
        T::upper_bound(),
    );

    repeated_test_enumerate(|i| {
        let (a, b, source) = loop {
            let a = rng.sample(valid_dist).trunc();
            let b = if i == 0 { 0.0 } else { random_float() };
            if let Ok(source) = TwoFloat::try_from((a, b)) {
                break (a, b, source);
            }
        };

        let expected = if a < 0.0 && b > 0.0 {
            Ok(T::from(a).unwrap() + one())
        } else if a > 0.0 && b < 0.0 {
            Ok(T::from(a).unwrap() - one())
        } else {
            Ok(T::from(a).unwrap())
        };

        let result = T::try_from(source);
        check_try_from_result(&expected, &result, source);
    });
}

fn from_twofloat_with_fract<T>()
where
    T: ConvertBounds,
{
    let mut rng = rand::thread_rng();
    let valid_dist = rand::distributions::Uniform::new(
        f64::from_bits(T::lower_bound().to_bits() - 1),
        T::upper_bound(),
    );

    repeated_test_enumerate(|i| {
        let (a, source) = loop {
            let a = rng.sample(valid_dist);
            if a.fract() == 0.0 {
                continue;
            }
            let b = if i == 0 { 0.0 } else { random_float() };
            if let Ok(source) = TwoFloat::try_from((a, b)) {
                break (a, source);
            }
        };

        let expected = Ok(T::from(a.trunc()).unwrap());

        let result = T::try_from(source);
        check_try_from_result(&expected, &result, source);
    });
}

fn from_twofloat_out_of_range<T>()
where
    T: ConvertBounds,
{
    repeated_test(|| {
        let source = get_valid_twofloat(|x, _| !(T::lower_bound()..=T::upper_bound()).contains(&x));
        let result = T::try_from(source);

        assert!(
            result.is_err(),
            "Conversion of {:?} produced value instead of error",
            source
        );
    })
}

fn to_twofloat<T>()
where
    T: ConvertBounds,
{
    let test = |source: T| {
        let result: TwoFloat = source.into();

        assert!(result.is_valid(), "Conversion of {:?} was invalid", source);
        assert_eq!(
            result.hi(),
            source.to_f64().unwrap(),
            "Conversion of {source:?} failed: mismatch in high word"
        );
        assert_eq!(
            result.lo(),
            0.0,
            "Conversion of {source:?} failed: non-zero low word"
        );
    };

    if core::mem::size_of::<T>() < core::mem::size_of::<u32>() {
        let mut source = T::min_value();
        loop {
            test(source);
            if source == T::max_value() {
                break;
            }

            source += one();
        }
    } else {
        let mut rng = rand::thread_rng();
        let dist = rand::distributions::Uniform::new_inclusive(T::min_value(), T::max_value());
        repeated_test(|| test(rng.sample(&dist)));
    }
}

macro_rules! int_test {
    ($name:ident::<$t:ty>();) => {
        mod $name {
            #[test]
            fn from_twofloat_lower_bound() {
                super::from_twofloat_lower_bound::<$t>();
            }

            #[test]
            fn from_twofloat_upper_bound() {
                super::from_twofloat_upper_bound::<$t>();
            }

            #[test]
            fn from_twofloat_split_fract() {
                super::from_twofloat_split_fract::<$t>();
            }

            #[test]
            fn from_twofloat_with_fract() {
                super::from_twofloat_with_fract::<$t>();
            }

            #[test]
            fn from_twofloat_out_of_range() {
                super::from_twofloat_out_of_range::<$t>();
            }

            #[test]
            fn to_twofloat() {
                super::to_twofloat::<$t>();
            }
        }
    };
    ($name:ident::<$t:ty>(); $($names:ident::<$ts:ty>();)+) => {
        int_test! { $name::<$t>(); }
        int_test! { $($names::<$ts>();)+ }
    }
}

int_test! {
    i32_test::<i32>();
    i16_test::<i16>();
    i8_test::<i8>();
    u32_test::<u32>();
    u16_test::<u16>();
    u8_test::<u8>();
}

// Helper functions for tests of 64- and 128-bit integer conversions

fn random_mantissa() -> u64 {
    const MANTISSA_RANGE: u64 = 1 << 52;
    rand::thread_rng().gen_range(0..MANTISSA_RANGE)
}

fn random_positive_float_exp_range(exp_range: Range<u64>) -> f64 {
    let mut rng = rand::thread_rng();

    f64::from_bits((rng.gen_range(exp_range.start..exp_range.end) << 52) | random_mantissa())
}

fn random_float_exp_range(exp_range: Range<u64>) -> f64 {
    let x = random_positive_float_exp_range(exp_range);
    if rand::thread_rng().gen() {
        x
    } else {
        -x
    }
}

// Tests for conversions of 64-bit integers

fn from_twofloat_lower_bound64<T>()
where
    T: ConvertBounds,
{
    repeated_test(|| {
        let source = loop {
            if let Ok(result) = try_get_twofloat_with_hi(T::lower_bound()) {
                break result;
            }
        };
        let expected = if source.hi() < T::min_value().to_f64().unwrap() {
            if source.lo() > 0.0 {
                Ok(T::min_value())
            } else {
                Err(TwoFloatError::ConversionError {})
            }
        } else if source.lo() > -1.0 {
            Ok(T::min_value() + T::from(source.lo().ceil()).unwrap())
        } else {
            Err(TwoFloatError::ConversionError {})
        };

        let result = T::try_from(source);
        check_try_from_result(&expected, &result, source);
    });
}

fn from_twofloat_upper_bound64<T>()
where
    T: ConvertBounds,
{
    repeated_test(|| {
        let source = loop {
            if let Ok(result) = try_get_twofloat_with_hi(T::upper_bound()) {
                break result;
            }
        };
        let expected = if source.lo() < 0.0 {
            Ok(T::max_value() - T::from(-source.lo().floor()).unwrap() + one())
        } else {
            Err(TwoFloatError::ConversionError {})
        };

        let result = T::try_from(source);
        check_try_from_result(&expected, &result, source);
    });
}

fn from_twofloat_high_fract64<T>()
where
    T: ConvertBounds,
{
    let mut rng = rand::thread_rng();

    let mut gen_f64 = if T::min_value() == zero() {
        || random_positive_float_exp_range(53..1075)
    } else {
        || random_float_exp_range(53..1075)
    };

    repeated_test(|| {
        let (a, b) = loop {
            let a = get_valid_f64_gen(&mut gen_f64, |x| {
                x > T::lower_bound() && x < T::upper_bound() && x.fract() != 0.0
            });
            let rb = right_bit(a).unwrap_or(i16::MIN);
            if rb < -1019 {
                continue;
            }
            let b_exponent = (rng.gen_range(-1022..rb) + 1023) as u64;
            let b_mantissa = random_mantissa();
            let b = f64::from_bits(b_mantissa | (b_exponent << 52));
            if no_overlap(a, b) {
                break if rng.gen() { (a, b) } else { (a, -b) };
            }
        };

        let source = TwoFloat::try_from((a, b)).unwrap();
        let expected = Ok(T::from(a).unwrap());

        let result = T::try_from(source);
        check_try_from_result(&expected, &result, source);
    });
}

fn from_twofloat_split_fract64<T>()
where
    T: ConvertBounds,
{
    let mut rng = rand::thread_rng();

    let mut gen_f64 = if T::min_value() == zero() {
        || random_positive_float_exp_range(1023..1087)
    } else {
        || random_float_exp_range(1023..1087)
    };

    let fract_dist =
        rand::distributions::Uniform::new(f64::from_bits((-1.0f64).to_bits() - 1), 1.0);
    repeated_test_enumerate(|i| {
        let (a, b) = loop {
            let a = get_valid_f64_gen(&mut gen_f64, |x| {
                x > T::lower_bound() && x < T::upper_bound()
            })
            .trunc();
            if a == 0.0 {
                continue;
            }
            let b = if i == 0 { 0.0 } else { rng.sample(fract_dist) };
            if no_overlap(a, b) {
                break (a, b);
            }
        };

        let source = TwoFloat::try_from((a, b)).unwrap();
        let expected = if a < 0.0 && b > 0.0 {
            Ok(T::from(a).unwrap() + one())
        } else if a > 0.0 && b < 0.0 {
            Ok(T::from(a).unwrap() - one())
        } else {
            Ok(T::from(a).unwrap())
        };

        let result = T::try_from(source);
        check_try_from_result(&expected, &result, source);
    });
}

fn from_twofloat_large64<T>()
where
    T: ConvertBounds,
{
    let mut rng = rand::thread_rng();
    let valid_dist = rand::distributions::Uniform::new(
        f64::from_bits(T::lower_bound().to_bits() - 1),
        T::upper_bound(),
    );
    repeated_test(|| {
        let (a, rb) = loop {
            let a = rng.sample(valid_dist);
            let rb = right_bit(a).unwrap_or(-1) - 1;
            if rb >= 1 {
                break (a, rb);
            }
        };
        let b = loop {
            let b = rng.gen_range(1.0..(1 << rb) as f64);
            if no_overlap(a, b) {
                if rng.gen() {
                    break b;
                } else {
                    break -b;
                }
            }
        };

        let source = TwoFloat::try_from((a, b)).unwrap();
        let expected = if a >= 0.0 {
            if b >= 0.0 {
                Ok(T::from(a).unwrap() + T::from(b).unwrap())
            } else {
                Ok(T::from(a).unwrap() - T::from(-b).unwrap() - one())
            }
        } else if b >= 0.0 {
            Ok(T::from(a).unwrap() + T::from(b).unwrap() + one())
        } else {
            Ok(T::from(a).unwrap() - T::from(-b).unwrap())
        };

        let result = T::try_from(source);
        check_try_from_result(&expected, &result, source);
    });
}

fn from_twofloat_out_of_range64<T>()
where
    T: ConvertBounds,
{
    repeated_test(|| {
        let source = get_valid_twofloat(|x, _| !(T::lower_bound()..=T::upper_bound()).contains(&x));

        let result = T::try_from(source);

        assert!(
            result.is_err(),
            "Conversion of {:?} produced value instead of error",
            source
        );
    })
}

fn to_twofloat64<T>()
where
    T: ConvertBounds,
{
    let mut rng = rand::thread_rng();
    let source_dist = rand::distributions::Uniform::new_inclusive(T::min_value(), T::max_value());
    repeated_test(|| {
        let source = rng.sample(&source_dist);
        let result: TwoFloat = source.into();

        assert!(result.is_valid(), "Conversion of {:?} was invalid", source);
        assert!(
            result.hi() >= T::min_value().to_f64().unwrap()
                && result.hi() <= T::max_value().to_f64().unwrap(),
            "Conversion of {:?} high word out of range",
            source
        );
        assert!(
            result.hi().fract() == 0.0,
            "Integer conversion of {:?} produced a fraction",
            source
        );
        assert!(
            result.lo().fract() == 0.0,
            "Integer conversion of {:?} produced a fraction",
            source
        );

        if result.hi() == T::max_value().to_f64().unwrap() {
            assert!(
                result.lo() < 0.0,
                "Converted result of {:?} out of range",
                source
            );
            assert_eq!(
                T::from(-result.lo()).unwrap() - one(),
                T::max_value() - source,
                "Conversion of {source:?} did not produce matching value"
            );
        } else if result.hi() == T::min_value().to_f64().unwrap() {
            assert!(
                result.lo() >= 0.0,
                "Converted result of {:?} out of range",
                source
            );
            assert_eq!(
                T::from(result.lo()).unwrap(),
                source - T::min_value(),
                "Conversion of {source:?} did not produce matching value"
            );
        } else if result.lo() >= 0.0 {
            assert_eq!(
                T::from(result.hi()).unwrap() + T::from(result.lo()).unwrap(),
                source,
                "Conversion of {source:?} did not produce matching value"
            );
        } else {
            assert_eq!(
                T::from(result.hi()).unwrap() - T::from(-result.lo()).unwrap(),
                source,
                "Conversion of {source:?} did not produce matching value"
            );
        }
    });
}

macro_rules! int64_test {
    ($name:ident::<$t:ty>();) => {
        mod $name {
            #[test]
            fn from_twofloat_lower_bound() {
                super::from_twofloat_lower_bound64::<$t>();
            }

            #[test]
            fn from_twofloat_upper_bound() {
                super::from_twofloat_upper_bound64::<$t>();
            }

            #[test]
            fn from_twofloat_high_fract() {
                super::from_twofloat_high_fract64::<$t>();
            }

            #[test]
            fn from_twofloat_split_fract() {
                super::from_twofloat_split_fract64::<$t>();
            }

            #[test]
            fn from_twofloat_large() {
                super::from_twofloat_large64::<$t>();
            }

            #[test]
            fn from_twofloat_out_of_range() {
                super::from_twofloat_out_of_range64::<$t>();
            }

            #[test]
            fn to_twofloat64() {
                super::to_twofloat64::<$t>();
            }
        }
    };
    ($name:ident::<$t:ty>(); $($names:ident::<$ts:ty>();)+) => {
        int64_test! { $name::<$t>(); }
        int64_test! { $($names::<$ts>();)+ }
    };
}

int64_test! {
    i64_test::<i64>();
    u64_test::<u64>();
}

// Helper trait for 128-bit integers

trait ConvertBounds128: ConvertBounds {
    fn roundtrip_max() -> Self {
        Self::one() << 107
    }
}

impl ConvertBounds128 for i128 {}
impl ConvertBounds128 for u128 {}

// Tests for conversions of 128-bit integers

fn to_twofloat_exact<T>()
where
    T: ConvertBounds128,
{
    let mut rng = rand::thread_rng();
    let source_dist = rand::distributions::Uniform::new_inclusive(
        T::zero().saturating_sub(T::roundtrip_max()),
        T::roundtrip_max(),
    );
    repeated_test(|| {
        let source = rng.sample(&source_dist);
        let result: TwoFloat = source.into();

        assert!(result.is_valid(), "Conversion of {:?} was invalid", source);
        assert!(
            result.hi() >= T::min_value().to_f64().unwrap()
                && result.hi() <= T::max_value().to_f64().unwrap(),
            "Conversion of {:?} high word out of range",
            source
        );
        assert!(
            result.hi().fract() == 0.0,
            "Integer conversion of {:?} produced a fraction",
            source
        );
        assert!(
            result.lo().fract() == 0.0,
            "Integer conversion of {:?} produced a fraction",
            source
        );

        if result.lo() >= 0.0 {
            assert_eq!(
                T::from(result.hi()).unwrap() + T::from(result.lo()).unwrap(),
                source,
                "Conversion of {source:?} did not produce matching value"
            );
        } else {
            assert_eq!(
                T::from(result.hi()).unwrap() - T::from(-result.lo()).unwrap(),
                source,
                "Conversion of {source:?} did not produce matching value"
            );
        }
    });
}

fn inexact_roundtrip<T>()
where
    T: ConvertBounds128,
{
    let mut rng = rand::thread_rng();
    let source_dist =
        rand::distributions::Uniform::new_inclusive(T::roundtrip_max(), T::max_value());
    repeated_test(|| {
        let source = rng.sample(&source_dist);
        let source_signed = if T::min_value() == zero() || rng.gen() {
            source
        } else {
            T::zero() - source
        };

        let value: TwoFloat = source_signed.into();

        assert!(
            value.is_valid(),
            "Conversion of {:?} was invalid",
            source_signed
        );

        match T::try_from(value) {
            Ok(result) => {
                let difference = if result >= source_signed {
                    result - source_signed
                } else {
                    source_signed - result
                };
                assert!(
                    difference.leading_zeros() > source.leading_zeros() + 106,
                    "Conversion of {:?} produced too large error on roundtrip",
                    source_signed
                );
            }
            Err(_) => {
                panic!("Value {:?} produced error on roundtrip", source_signed);
            }
        }
    });
}

macro_rules! int128_test {
    ($name:ident::<$t:ty>();) => {
        mod $name {
            #[test]
            fn from_twofloat_lower_bound() {
                super::from_twofloat_lower_bound64::<$t>();
            }

            #[test]
            fn from_twofloat_upper_bound() {
                super::from_twofloat_upper_bound64::<$t>();
            }

            #[test]
            fn from_twofloat_high_fract() {
                super::from_twofloat_high_fract64::<$t>();
            }

            #[test]
            fn from_twofloat_split_fract() {
                super::from_twofloat_split_fract64::<$t>();
            }

            #[test]
            fn from_twofloat_out_of_range() {
                super::from_twofloat_out_of_range64::<$t>();
            }

            #[test]
            fn to_twofloat() {
                super::to_twofloat_exact::<$t>();
            }

            #[test]
            fn inexact_roundtrip() {
                super::inexact_roundtrip::<$t>();
            }
        }
    };
    ($name:ident::<$t:ty>(); $($names:ident::<$ts:ty>();)+) => {
        int128_test! { $name::<$t>(); }
        int128_test! { $($names::<$ts>();)+ }
    };
}

int128_test! {
    i128_test::<i128>();
    u128_test::<u128>();
}
