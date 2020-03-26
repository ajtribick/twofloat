use std::convert::TryFrom;

use rand::Rng;
use twofloat::*;

pub mod common;
use common::*;

fn right_bit(f: f64) -> Option<i16> {
    let fbits = f.to_bits();
    let exponent = ((fbits >> 52) & 0x7ff) as i16 - 1023;
    match exponent {
        -1023 => {
            let mantissa = fbits & ((1 << 52) - 1);
            if mantissa == 0 {
                Some(std::i16::MIN)
            } else {
                Some(-1074)
            }
        }
        1024 => None,
        _ => Some(exponent - 52),
    }
}

macro_rules! float_test {
    ($type:tt, $from_test:ident, $into_test:ident) => {
        randomized_test!($from_test, |rng: F64Rand| {
            let source = loop {
                let source = rng() as $type;
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

        randomized_test!($into_test, |rng: F64Rand| {
            let source = get_twofloat(rng);
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
    };
}

float_test!(f64, from_f64_test, into_f64_test);
float_test!(f32, from_f32_test, into_f32_test);

fn check_try_from_result<T: std::fmt::Debug + PartialEq>(
    expected: Result<T, ()>,
    result: Result<T, ()>,
    source: TwoFloat,
) {
    if let Ok(expected_value) = expected {
        assert!(
            result.is_ok(),
            "Conversion of {:?} produced error instead of result",
            source
        );
        assert_eq!(
            result.unwrap(),
            expected_value,
            "Conversion of {:?} produced incorrect result",
            source
        );
    } else {
        assert!(
            result.is_err(),
            "Conversion of {:?} produced result instead of error",
            source
        );
    }
}

macro_rules! from_twofloat_test {
    ($type:tt) => {
        const LOWER_BOUND: f64 = std::$type::MIN as f64 - 1.0;
        const UPPER_BOUND: f64 = std::$type::MAX as f64 + 1.0;

        randomized_test!(from_twofloat_lower_bound, |rng: F64Rand| {
            let source = loop {
                if let Ok(source) = try_get_twofloat_with_hi(rng, LOWER_BOUND) {
                    break source;
                }
            };

            let expected = if source.lo() > 0.0 {
                Ok(std::$type::MIN)
            } else {
                Err(())
            };
            let result = $type::try_from(source);

            check_try_from_result(expected, result, source);

            let result_ref = $type::try_from(&source);
            assert_eq!(
                result, result_ref,
                "Different value and reference conversions for {:?}",
                source
            );
        });

        randomized_test!(from_twofloat_upper_bound, |rng: F64Rand| {
            let source = loop {
                if let Ok(source) = try_get_twofloat_with_hi(rng, UPPER_BOUND) {
                    break source;
                }
            };
            let expected = if source.lo() < 0.0 {
                Ok(std::$type::MAX)
            } else {
                Err(())
            };
            let result = $type::try_from(source);

            check_try_from_result(expected, result, source);

            let result_ref = $type::try_from(&source);
            assert_eq!(
                result, result_ref,
                "Different value and reference conversions for {:?}",
                source
            );
        });

        #[test]
        fn from_twofloat_split_fract() {
            let mut rng = rand::thread_rng();
            let mut get_f64 = float_generator();
            let valid_dist = rand::distributions::Uniform::new(
                f64::from_bits(LOWER_BOUND.to_bits() - 1),
                UPPER_BOUND,
            );

            for i in 0..TEST_ITERS {
                let (a, b, source) = loop {
                    let a = rng.sample(valid_dist).trunc();
                    let b = if i == 0 { 0.0 } else { get_f64() };
                    if let Ok(source) = TwoFloat::try_new(a, b) {
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

                check_try_from_result(expected, result, source);

                let result_ref = $type::try_from(&source);
                assert_eq!(
                    result, result_ref,
                    "Different value and reference conversions for {:?}",
                    source
                );
            }
        }

        #[test]
        fn from_twofloat_with_fract() {
            let mut rng = rand::thread_rng();
            let mut get_f64 = float_generator();
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
                    let b = if i == 0 { 0.0 } else { get_f64() };
                    if let Ok(source) = TwoFloat::try_new(a, b) {
                        break (a, source);
                    }
                };

                let expected = Ok(a.trunc() as $type);
                let result = $type::try_from(source);

                check_try_from_result(expected, result, source);

                let result_ref = $type::try_from(&source);
                assert_eq!(
                    result, result_ref,
                    "Different value and reference conversions for {:?}",
                    source
                );
            }
        }

        randomized_test!(from_twofloat_out_of_range, |rng: F64Rand| {
            let source = get_valid_twofloat(rng, |x, _| x < LOWER_BOUND || x > UPPER_BOUND);
            let result = $type::try_from(source);

            assert!(
                result.is_err(),
                "Conversion of {:?} produced value instead of error",
                source
            );

            let result_ref = $type::try_from(&source);
            assert_eq!(
                result, result_ref,
                "Different value and reference conversions for {:?}",
                source
            );
        });
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
                let dist =
                    rand::distributions::Uniform::new_inclusive(std::$type::MIN, std::$type::MAX);
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
                let mut source = std::$type::MIN;
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

                    if source == std::$type::MAX {
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

macro_rules! int64_test {
    ($type:tt, $test_i:ident) => {
        #[cfg(test)]
        mod $test_i {
            use super::*;

            #[test]
            fn to_twofloat() {
                let mut rng = rand::thread_rng();
                let source_dist =
                    rand::distributions::Uniform::new_inclusive(std::$type::MIN, std::$type::MAX);
                for _ in 0..TEST_ITERS {
                    let source = rng.sample(source_dist);
                    let result: TwoFloat = source.into();

                    assert!(result.is_valid(), "Conversion of {} was invalid", source);
                    assert!(
                        result.hi() >= std::$type::MIN as f64
                            && result.hi() <= std::$type::MAX as f64,
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

                    if result.hi() == std::$type::MAX as f64 {
                        assert!(
                            result.lo() < 0.0,
                            "Converted result of {} out of range",
                            source
                        );
                        assert_eq!(
                            (-result.lo()) as $type - 1,
                            std::$type::MAX - source,
                            "Conversion of {} did not produce matching value",
                            source
                        );
                    } else if result.hi() == std::$type::MIN as f64 {
                        assert!(
                            result.lo() >= 0.0,
                            "Converted result of {} out of range",
                            source
                        );
                        assert_eq!(
                            result.lo() as $type,
                            source - std::$type::MIN,
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

            const LOWER_BOUND: f64 = std::$type::MIN as f64 - 1.0;
            const UPPER_BOUND: f64 = std::$type::MAX as f64;

            randomized_test!(from_twofloat_lower_bound, |rng: F64Rand| {
                let source = loop {
                    if let Ok(result) = try_get_twofloat_with_hi(rng, LOWER_BOUND) {
                        break result;
                    }
                };
                let expected = if source.lo() >= 0.0 {
                    Ok(std::$type::MIN + source.lo() as $type)
                } else {
                    Err(())
                };

                let result = $type::try_from(source);

                check_try_from_result(expected, result, source);

                let result_ref = $type::try_from(&source);
                assert_eq!(
                    result, result_ref,
                    "Different value and reference conversions for {:?}",
                    source
                );
            });

            randomized_test!(from_twofloat_upper_bound, |rng: F64Rand| {
                let source = loop {
                    if let Ok(result) = try_get_twofloat_with_hi(rng, UPPER_BOUND) {
                        break result;
                    }
                };
                let expected = if source.lo() < 0.0 {
                    Ok(std::$type::MAX - ((-source.lo().floor()) as $type) + 1)
                } else {
                    Err(())
                };
                let result = $type::try_from(source);

                check_try_from_result(expected, result, source);

                let result_ref = $type::try_from(&source);
                assert_eq!(
                    result, result_ref,
                    "Different value and reference conversions for {:?}",
                    source
                );
            });

            #[test]
            fn from_twofloat_high_fract() {
                let mut rng = rand::thread_rng();

                let exponent_dist = rand::distributions::Uniform::new(53u64, 1075u64);
                let mantissa_dist = rand::distributions::Uniform::new(0u64, 1u64 << 52);

                let mut gen_valid_f64 = move || {
                    let x = f64::from_bits(
                        rng.sample(mantissa_dist) | (rng.sample(exponent_dist) << 52),
                    );
                    if std::$type::MIN == 0 || rng.gen() {
                        x
                    } else {
                        -x
                    }
                };

                for _ in 0..TEST_ITERS {
                    let (a, b) = loop {
                        let a = get_valid_f64(&mut gen_valid_f64, |x| {
                            x > LOWER_BOUND && x < UPPER_BOUND && x.fract() != 0.0
                        });
                        let rb = right_bit(a).unwrap_or(std::i16::MIN);
                        if (rb < -1019) {
                            continue;
                        }
                        let b_exponent = (rng.gen_range(-1022, rb) + 1023) as u64;
                        let b_mantissa = rng.sample(mantissa_dist);
                        let b = f64::from_bits(b_mantissa | (b_exponent << 52));
                        if no_overlap(a, b) {
                            break if rng.gen() { (a, b) } else { (a, -b) };
                        }
                    };

                    let source = TwoFloat::try_new(a, b).unwrap();
                    let expected = Ok(a as $type);
                    let result = $type::try_from(source);

                    check_try_from_result(expected, result, source);

                    let result_ref = $type::try_from(&source);
                    assert_eq!(
                        result, result_ref,
                        "Different value and reference conversions for {:?}",
                        source
                    );
                }
            }

            #[test]
            fn from_twofloat_split_fract() {
                let mut rng = rand::thread_rng();
                let exponent_dist = rand::distributions::Uniform::new(1023u64, 1087u64);
                let mantissa_dist = rand::distributions::Uniform::new(0u64, 1u64 << 52);

                let mut gen_f64 = move || {
                    let x = f64::from_bits(
                        rng.sample(mantissa_dist) | (rng.sample(exponent_dist) << 52),
                    );
                    if std::$type::MIN == 0 || rng.gen() {
                        x
                    } else {
                        -x
                    }
                };

                let fract_dist =
                    rand::distributions::Uniform::new(f64::from_bits((-1.0f64).to_bits() - 1), 1.0);
                for i in 0..TEST_ITERS {
                    let (a, b) = loop {
                        let a = get_valid_f64(&mut gen_f64, |x| x > LOWER_BOUND && x < UPPER_BOUND)
                            .trunc();
                        if a == 0.0 {
                            continue;
                        }
                        let b = if i == 0 { 0.0 } else { rng.sample(fract_dist) };
                        if no_overlap(a, b) {
                            break (a, b);
                        }
                    };

                    let source = TwoFloat::try_new(a, b).unwrap();
                    let expected = if a < 0.0 && b > 0.0 {
                        Ok(a as $type + 1)
                    } else if a > 0.0 && b < 0.0 {
                        Ok(a as $type - 1)
                    } else {
                        Ok(a as $type)
                    };
                    let result = $type::try_from(source);

                    check_try_from_result(expected, result, source);

                    let result_ref = $type::try_from(&source);
                    assert_eq!(
                        result, result_ref,
                        "Different value and reference conversions for {:?}",
                        source
                    );
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
                            break b;
                        }
                    };

                    let source = TwoFloat::try_new(a, b).unwrap();
                    let expected = if b >= 0.0 {
                        Ok(a as $type + b as $type)
                    } else {
                        Ok(a as $type - (-b) as $type)
                    };
                    let result = $type::try_from(source);

                    check_try_from_result(expected, result, source);

                    let result_ref = $type::try_from(&source);
                    assert_eq!(
                        result, result_ref,
                        "Different value and reference conversions for {:?}",
                        source
                    );
                }
            }

            randomized_test!(from_twofloat_out_of_range, |rng: F64Rand| {
                let source = get_valid_twofloat(rng, |x, _| x < LOWER_BOUND || x > UPPER_BOUND);

                let result = $type::try_from(source);

                assert!(
                    result.is_err(),
                    "Conversion of {:?} produced value instead of error",
                    source
                );

                let result_ref = $type::try_from(&source);
                assert_eq!(
                    result, result_ref,
                    "Different value and reference conversions for {:?}",
                    source
                );
            });
        }
    };
}

int64_test!(i64, i64_test);
int64_test!(u64, u64_test);
