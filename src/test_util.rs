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
    ($test_name: ident, $code: expr) => {
        #[test]
        fn $test_name() {
            let mut create_float = float_generator();
            for _ in 0..10000 {
                $code(&mut create_float);
            }
        }
    };
}

pub type F64Rand<'a> = &'a mut dyn FnMut() -> f64;
