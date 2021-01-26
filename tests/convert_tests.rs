#![allow(clippy::float_cmp)]

use core::{convert::TryFrom, mem::discriminant, ops::Range};

use rand::Rng;
use twofloat::{no_overlap, TwoFloat, TwoFloatError};

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

macro_rules! float_test {
    ($type:tt, $from_test:ident, $into_test:ident) => {
        #[test]
        fn $from_test() {
            repeated_test(|| {
                let source = loop {
                    let source = random_float() as $type;
                    if source.is_finite() {
                        break source;
                    };
                };

                let result: TwoFloat = source.into();

                assert_eq!(
                    result.hi(),
                    source as f64,
                    "Float conversion failed: mismatch in high word"
                );
                assert_eq!(
                    result.lo(),
                    0.0,
                    "Float conversion failed: non-zero low word"
                );
            });
        }

        #[test]
        fn $into_test() {
            repeated_test(|| {
                let source = get_twofloat();
                let source_ref = &source;

                let result: $type = source.into();
                assert_eq!(
                    result,
                    source.hi() as $type,
                    "Float conversion from TwoFloat failed"
                );

                let result_ref: $type = source_ref.into();

                assert_eq!(
                    result, result_ref,
                    "Value and reference float conversion give different results"
                );
            });
        }
    };
}

float_test!(f64, from_f64_test, into_f64_test);
float_test!(f32, from_f32_test, into_f32_test);

fn check_try_from_result<T: core::fmt::Debug + PartialEq>(
    expected: &Result<T, TwoFloatError>,
    result: &Result<T, TwoFloatError>,
    source: TwoFloat,
) {
    assert_eq!(
        discriminant(expected),
        discriminant(result),
        "Conversion of {:?} produced unexpected Err/Ok state",
        source
    );
    match (expected, result) {
        (Ok(expected_value), Ok(result_value)) => assert_eq!(
            expected_value, result_value,
            "Conversion of {:?} produced incorrect result",
            source
        ),
        (Err(expected_err), Err(result_err)) => assert_eq!(
            discriminant(expected_err),
            discriminant(result_err),
            "Conversion of {:?} produced mismatched error types",
            source
        ),
        _ => unreachable!(),
    }
}

macro_rules! from_twofloat_test {
    ($type:tt) => {
        const LOWER_BOUND: f64 = $type::MIN as f64 - 1.0;
        const UPPER_BOUND: f64 = $type::MAX as f64 + 1.0;

        #[test]
        fn from_twofloat_lower_bound() {
            repeated_test(|| {
                let source = loop {
                    if let Ok(source) = try_get_twofloat_with_hi(LOWER_BOUND) {
                        break source;
                    }
                };

                let expected = if source.lo() > 0.0 {
                    Ok($type::MIN)
                } else {
                    Err(TwoFloatError::ConversionError {})
                };
                let result = $type::try_from(source);

                check_try_from_result(&expected, &result, source);

                let result_ref = $type::try_from(&source);
                check_try_from_result(&expected, &result_ref, source);
            })
        }

        #[test]
        fn from_twofloat_upper_bound() {
            repeated_test(|| {
                let source = loop {
                    if let Ok(source) = try_get_twofloat_with_hi(UPPER_BOUND) {
                        break source;
                    }
                };

                let expected = if source.lo() < 0.0 {
                    Ok($type::MAX)
                } else {
                    Err(TwoFloatError::ConversionError {})
                };

                let result = $type::try_from(source);
                check_try_from_result(&expected, &result, source);

                let result_ref = $type::try_from(&source);
                check_try_from_result(&expected, &result_ref, source);
            })
        }

        #[test]
        fn from_twofloat_split_fract() {
            let mut rng = rand::thread_rng();
            let valid_dist = rand::distributions::Uniform::new(
                f64::from_bits(LOWER_BOUND.to_bits() - 1),
                UPPER_BOUND,
            );

            for i in 0..TEST_ITERS {
                let (a, b, source) = loop {
                    let a = rng.sample(valid_dist).trunc();
                    let b = if i == 0 { 0.0 } else { random_float() };
                    if let Ok(source) = TwoFloat::try_from((a, b)) {
                        break (a, b, source);
                    }
                };

                let expected = if a < 0.0 && b > 0.0 {
                    Ok(a as $type + 1)
                } else if a > 0.0 && b < 0.0 {
                    Ok(a as $type - 1)
                } else {
                    Ok(a as $type)
                };

                let result = $type::try_from(source);
                check_try_from_result(&expected, &result, source);

                let result_ref = $type::try_from(&source);
                check_try_from_result(&expected, &result_ref, source);
            }
        }

        #[test]
        fn from_twofloat_with_fract() {
            let mut rng = rand::thread_rng();
            let valid_dist = rand::distributions::Uniform::new(
                f64::from_bits(LOWER_BOUND.to_bits() - 1),
                UPPER_BOUND,
            );

            for i in 0..TEST_ITERS {
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

                let expected = Ok(a.trunc() as $type);

                let result = $type::try_from(source);
                check_try_from_result(&expected, &result, source);

                let result_ref = $type::try_from(&source);
                check_try_from_result(&expected, &result_ref, source);
            }
        }

        #[test]
        fn from_twofloat_out_of_range() {
            repeated_test(|| {
                let source = get_valid_twofloat(|x, _| !(LOWER_BOUND..=UPPER_BOUND).contains(&x));
                let result = $type::try_from(source);

                assert!(
                    result.is_err(),
                    "Conversion of {:?} produced value instead of error",
                    source
                );

                let result_ref = $type::try_from(&source);
                assert!(
                    result_ref.is_err(),
                    "Conversion of {:?} produced value instead of error",
                    source
                );
            })
        }
    };
}

macro_rules! int_test {
    ($type:tt, $test_i:ident, false) => {
        #[cfg(test)]
        mod $test_i {
            use super::*;

            #[test]
            fn to_twofloat() {
                let mut rng = rand::thread_rng();
                let dist = rand::distributions::Uniform::new_inclusive($type::MIN, $type::MAX);
                for _ in 0..TEST_ITERS {
                    let source = rng.sample(dist);

                    let result: TwoFloat = source.into();

                    assert!(result.is_valid(), "Conversion of {} was invalid", source);
                    assert_eq!(
                        result.hi(),
                        source as f64,
                        "Conversion of {} failed: mismatch in high word",
                        source
                    );
                    assert_eq!(
                        result.lo(),
                        0.0,
                        "Conversion of {} failed: non-zero low word",
                        source
                    );
                }
            }

            from_twofloat_test!($type);
        }
    };
    ($type:tt, $test_i:ident, true) => {
        #[cfg(test)]
        mod $test_i {
            use super::*;

            #[test]
            fn to_twofloat() {
                let mut source = $type::MIN;
                loop {
                    let result: TwoFloat = source.into();

                    assert!(result.is_valid(), "Conversion of {} was invalid", source);
                    assert_eq!(
                        result.hi(),
                        source as f64,
                        "Conversion of {} failed: mismatch in high word",
                        source
                    );
                    assert_eq!(
                        result.lo(),
                        0.0,
                        "Conversion of {} failed: non-zero low word",
                        source
                    );

                    if source == $type::MAX {
                        break;
                    }
                    source += 1;
                }
            }

            from_twofloat_test!($type);
        }
    };
}

int_test!(i32, i32_test, false);
int_test!(i16, i16_test, true);
int_test!(i8, i8_test, true);
int_test!(u32, u32_test, false);
int_test!(u16, u16_test, true);
int_test!(u8, u8_test, true);

fn random_mantissa() -> u64 {
    rand::thread_rng().gen_range(0, 1 << 52)
}

fn random_positive_float_exp_range(exp_range: Range<u64>) -> f64 {
    let mut rng = rand::thread_rng();

    f64::from_bits((rng.gen_range(exp_range.start, exp_range.end) << 52) | random_mantissa())
}

fn random_float_exp_range(exp_range: Range<u64>) -> f64 {
    let x = random_positive_float_exp_range(exp_range);
    if rand::thread_rng().gen() {
        x
    } else {
        -x
    }
}

macro_rules! int64_test {
    ($type:tt, $test_i:ident) => {
        #[cfg(test)]
        mod $test_i {
            use super::*;

            #[test]
            fn to_twofloat() {
                let mut rng = rand::thread_rng();
                let source_dist =
                    rand::distributions::Uniform::new_inclusive($type::MIN, $type::MAX);
                for _ in 0..TEST_ITERS {
                    let source = rng.sample(source_dist);
                    let result: TwoFloat = source.into();

                    assert!(result.is_valid(), "Conversion of {} was invalid", source);
                    assert!(
                        result.hi() >= $type::MIN as f64 && result.hi() <= $type::MAX as f64,
                        "Conversion of {} high word out of range",
                        source
                    );
                    assert!(
                        result.hi().fract() == 0.0,
                        "Integer conversion of {} produced a fraction",
                        source
                    );
                    assert!(
                        result.lo().fract() == 0.0,
                        "Integer conversion of {} produced a fraction",
                        source
                    );

                    if result.hi() == $type::MAX as f64 {
                        assert!(
                            result.lo() < 0.0,
                            "Converted result of {} out of range",
                            source
                        );
                        assert_eq!(
                            (-result.lo()) as $type - 1,
                            $type::MAX - source,
                            "Conversion of {} did not produce matching value",
                            source
                        );
                    } else if result.hi() == $type::MIN as f64 {
                        assert!(
                            result.lo() >= 0.0,
                            "Converted result of {} out of range",
                            source
                        );
                        assert_eq!(
                            result.lo() as $type,
                            source - $type::MIN,
                            "Conversion of {} did not produce matching value",
                            source
                        );
                    } else if result.lo() >= 0.0 {
                        assert_eq!(
                            result.hi() as $type + result.lo() as $type,
                            source,
                            "Conversion of {} did not produce matching value",
                            source
                        );
                    } else {
                        assert_eq!(
                            result.hi() as $type - ((-result.lo()) as $type),
                            source,
                            "Conversion of {} did not produce matching value",
                            source
                        );
                    }
                }
            }

            const LOWER_BOUND: f64 = $type::MIN as f64 - 1.0;
            const UPPER_BOUND: f64 = $type::MAX as f64;

            #[test]
            fn from_twofloat_lower_bound() {
                repeated_test(|| {
                    let source = loop {
                        if let Ok(result) = try_get_twofloat_with_hi(LOWER_BOUND) {
                            break result;
                        }
                    };
                    let expected = if source.hi() < $type::MIN as f64 {
                        if source.lo() > 0.0 {
                            Ok($type::MIN)
                        } else {
                            Err(TwoFloatError::ConversionError {})
                        }
                    } else {
                        if source.lo() > -1.0 {
                            Ok($type::MIN + source.lo().ceil() as $type)
                        } else {
                            Err(TwoFloatError::ConversionError {})
                        }
                    };

                    let result = $type::try_from(source);
                    check_try_from_result(&expected, &result, source);

                    let result_ref = $type::try_from(&source);
                    check_try_from_result(&expected, &result_ref, source);
                });
            }

            #[test]
            fn from_twofloat_upper_bound() {
                repeated_test(|| {
                    let source = loop {
                        if let Ok(result) = try_get_twofloat_with_hi(UPPER_BOUND) {
                            break result;
                        }
                    };
                    let expected = if source.lo() < 0.0 {
                        Ok($type::MAX - ((-source.lo().floor()) as $type) + 1)
                    } else {
                        Err(TwoFloatError::ConversionError {})
                    };

                    let result = $type::try_from(source);
                    check_try_from_result(&expected, &result, source);

                    let result_ref = $type::try_from(&source);
                    check_try_from_result(&expected, &result_ref, source);
                });
            }

            #[test]
            fn from_twofloat_high_fract() {
                let mut rng = rand::thread_rng();

                let mut gen_f64 = if $type::MIN == 0 {
                    || random_positive_float_exp_range(53..1075)
                } else {
                    || random_float_exp_range(53..1075)
                };

                for _ in 0..TEST_ITERS {
                    let (a, b) = loop {
                        let a = get_valid_f64_gen(&mut gen_f64, |x| {
                            x > LOWER_BOUND && x < UPPER_BOUND && x.fract() != 0.0
                        });
                        let rb = right_bit(a).unwrap_or(i16::MIN);
                        if (rb < -1019) {
                            continue;
                        }
                        let b_exponent = (rng.gen_range(-1022, rb) + 1023) as u64;
                        let b_mantissa = random_mantissa();
                        let b = f64::from_bits(b_mantissa | (b_exponent << 52));
                        if no_overlap(a, b) {
                            break if rng.gen() { (a, b) } else { (a, -b) };
                        }
                    };

                    let source = TwoFloat::try_from((a, b)).unwrap();
                    let expected = Ok(a as $type);

                    let result = $type::try_from(source);
                    check_try_from_result(&expected, &result, source);

                    let result_ref = $type::try_from(&source);
                    check_try_from_result(&expected, &result_ref, source);
                }
            }

            #[test]
            fn from_twofloat_split_fract() {
                let mut rng = rand::thread_rng();

                let mut gen_f64 = if $type::MIN == 0 {
                    || random_positive_float_exp_range(1023..1087)
                } else {
                    || random_float_exp_range(1023..1087)
                };

                let fract_dist =
                    rand::distributions::Uniform::new(f64::from_bits((-1.0f64).to_bits() - 1), 1.0);
                for i in 0..TEST_ITERS {
                    let (a, b) = loop {
                        let a =
                            get_valid_f64_gen(&mut gen_f64, |x| x > LOWER_BOUND && x < UPPER_BOUND)
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
                        Ok(a as $type + 1)
                    } else if a > 0.0 && b < 0.0 {
                        Ok(a as $type - 1)
                    } else {
                        Ok(a as $type)
                    };

                    let result = $type::try_from(source);
                    check_try_from_result(&expected, &result, source);

                    let result_ref = $type::try_from(&source);
                    check_try_from_result(&expected, &result_ref, source);
                }
            }

            #[test]
            fn from_twofloat_large() {
                let mut rng = rand::thread_rng();
                let valid_dist = rand::distributions::Uniform::new(
                    f64::from_bits(LOWER_BOUND.to_bits() - 1),
                    UPPER_BOUND,
                );
                for _ in 0..TEST_ITERS {
                    let (a, rb) = loop {
                        let a = rng.sample(valid_dist);
                        let rb = right_bit(a).unwrap_or(-1) - 1;
                        if rb >= 1 {
                            break (a, rb);
                        }
                    };
                    let b = loop {
                        let b = rng.gen_range(1.0, (1 << rb) as f64);
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
                            Ok(a as $type + b as $type)
                        } else {
                            Ok(a as $type - (-b) as $type - 1)
                        }
                    } else {
                        if b >= 0.0 {
                            Ok(a as $type + b as $type + 1)
                        } else {
                            Ok(a as $type - (-b) as $type)
                        }
                    };

                    let result = $type::try_from(source);
                    check_try_from_result(&expected, &result, source);

                    let result_ref = $type::try_from(&source);
                    check_try_from_result(&expected, &result_ref, source);
                }
            }

            #[test]
            fn from_twofloat_out_of_range() {
                repeated_test(|| {
                    let source =
                        get_valid_twofloat(|x, _| !(LOWER_BOUND..=UPPER_BOUND).contains(&x));

                    let result = $type::try_from(source);

                    assert!(
                        result.is_err(),
                        "Conversion of {:?} produced value instead of error",
                        source
                    );

                    let result_ref = $type::try_from(&source);
                    assert!(
                        result_ref.is_err(),
                        "Conversion of {:?} produced value instead of error",
                        source
                    );
                })
            }
        }
    };
}

int64_test!(i64, i64_test);
int64_test!(u64, u64_test);

macro_rules! int128_test {
    ($type:tt, $test_i:ident) => {
        #[cfg(test)]
        mod $test_i {
            use super::*;

            const ROUNDTRIP_MAX: $type = 1 << 107;

            #[test]
            fn to_twofloat_exact() {
                let mut rng = rand::thread_rng();
                let source_dist = rand::distributions::Uniform::new_inclusive(
                    (0 as $type).saturating_sub(ROUNDTRIP_MAX),
                    ROUNDTRIP_MAX,
                );
                for _ in 0..TEST_ITERS {
                    let source = rng.sample(source_dist);
                    let result: TwoFloat = source.into();

                    assert!(result.is_valid(), "Conversion of {} was invalid", source);
                    assert!(
                        result.hi() >= $type::MIN as f64 && result.hi() <= $type::MAX as f64,
                        "Conversion of {} high word out of range",
                        source
                    );
                    assert!(
                        result.hi().fract() == 0.0,
                        "Integer conversion of {} produced a fraction",
                        source
                    );
                    assert!(
                        result.lo().fract() == 0.0,
                        "Integer conversion of {} produced a fraction",
                        source
                    );

                    if result.lo() >= 0.0 {
                        assert_eq!(
                            result.hi() as $type + result.lo() as $type,
                            source,
                            "Conversion of {} did not produce matching value",
                            source
                        );
                    } else {
                        assert_eq!(
                            result.hi() as $type - ((-result.lo()) as $type),
                            source,
                            "Conversion of {} did not produce matching value",
                            source
                        );
                    }
                }
            }

            #[test]
            fn inexact_roundtrip() {
                let mut rng = rand::thread_rng();
                let source_dist =
                    rand::distributions::Uniform::new_inclusive(ROUNDTRIP_MAX, $type::MAX);
                for _ in 0..TEST_ITERS {
                    let source = rng.sample(source_dist);
                    let source_signed = if $type::MIN == 0 || rng.gen() {
                        source
                    } else {
                        0 - source
                    };

                    let value: TwoFloat = source_signed.into();

                    assert!(
                        value.is_valid(),
                        "Conversion of {} was invalid",
                        source_signed
                    );

                    match $type::try_from(value) {
                        Ok(result) => {
                            let difference = if result >= source_signed {
                                result - source_signed
                            } else {
                                source_signed - result
                            };
                            assert!(
                                difference.leading_zeros() > source.leading_zeros() + 106,
                                "Conversion of {} produced too large error on roundtrip",
                                source_signed
                            );
                        }
                        Err(_) => {
                            panic!("Value {} produced error on roundtrip", source_signed);
                        }
                    }
                }
            }

            const LOWER_BOUND: f64 = $type::MIN as f64 - 1.0;
            const UPPER_BOUND: f64 = $type::MAX as f64;

            #[test]
            fn from_twofloat_lower_bound() {
                repeated_test(|| {
                    let source = loop {
                        if let Ok(result) = try_get_twofloat_with_hi(LOWER_BOUND) {
                            break result;
                        }
                    };
                    let expected = if source.hi() < $type::MIN as f64 {
                        if source.lo() > 0.0 {
                            Ok($type::MIN)
                        } else {
                            Err(TwoFloatError::ConversionError {})
                        }
                    } else {
                        if source.lo() > -1.0 {
                            Ok($type::MIN + source.lo().ceil() as $type)
                        } else {
                            Err(TwoFloatError::ConversionError {})
                        }
                    };

                    let result = $type::try_from(source);
                    check_try_from_result(&expected, &result, source);

                    let result_ref = $type::try_from(&source);
                    check_try_from_result(&expected, &result_ref, source);
                })
            }

            #[test]
            fn from_twofloat_upper_bound() {
                repeated_test(|| {
                    let source = loop {
                        if let Ok(result) = try_get_twofloat_with_hi(UPPER_BOUND) {
                            break result;
                        }
                    };
                    let expected = if source.lo() < 0.0 {
                        Ok($type::MAX - ((-source.lo().floor()) as $type) + 1)
                    } else {
                        Err(TwoFloatError::ConversionError {})
                    };

                    let result = $type::try_from(source);
                    check_try_from_result(&expected, &result, source);

                    let result_ref = $type::try_from(&source);
                    check_try_from_result(&expected, &result_ref, source);
                })
            }

            #[test]
            fn from_twofloat_high_fract() {
                let mut rng = rand::thread_rng();

                let mut gen_f64 = if $type::MIN == 0 {
                    || random_positive_float_exp_range(53..1075)
                } else {
                    || random_float_exp_range(53..1075)
                };

                for _ in 0..TEST_ITERS {
                    let (a, b) = loop {
                        let a = get_valid_f64_gen(&mut gen_f64, |x| {
                            x > LOWER_BOUND && x < UPPER_BOUND && x.fract() != 0.0
                        });
                        let rb = right_bit(a).unwrap_or(i16::MIN);
                        if (rb < -1019) {
                            continue;
                        }
                        let b_exponent = (rng.gen_range(-1022, rb) + 1023) as u64;
                        let b_mantissa = random_mantissa();
                        let b = f64::from_bits(b_mantissa | (b_exponent << 52));
                        if no_overlap(a, b) {
                            break if rng.gen() { (a, b) } else { (a, -b) };
                        }
                    };

                    let source = TwoFloat::try_from((a, b)).unwrap();
                    let expected = Ok(a as $type);

                    let result = $type::try_from(source);
                    check_try_from_result(&expected, &result, source);

                    let result_ref = $type::try_from(&source);
                    check_try_from_result(&expected, &result_ref, source);
                }
            }

            #[test]
            fn from_twofloat_split_fract() {
                let mut rng = rand::thread_rng();

                let mut gen_f64 = if $type::MIN == 0 {
                    || random_positive_float_exp_range(1023..1087)
                } else {
                    || random_float_exp_range(1023..1087)
                };

                let fract_dist =
                    rand::distributions::Uniform::new(f64::from_bits((-1.0f64).to_bits() - 1), 1.0);
                for i in 0..TEST_ITERS {
                    let (a, b) = loop {
                        let a =
                            get_valid_f64_gen(&mut gen_f64, |x| x > LOWER_BOUND && x < UPPER_BOUND)
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
                        Ok(a as $type + 1)
                    } else if a > 0.0 && b < 0.0 {
                        Ok(a as $type - 1)
                    } else {
                        Ok(a as $type)
                    };

                    let result = $type::try_from(source);
                    check_try_from_result(&expected, &result, source);

                    let result_ref = $type::try_from(&source);
                    check_try_from_result(&expected, &result_ref, source);
                }
            }

            #[test]
            fn from_twofloat_out_of_range() {
                repeated_test(|| {
                    let source =
                        get_valid_twofloat(|x, _| !(LOWER_BOUND..=UPPER_BOUND).contains(&x));

                    let result = $type::try_from(source);

                    assert!(
                        result.is_err(),
                        "Conversion of {:?} produced value instead of error",
                        source
                    );

                    let result_ref = $type::try_from(&source);
                    assert!(
                        result_ref.is_err(),
                        "Conversion of {:?} produced value instead of error",
                        source
                    );
                })
            }
        }
    };
}

int128_test!(i128, i128_test);
int128_test!(u128, u128_test);
