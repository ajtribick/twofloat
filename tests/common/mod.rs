use core::convert::TryFrom;

use rand::Rng;

use twofloat::{TwoFloat, TwoFloatError};

// This runs substantially slower than the other CI targets, so reduce the
// number of iterations
#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
const TEST_ITERS: usize = 10000;

#[cfg(not(all(target_arch = "aarch64", target_os = "linux")))]
const TEST_ITERS: usize = 100_000;

pub fn random_float() -> f64 {
    let mut engine = rand::thread_rng();
    let mantissa_dist = rand::distributions::Uniform::new(0, 1u64 << 52);
    let exponent_dist = rand::distributions::Uniform::new(0, 2047u64);
    let x = f64::from_bits(engine.sample(mantissa_dist) | (engine.sample(exponent_dist) << 52));
    if engine.gen() {
        x
    } else {
        -x
    }
}

pub fn random_ddouble() -> TwoFloat {
    let mut rng = rand::thread_rng();
    let mantissa_bits_hi: u64 =
        (0..f64::MANTISSA_DIGITS).fold(0u64, |bits, n| bits + (rng.gen_range(0..=1) << n));
    let mantissa_bits_lo: u64 =
        (0..f64::MANTISSA_DIGITS).fold(0u64, |bits, n| bits + (rng.gen_range(0..=1) << n));
    let exponent_bits: u64 = (0..11)
        .fold(0u64, |bits, n| bits + (rng.gen_range(0..=1) << n))
        .max(53);
    let sgn: u64 = rng.gen_range(0..=1);
    let x_hi = f64::from_bits(sgn << 63 | exponent_bits << 52 | mantissa_bits_hi);
    let x_lo = f64::from_bits(sgn << 63 | (exponent_bits - 53) << 52 | mantissa_bits_lo);
    TwoFloat::new_add(x_hi, x_lo)
}

pub fn repeated_test(mut test: impl FnMut()) {
    for _ in 0..TEST_ITERS {
        test();
    }
}

pub fn repeated_test_enumerate(mut test: impl FnMut(usize)) {
    for i in 0..TEST_ITERS {
        test(i);
    }
}

pub fn get_valid_f64<F>(pred: F) -> f64
where
    F: Fn(f64) -> bool,
{
    loop {
        let a = random_float();
        if pred(a) {
            return a;
        }
    }
}

pub fn get_valid_f64_gen<G, F>(mut gen: G, pred: F) -> f64
where
    G: FnMut() -> f64,
    F: Fn(f64) -> bool,
{
    loop {
        let a = gen();
        if pred(a) {
            return a;
        }
    }
}

pub fn get_twofloat() -> TwoFloat {
    loop {
        if let Ok(result) = TwoFloat::try_from((random_float(), random_float())) {
            return result;
        }
    }
}

pub fn try_get_twofloat_with_hi(hi: f64) -> Result<TwoFloat, TwoFloatError> {
    if hi == 0.0 {
        return Ok(TwoFloat::from(0.0));
    }

    for _ in 0..10 {
        let result = TwoFloat::try_from((hi, random_float() % hi));
        if result.is_ok() {
            return result;
        }
    }

    Err(TwoFloatError::ConversionError {})
}

pub fn try_get_twofloat_with_lo(lo: f64) -> Result<TwoFloat, TwoFloatError> {
    for _ in 0..10 {
        let result = TwoFloat::try_from((random_float(), lo));
        if result.is_ok() {
            return result;
        }
    }

    Err(TwoFloatError::ConversionError {})
}

pub fn get_valid_twofloat<F>(pred: F) -> TwoFloat
where
    F: Fn(f64, f64) -> bool,
{
    loop {
        let a = random_float();
        let b = random_float();
        if !pred(a, b) {
            continue;
        }

        if let Ok(result) = TwoFloat::try_from((a, b)) {
            return result;
        }
    }
}

pub fn get_valid_pair<F>(pred: F) -> (f64, f64)
where
    F: Fn(f64, f64) -> bool,
{
    loop {
        let a = random_float();
        let b = random_float();
        if pred(a, b) {
            return (a, b);
        }
    }
}

pub fn get_valid_ddouble<F>(pred: F) -> TwoFloat
where
    F: Fn(TwoFloat) -> bool,
{
    loop {
        let a = random_ddouble();
        if pred(a) {
            return a;
        }
    }
}

#[allow(unused_macros)]
macro_rules! assert_eq_ulp {
    ($left:expr, $right:expr, $ulp:expr) => ({
        let left_val = $left;
        let right_val = $right;
        let ulp_val = $ulp;

        let a_bits = left_val.to_bits();
        let b_bits = right_val.to_bits();
        let fix_sign = |x| {
            if x & (1 << 63) == 0 {
                x
            } else {
                x ^ ((1 << 63) - 1)
            }
        };
        let diff = (fix_sign(a_bits) as i64)
            .saturating_sub(fix_sign(b_bits) as i64)
            .abs();
        if !(diff <= *ulp_val) {
            panic!(r#"assertion failed: `(left == right) ({:?} ulp)`
  left: `{:?}`,
 right: `{:?}`,
  diff: `{}`"#, ulp_val, left_val, right_val, diff)
        }
    });
    ($left:expr, $right:expr, $ulp:expr, $($args:tt,)+) => ({
        let left_val = $left;
        let right_val = $right;
        let ulp_val = $ulp;

        let a_bits = left_val.to_bits();
        let b_bits = right_val.to_bits();
        let fix_sign = |x| {
            if x & (1 << 63) == 0 {
                x
            } else {
                x ^ ((1 << 63) - 1)
            }
        };
        let diff = (fix_sign(a_bits) as i64)
            .saturating_sub(fix_sign(b_bits) as i64)
            .abs();
        if !(diff <= ulp_val) {
            panic!(r#"assertion failed: `(left == right) ({:?} ulp)`
  left: `{:?}`,
 right: `{:?}`,
  diff: `{}`: {}"#, ulp_val, left_val, right_val, diff, format_args!($($args,)+))
        }
    });
    ($left:expr, $right:expr, $ulp:expr, $($args:tt),+) => {
        assert_eq_ulp!($left, $right, $ulp, $($args,)+)
    };
}
