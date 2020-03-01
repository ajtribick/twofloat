#![cfg(test)]
#![macro_use]

use rand::Rng;

pub fn float_generator() -> Box<dyn FnMut() -> f64> {
    let mut engine = rand::thread_rng();
    let mantissa_dist = rand::distributions::Uniform::new(0, 1u64 << 52);
    let exponent_dist = rand::distributions::Uniform::new(0, 2047u64);
    Box::new(move || {
        let x = f64::from_bits(engine.sample(mantissa_dist)
                               | (engine.sample(exponent_dist) << 52));
        if engine.gen() { x } else { -x }
    })
}

macro_rules! randomized_test {
    ($test_name:ident, $code:expr) => {
        #[test]
        fn $test_name() {
            let mut rng = float_generator();
            for _ in 0..100000 {
                $code(&mut rng);
            };
        }
    };
}

pub type F64Rand<'a> = &'a mut dyn FnMut() -> f64;

pub fn get_valid_pair<F : Fn(f64, f64) -> bool>(rng: F64Rand, pred: F) -> (f64, f64) {
    loop {
        let a = rng();
        let b = rng();
        if pred(a, b) { return (a, b); };
    }
}

/// Returns the rightmost set bit of a floating point number
fn right_bit(f: f64) -> Option<i16> {
    let fbits = f.to_bits();
    let exponent = ((fbits >> 52) & 0x7ff) as i16 - 1023;
    match exponent {
        -1023 => {
            let mantissa = fbits & 0xfffffffffffff;
            if mantissa == 0 {
                Some(std::i16::MAX)
            } else {
                Some(mantissa.trailing_zeros() as i16 - 1074)
            }
        }
        1024 => None,
        _ => {
            let mantissa = fbits & 0xfffffffffffff | 0x10000000000000;
            Some(mantissa.trailing_zeros() as i16 - 52 + exponent)
        },
    }
}

/// Returns the leftmost set bit of a floating point number
fn left_bit(f: f64) -> Option<i16> {
    let fbits = f.to_bits();
    let exponent = ((fbits >> 52) & 0x7ff) as i16 - 1023;
    match exponent {
        -1023 => {
            let mantissa = fbits & 0xfffffffffffff;
            if mantissa == 0 {
                Some(std::i16::MIN)
            } else {
                Some(-1011 - mantissa.leading_zeros() as i16)
            }
        }
        1024 => None,
        _ => Some(exponent),
    }
}

pub fn no_overlap(a: f64, b: f64) -> bool {
    match (right_bit(a), left_bit(b)) {
        (Some(r), Some(l)) => r > l,
        _ => false,
    }
}

pub fn ulp_diff(a: f64, b: f64) -> i64 {
    let a_bits = a.to_bits();
    let b_bits = b.to_bits();
    let fix_sign = |x: u64| { if x & 0x8000000000000000 == 0 { x } else { x ^ 0x7fffffffffffffff } };
    (fix_sign(a_bits) as i64).saturating_sub(fix_sign(b_bits) as i64)
}

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

#[test]
fn right_bit_test() {
    assert_eq!(right_bit(std::f64::INFINITY), None);
    assert_eq!(right_bit(std::f64::NEG_INFINITY), None);
    assert_eq!(right_bit(std::f64::NAN), None);
    assert_eq!(right_bit(1f64), Some(0));
    assert_eq!(right_bit(2f64), Some(1));
    assert_eq!(right_bit(0.5f64), Some(-1));
    assert_eq!(right_bit(2.2250738585072014e-308), Some(-1022));
    assert_eq!(right_bit(2.2250738585072009e-308), Some(-1074));
    assert_eq!(right_bit(4.9406564584124654e-324), Some(-1074));
    assert!(right_bit(0f64).unwrap_or(0) > 1024);
}

#[test]
fn left_bit_test() {
    assert_eq!(left_bit(std::f64::INFINITY), None);
    assert_eq!(left_bit(std::f64::NEG_INFINITY), None);
    assert_eq!(left_bit(std::f64::NAN), None);
    assert_eq!(left_bit(1f64), Some(0));
    assert_eq!(left_bit(2f64), Some(1));
    assert_eq!(left_bit(0.5f64), Some(-1));
    assert_eq!(right_bit(2.2250738585072014e-308), Some(-1022));
    assert_eq!(left_bit(2.2250738585072009e-308), Some(-1023));
    assert_eq!(left_bit(4.9406564584124654e-324), Some(-1074));
    assert!(left_bit(0f64).unwrap_or(0) < -1074);
}
