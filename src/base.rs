use std::cmp::Ordering;
use std::convert::{From, TryFrom};

/// Represents a two-word floating point type, represented as the sum of two
/// non-overlapping f64 values.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TwoFloat {
    pub(crate) hi: f64,
    pub(crate) lo: f64,
}

impl PartialEq<f64> for TwoFloat {
    fn eq(&self, other: &f64) -> bool {
        self.hi.eq(other) && self.lo == 0.0
    }
}

impl PartialEq<TwoFloat> for f64 {
    fn eq(&self, other: &TwoFloat) -> bool {
        self.eq(&other.hi) && other.lo == 0.0
    }
}

impl PartialOrd<f64> for TwoFloat {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        let hi_cmp = self.hi.partial_cmp(other);
        if hi_cmp == Some(Ordering::Equal) {
            self.lo.partial_cmp(&0.0)
        } else {
            hi_cmp
        }
    }
}

impl PartialOrd<TwoFloat> for f64 {
    fn partial_cmp(&self, other: &TwoFloat) -> Option<Ordering> {
        let hi_cmp = self.partial_cmp(&other.hi);
        if hi_cmp == Some(Ordering::Equal) {
            0.0.partial_cmp(&other.lo)
        } else {
            hi_cmp
        }
    }
}

macro_rules! from_conversion {
    (|$source_i:ident : TwoFloat| -> $dest:tt $code:block) => {
        impl From<TwoFloat> for $dest {
            fn from($source_i: TwoFloat) -> Self $code
        }

        impl<'a> From<&'a TwoFloat> for $dest {
            fn from($source_i: &'a TwoFloat) -> Self $code
        }
    };
    (|$source_i:ident: TwoFloat| -> Result<$dest:tt, $err:tt> $code:block) => {
        impl TryFrom<TwoFloat> for $dest {
            type Error = $err;

            fn try_from($source_i: TwoFloat) -> Result<Self, Self::Error> $code
        }

        impl<'a> TryFrom<&'a TwoFloat> for $dest {
            type Error = $err;

            fn try_from($source_i: &'a TwoFloat) -> Result<Self, Self::Error> $code
        }
    };
}

macro_rules! float_convert {
    ($type:tt) => {
        impl From<$type> for TwoFloat {
            fn from(value: $type) -> Self {
                TwoFloat { hi: value as f64, lo: 0f64 }
            }
        }

        from_conversion!(|value: TwoFloat| -> $type {
            value.hi as $type
        });
    }
}

float_convert!(f64);
float_convert!(f32);

macro_rules! int_convert {
    ($type:tt) => {
        impl From<$type> for TwoFloat {
            fn from(value: $type) -> Self {
                TwoFloat { hi: value as f64, lo: 0f64 }
            }
        }

        from_conversion!(|value: TwoFloat| -> Result<$type, ()> {
            const LOWER_BOUND: f64 = std::$type::MIN as f64 - 1.0;
            const UPPER_BOUND: f64 = std::$type::MAX as f64 + 1.0;
            if value.hi < LOWER_BOUND || value.hi > UPPER_BOUND {
                Err(())
            } else if value.hi == LOWER_BOUND {
                if value.lo > 0.0 { Ok(std::$type::MIN) } else { Err(()) }
            } else if value.hi == UPPER_BOUND {
                if value.lo < 0.0 { Ok(std::$type::MAX) } else { Err(()) }
            } else if value.hi.fract() == 0.0 {
                if value.hi < 0.0 && value.lo > 0.0 {
                    Ok(value.hi as $type + 1)
                } else if value.hi >= 0.0 && value.lo < 0.0 {
                    Ok(value.hi as $type - 1)
                } else {
                    Ok(value.hi as $type)
                }
            } else {
                Ok(value.hi as $type)
            }
        });
    };
}

int_convert!(i32);
int_convert!(i16);
int_convert!(i8);
int_convert!(u32);
int_convert!(u16);
int_convert!(u8);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::*;

    use rand::Rng;

    randomized_test!(copy_test, |rng: F64Rand| {
        let a = TwoFloat { hi: rng(), lo: rng() };
        let b = a;
        assert_eq!(a.hi, b.hi, "Copy failed for {:?}", a);
        assert_eq!(a.lo, b.lo, "Copy failed for {:?}", a);
    });

    randomized_test!(clone_test, |rng: F64Rand| {
        let a = TwoFloat { hi: rng(), lo: rng() };
        let b = a.clone();
        assert_eq!(a.hi, b.hi, "Clone failed for {:?}", a);
        assert_eq!(a.lo, b.lo, "Clone failed for {:?}", a);
    });

    randomized_test!(equality_test, |rng: F64Rand| {
        let a = TwoFloat { hi: rng(), lo: rng() };
        assert_eq!(a, a);
        assert_eq!(&a, &a);

        let b = TwoFloat { hi: rng(), lo: rng() };
        if a.hi != b.hi || a.lo != b.lo {
            assert_ne!(a, b);
            assert_ne!(&a, &b);
        }
    });

    randomized_test!(equality_f64_test, |rng: F64Rand| {
        let a = rng();
        assert_eq!(TwoFloat { hi: a, lo: 0.0 }, a);
        assert_eq!(a, TwoFloat { hi: a, lo: 0.0 });

        let b = rng();
        if b != 0.0 {
            assert_ne!(TwoFloat { hi: a, lo: b }, a);
            assert_ne!(a, TwoFloat { hi: a, lo: b });
        }
    });

    macro_rules! comparison_test {
        ($val_test: ident, $ref_test: ident, $op: tt, $allow_equal: expr) => {
            randomized_test!($val_test, |rng: F64Rand| {
                let a = TwoFloat { hi: rng(), lo: rng() };
                assert_eq!($allow_equal, a $op a, "Self-comparison using {} failed", stringify!($op));

                let b = TwoFloat { hi: a.hi, lo: rng() };
                assert_eq!(a.lo $op b.lo, a $op b, "Comparison using {} failed", stringify!($op));

                let c = TwoFloat { hi: rng(), lo: a.lo };
                assert_eq!(a.hi $op c.hi, a $op c, "Comparison using {} failed", stringify!($op));
            });

            randomized_test!($ref_test, |rng: F64Rand| {
                let a = TwoFloat { hi: rng(), lo: rng() };
                assert_eq!($allow_equal, &a $op &a, "Self-comparison using {} failed", stringify!($op));

                let b = TwoFloat { hi: a.hi, lo: rng() };
                assert_eq!(a.lo $op b.lo, &a $op &b, "Comparison using {} failed", stringify!($op));

                let c = TwoFloat { hi: rng(), lo: a.lo };
                assert_eq!(a.hi $op c.hi, &a $op &c, "Comparison using {} failed", stringify!($op));
            });
        };
    }

    comparison_test!(less_than_test, less_than_ref_test, <, false);
    comparison_test!(greater_than_test, greater_than_ref_test, >, false);
    comparison_test!(less_equal_test, less_equal_ref_test, <=, true);
    comparison_test!(greater_equal_test, greater_equal_ref_test, >=, true);

    randomized_test!(compare_f64_test, |rng: F64Rand| {
        let a = rng();
        let a_two = TwoFloat { hi: a, lo: 0.0 };
        assert!(a.partial_cmp(&a_two) == Some(Ordering::Equal), "Comparison of f64 <=> TwoFloat failed");
        assert!(a_two.partial_cmp(&a) == Some(Ordering::Equal), "Comparison of TwoFloat <=> f64 failed");
        assert!(a <= a_two);
        assert!(a_two <= a);
        assert!(a >= a_two);
        assert!(a_two >= a);

        let b = rng();
        let ab = TwoFloat { hi: a, lo: b };
        if b < 0.0 {
            assert!(a.partial_cmp(&ab) == Some(Ordering::Greater), "Comparison of f64 <=> TwoFloat failed");
            assert!(a > ab, "Comparison of f64 > TwoFloat failed");
            assert!(a >= ab, "Comparison of f64 >= TwoFloat failed");

            assert!(ab.partial_cmp(&a) == Some(Ordering::Less), "Comparison of TwoFloat <=> f64 failed");
            assert!(ab < a, "Comparison of TwoFloat < f64 failed");
            assert!(ab <= a, "Comparison of TwoFloat <= f64 failed");
        } else if b > 0.0 {
            assert!(a.partial_cmp(&ab) == Some(Ordering::Less), "Comparison of f64 <=> TwoFloat failed");
            assert!(a < ab, "Comparison of f64 < TwoFloat failed");
            assert!(a <= ab, "Comparison of f64 <= TwoFloat failed");

            assert!(ab.partial_cmp(&a) == Some(Ordering::Greater), "Comparison of TwoFloat <=> f64 failed");
            assert!(ab > a, "Comparison of TwoFloat > f64 failed");
            assert!(ab >= a, "Comparison of TwoFloat >= f64 failed");
        }

        let c = rng();
        if c < a {
            assert!(c.partial_cmp(&ab) == Some(Ordering::Less), "Comparison of f64 <=> TwoFloat failed");
            assert!(c < ab, "Comparison of f64 < TwoFloat failed");
            assert!(c <= ab, "Comparison of f64 <= TwoFloat failed");

            assert!(ab.partial_cmp(&c) == Some(Ordering::Greater), "Comparison of TwoFloat <=> f64 failed");
            assert!(ab > c, "Comparison of TwoFloat > f64 failed");
            assert!(ab >= c, "Comparison of TwoFloat >= f64 failed");
        } else if c > a {
            assert!(c.partial_cmp(&ab) == Some(Ordering::Greater), "Comparison of f64 <=> TwoFloat failed");
            assert!(c > ab, "Comparison of f64 > TwoFloat failed");
            assert!(c >= ab, "Comparison of f64 >= TwoFloat failed");

            assert!(ab.partial_cmp(&c) == Some(Ordering::Less), "Comparison of TwoFloat <=> f64 failed");
            assert!(ab < c, "Comparison of TwoFloat < f64 failed");
            assert!(ab <= c, "Comparison of TwoFloat <= f64 failed");
        }
    });

    macro_rules! float_test {
        ($type:tt, $from_test:ident, $into_test:ident) => {
            randomized_test!($from_test, |rng: F64Rand| {
                let source = loop {
                    let source = rng() as $type;
                    if source.is_finite() { break source; };
                };

                let result: TwoFloat = source.into();

                assert_eq!(result.hi, source as f64, "Float conversion failed: mismatch in high word");
                assert_eq!(result.lo, 0f64, "Float conversion failed: non-zero low word");
            });

            randomized_test!($into_test, |rng: F64Rand| {
                let source = TwoFloat { hi: rng(), lo: rng() };
                let source_ref = &source;

                let result: $type = source.into();
                assert_eq!(result, source.hi as $type, "Float conversion from TwoFloat failed");

                let result_ref: $type = source_ref.into();

                assert_eq!(result, result_ref, "Value and reference float conversion give different results");
            });
        };
    }

    float_test!(f64, from_f64_test, into_f64_test);
    float_test!(f32, from_f32_test, into_f32_test);

    fn check_try_from_result<T: std::fmt::Debug + PartialEq>(expected: Result<T, ()>, result: Result<T, ()>, source: TwoFloat) {
        if let Ok(expected_value) = expected {
            assert!(result.is_ok(), "Conversion of {:?} produced error instead of result", source);
            assert_eq!(result.unwrap(), expected_value, "Conversion of {:?} produced incorrect result", source);
        } else {
            assert!(result.is_err(), "Conversion of {:?} produced result instead of error", source);
        }
    }

    macro_rules! from_twofloat_test {
        ($type:tt) => {
            const LOWER_BOUND: f64 = std::$type::MIN as f64 - 1.0;
            const UPPER_BOUND: f64 = std::$type::MAX as f64 + 1.0;

            #[test]
            fn from_twofloat_lower_bound() {
                let mut get_f64 = float_generator();

                for i in 0..TEST_ITERS {
                    let a = LOWER_BOUND;
                    let b = if i == 0 { 0f64 } else { get_valid_f64(&mut get_f64, &|x: f64| { no_overlap(a, x) }) };
                    let source = TwoFloat { hi: a, lo: b };
                    let expected = if b > 0.0 { Ok(std::$type::MIN) } else { Err(()) };
                    let result = $type::try_from(source);

                    check_try_from_result(expected, result, source);

                    let result_ref = $type::try_from(&source);
                    assert_eq!(result, result_ref, "Different value and reference conversions for {:?}", source);
                }
            }

            #[test]
            fn from_twofloat_upper_bound() {
                let mut get_f64 = float_generator();

                for i in 0..TEST_ITERS {
                    let a = UPPER_BOUND;
                    let b = if i == 0 { 0f64 } else { get_valid_f64(&mut get_f64, &|x: f64| { no_overlap(a, x) }) };
                    let source = TwoFloat { hi: a, lo: b };
                    let expected = if b < 0.0 { Ok(std::$type::MAX) } else { Err(()) };
                    let result = $type::try_from(source);

                    check_try_from_result(expected, result, source);

                    let result_ref = $type::try_from(&source);
                    assert_eq!(result, result_ref, "Different value and reference conversions for {:?}", source);
                }
            }

            #[test]
            fn from_twofloat_split_fract() {
                let mut rng = rand::thread_rng();
                let mut get_f64 = float_generator();
                let valid_dist = rand::distributions::Uniform::new(f64::from_bits(LOWER_BOUND.to_bits() - 1), UPPER_BOUND);

                for i in 0..TEST_ITERS {
                    let (a, b) = loop {
                        let a = rng.sample(valid_dist).trunc();
                        let b = if i == 0 { 0f64 } else { get_f64() };
                        if no_overlap(a, b) { break (a, b); }
                    };
                    let source = TwoFloat { hi: a, lo: b };
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
                    assert_eq!(result, result_ref, "Different value and reference conversions for {:?}", source);
                }
            }

            #[test]
            fn from_twofloat_with_fract() {
                let mut rng = rand::thread_rng();
                let mut get_f64 = float_generator();
                let valid_dist = rand::distributions::Uniform::new(f64::from_bits(LOWER_BOUND.to_bits() - 1), UPPER_BOUND);

                for i in 0..TEST_ITERS {
                    let (a, b) = loop {
                        let a = rng.sample(valid_dist);
                        if a.fract() == 0.0 { continue; }
                        let b = if i == 0 { 0f64 } else { get_f64() };
                        if no_overlap(a, b) { break (a, b); }
                    };
                    let source = if i == 1 { TwoFloat { hi: -0.4, lo: 0.0 } } else { TwoFloat { hi: a, lo: b } };
                    let expected = if i == 1 { Ok(0) } else { Ok(a.trunc() as $type) };
                    let result = $type::try_from(source);

                    check_try_from_result(expected, result, source);

                    let result_ref = $type::try_from(&source);
                    assert_eq!(result, result_ref, "Different value and reference conversions for {:?}", source);
                }
            }

            #[test]
            fn from_twofloat_out_of_range() {
                let mut get_f64 = float_generator();

                for _ in 0..TEST_ITERS {
                    let a = get_valid_f64(&mut get_f64, &|x: f64| { x < LOWER_BOUND || x > UPPER_BOUND });
                    let b = get_valid_f64(&mut get_f64, &|x: f64| { no_overlap(a, x) });
                    let source = TwoFloat { hi: a, lo: b};
                    let result = $type::try_from(source);

                    assert!(result.is_err(), "Conversion of {:?} produced value instead of error", source);

                    let result_ref = $type::try_from(&source);
                    assert_eq!(result, result_ref, "Different value and reference conversions for {:?}", source);
                }
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
                    let dist = rand::distributions::Uniform::new_inclusive(std::$type::MIN, std::$type::MAX);
                    for i in 0..TEST_ITERS {
                        let source = match i {
                            0 => std::$type::MIN,
                            1 => std::$type::MAX,
                            _ => rng.sample(dist),
                        };

                        let result: TwoFloat = source.into();

                        assert!(no_overlap(result.hi, result.lo), "Conversion of {} produced overlap", source);
                        assert_eq!(result.hi, source as f64, "Conversion of {} failed: mismatch in high word", source);
                        assert_eq!(result.lo, 0f64, "Conversion of {} failed: non-zero low word", source);
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

                        assert!(no_overlap(result.hi, result.lo), "Conversion of {} produced overlap", source);
                        assert_eq!(result.hi, source as f64, "Conversion of {} failed: mismatch in high word", source);
                        assert_eq!(result.lo, 0f64, "Conversion of {} failed: non-zero low word", source);

                        if source == std::$type::MAX { break; }
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

}
