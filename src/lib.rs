#[derive(Debug, PartialEq, Copy, Clone)]
pub struct TwoFloat {
    hi: f64,
    lo: f64,
}

impl TwoFloat {
    fn new(a: f64, b: f64) -> TwoFloat {
        TwoFloat { hi: a, lo: b }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    const TEST_ITERS: usize = 10000;

    fn float_generator() -> Box<dyn FnMut() -> f64> {
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
                for _ in 0..TEST_ITERS {
                    $code(&mut create_float);
                }
            }
        };
    }

    type F64Rand<'a> = &'a mut dyn FnMut() -> f64;

    randomized_test!(new_test, |rng: F64Rand| {
        let a = rng();
        let b = rng();
        let value = TwoFloat::new(a, b);
        assert_eq!(value.hi, a);
        assert_eq!(value.lo, b);
    });

    randomized_test!(copy_test, |rng: F64Rand| {
        let a = TwoFloat::new(rng(), rng());
        let b = a;
        assert_eq!(a.hi, b.hi);
        assert_eq!(a.lo, b.lo);
    });

    randomized_test!(clone_test, |rng: F64Rand| {
        let a = TwoFloat::new(rng(), rng());
        let b = a.clone();
        assert_eq!(a.hi, b.hi);
        assert_eq!(a.lo, b.lo);
    });

    randomized_test!(equality_test, |rng: F64Rand| {
        let a = TwoFloat::new(rng(), rng());
        let b = a.clone();
        assert_eq!(a, b);
    });

    randomized_test!(equality_ref_test, |rng: F64Rand| {
        let a = TwoFloat::new(rng(), rng());
        let b = a.clone();
        assert_eq!(&a, &b);
    });

    randomized_test!(inequality_test, |rng: F64Rand| {
        let a = TwoFloat::new(rng(), rng());
        let b = loop {
            let b = TwoFloat::new(rng(), rng());
            if b.hi != a.hi || b.lo != a.lo { break b; };
        };
        assert_ne!(a, b);
    });

    randomized_test!(inequality_ref_test, |rng: F64Rand| {
        let a = TwoFloat::new(rng(), rng());
        let b = loop {
            let b = TwoFloat::new(rng(), rng());
            if b.hi != a.hi || b.lo != a.lo { break b; };
        };
        assert_ne!(&a, &b);
    });
}
