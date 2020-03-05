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

macro_rules! float_convert {
    ($type:tt) => {
        impl From<$type> for TwoFloat {
            fn from(value: $type) -> Self {
                TwoFloat { hi: value as f64, lo: 0f64 }
            }
        }

        impl From<TwoFloat> for $type {
            fn from(value: TwoFloat) -> Self {
                value.hi as $type
            }
        }

        impl<'a> From<&'a TwoFloat> for $type {
            fn from(value: &'a TwoFloat) -> Self {
                value.hi as $type
            }
        }
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

        impl TryFrom<TwoFloat> for $type {
            type Error = ();

            fn try_from(value: TwoFloat) -> Result<Self, Self::Error> {
                if value.hi >= std::$type::MIN as f64 - 1.0 && value.hi <= std::$type::MAX as f64 + 1.0 { Ok(value.hi as $type) } else { Err(()) }
            }
        }

        impl<'a> TryFrom<&'a TwoFloat> for $type {
            type Error = ();

            fn try_from(value: &'a TwoFloat) -> Result<Self, Self::Error> {
                if value.hi >= std::$type::MIN as f64 - 1.0 && value.hi <= std::$type::MAX as f64 + 1.0 { Ok(value.hi as $type) } else { Err(()) }
            }
        }
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

    macro_rules! into_int_test {
        ($type:tt, $into_test:ident) => {
            #[test]
            fn $into_test() {
                let mut rng = rand::thread_rng();
                let lower_bound = f64::from_bits((std::$type::MIN as f64 - 1.0).to_bits() - 1);
                let upper_bound = f64::from_bits((std::$type::MAX as f64 + 1.0).to_bits() - 1);
                let valid_dist = rand::distributions::Uniform::new_inclusive(lower_bound, upper_bound);
                let mantissa_dist = rand::distributions::Uniform::new(0, 1u64 << 52);
                let exponent_dist = rand::distributions::Uniform::new(0, 2047u64);

                let mut get_f64 = move || {
                    let x_pos = f64::from_bits(rng.sample(mantissa_dist) | (rng.sample(exponent_dist) << 52));
                    return if rng.gen() { x_pos } else { -x_pos }
                };

                for _ in 0..TEST_ITERS {
                    let in_range: bool = rng.gen();

                    let a = if in_range {
                        rng.sample(valid_dist)
                    } else {
                        loop {
                            let x = get_f64();
                            if x < lower_bound || x > upper_bound { break x; }
                        }
                    };

                    let b = loop {
                        let x = get_f64();
                        if no_overlap(a, x) { break x; }
                    };

                    let source = TwoFloat { hi: a, lo: b };
                    let result = $type::try_from(source);

                    if in_range {
                        assert!(result.is_ok(), "Conversion to integer failed");
                        assert_eq!(result.unwrap(), a as $type, "Conversion to integer failed: value mismatch");
                    } else {
                        assert!(result.is_err(), "Conversion from out-of-range value did not produce error case");
                    }
                }
            }
        };
    }

    macro_rules! int_test {
        ($type:tt, $from_test:ident, $into_test:ident, false) => {
            #[test]
            fn $from_test() {
                let mut rng = rand::thread_rng();
                let dist = rand::distributions::Uniform::new_inclusive(std::$type::MIN, std::$type::MAX);
                for _ in 0..TEST_ITERS {
                    let source = rng.sample(dist);
                    let result: TwoFloat = source.into();

                    assert_eq!(result.hi, source as f64, "Integer conversion failed: mismatch in high word");
                    assert_eq!(result.lo, 0f64, "Integer conversion failed: non-zero low word");
                }
            }

            into_int_test!($type, $into_test);
        };
        ($type:tt, $from_test:ident, $into_test:ident, true) => {
            #[test]
            fn $from_test() {
                let mut source = std::$type::MIN;
                loop {
                    let result: TwoFloat = source.into();

                    assert_eq!(result.hi, source as f64, "Integer conversion failed: mismatch in high word");
                    assert_eq!(result.lo, 0f64, "Integer conversion failed: non-zero low word");

                    if source == std::$type::MAX { break; }
                    source += 1;
                }
            }

            into_int_test!($type, $into_test);
        }
    }

    int_test!(i32, from_i32_test, into_i32_test, false);
    int_test!(i16, from_i16_test, into_i16_test, true);
    int_test!(i8, from_i8_test, into_i8_test, true);
    int_test!(u32, from_u32_test, into_u32_test, false);
    int_test!(u16, from_u16_test, into_u16_test, true);
    int_test!(u8, from_u8_test, into_u8_test, true);
}
