#![macro_use]

use rand::Rng;
use twofloat::TwoFloat;

pub const TEST_ITERS: usize = 100000;

pub fn float_generator() -> impl FnMut() -> f64 {
    let mut engine = rand::thread_rng();
    let mantissa_dist = rand::distributions::Uniform::new(0, 1u64 << 52);
    let exponent_dist = rand::distributions::Uniform::new(0, 2047u64);
    move || {
        let x = f64::from_bits(engine.sample(mantissa_dist) | (engine.sample(exponent_dist) << 52));
        if engine.gen() {
            x
        } else {
            -x
        }
    }
}

macro_rules! randomized_test {
    ($test_name:ident, $code:expr) => {
        #[test]
        fn $test_name() {
            let mut rng = float_generator();
            for _ in 0..TEST_ITERS {
                $code(&mut rng);
            }
        }
    };
}

pub type F64Rand<'a> = &'a mut dyn FnMut() -> f64;

pub fn get_valid_f64<F: Fn(f64) -> bool>(rng: F64Rand, pred: F) -> f64 {
    loop {
        let a = rng();
        if pred(a) {
            return a;
        }
    }
}

pub fn get_twofloat(rng: F64Rand) -> TwoFloat {
    loop {
        if let Ok(result) = TwoFloat::try_new(rng(), rng()) {
            return result;
        }
    }
}

pub fn try_get_twofloat_with_hi(rng: F64Rand, hi: f64) -> Result<TwoFloat, ()> {
    if hi == 0.0 {
        return Ok(TwoFloat::from(0.0));
    }

    for _ in 0..10 {
        let result = TwoFloat::try_new(hi, rng() % hi);
        if result.is_ok() {
            return result;
        }
    }

    Err(())
}

pub fn try_get_twofloat_with_lo(rng: F64Rand, lo: f64) -> Result<TwoFloat, ()> {
    for _ in 0..10 {
        let result = TwoFloat::try_new(rng(), lo);
        if result.is_ok() {
            return result;
        }
    }

    Err(())
}

pub fn get_valid_twofloat<F: Fn(f64, f64) -> bool>(rng: F64Rand, pred: F) -> TwoFloat {
    loop {
        let a = rng();
        let b = rng();
        if !pred(a, b) {
            continue;
        }

        if let Ok(result) = TwoFloat::try_new(a, b) {
            return result;
        }
    }
}

pub fn get_valid_pair<F: Fn(f64, f64) -> bool>(rng: F64Rand, pred: F) -> (f64, f64) {
    loop {
        let a = rng();
        let b = rng();
        if pred(a, b) {
            return (a, b);
        }
    }
}

pub fn ulp_diff(a: f64, b: f64) -> i64 {
    let a_bits = a.to_bits();
    let b_bits = b.to_bits();
    let fix_sign = |x| {
        if x & (1 << 63) == 0 {
            x
        } else {
            x ^ ((1 << 63) - 1)
        }
    };
    (fix_sign(a_bits) as i64).saturating_sub(fix_sign(b_bits) as i64)
}

#[allow(unused_macros)]
macro_rules! assert_eq_ulp {
    ($left:expr, $right:expr, $ulp:expr) => ({
        match (&$left, &$right, &$ulp) {
            (left_val, right_val, ulp_val) => {
                let diff = ulp_diff(*left_val, *right_val).abs();
                if !(diff <= *ulp_val) {
                    panic!(r#"assertion failed: `(left == right) ({:?} ulp)`
  left: `{:?}`,
 right: `{:?}`,
  diff: `{}`"#, &*ulp_val, &*left_val, &*right_val, diff)
                }
            }
        }
    });
    ($left:expr, $right:expr, $ulp:expr,) => ({
        assert_eq_ulp!($left, $right, $ulp);
    });
    ($left:expr, $right:expr, $ulp:expr, $($arg:tt)+) => ({
        match (&$left, &$right, &$ulp) {
            (left_val, right_val, ulp_val) => {
                let diff = ulp_diff(*left_val, *right_val).abs();
                if !(ulp_diff(*left_val, *right_val).abs() <= *ulp_val) {
                    panic!(r#"assertion failed: `(left == right) ({:?} ulp)`
  left: `{:?}`,
 right: `{:?}`,
  diff: `{}`: {}"#, &*ulp_val, &*left_val, &*right_val, diff, format_args!($($arg)+))
                }
            }
        }
    });
}
