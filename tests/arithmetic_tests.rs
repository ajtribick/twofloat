use twofloat::*;
use rand::Rng;

pub mod common;
use common::*;

// Tests for construction from two f64 values

randomized_test!(new_add_test, |rng: F64Rand| {
    let (a, b) = get_valid_pair(rng, |x, y| (x + y).is_finite());
    let expected = a + b;
    let (hi, lo) = TwoFloat::new_add(a, b).data();

    assert!(no_overlap(hi, lo), "Result of new_add({}, {}) had overlap", a, b);
    assert_eq!(expected, hi, "Result of new_add({}, {}) had unexpected high word", a, b);
});

randomized_test!(new_sub_test, |rng: F64Rand| {
    let (a, b) = get_valid_pair(rng, |x, y| (x - y).is_finite());
    let expected = a - b;
    let (hi, lo) = TwoFloat::new_sub(a, b).data();

    assert!(no_overlap(hi, lo), "Result of new_sub({}, {}) had overlap", a, b);
    assert_eq!(expected, hi, "Result of new_sub({}, {}) had unexpected high word", a, b);
});

randomized_test!(new_mul_test, |rng: F64Rand| {
    let (a, b) = get_valid_pair(rng, |x, y| (x * y).is_finite());
    let expected = a * b;
    let (hi, lo) = TwoFloat::new_mul(a, b).data();

    assert!(no_overlap(hi, lo), "Result of new_mul({}, {}) had overlap", a, b);
    assert_eq!(expected, hi, "Result of new_mul({}, {}) had unexpected high word", a, b);
});

randomized_test!(new_div_test, |rng: F64Rand| {
    let (a, b) = get_valid_pair(rng, |x, y| (x / y).is_finite());
    let (hi, lo) = TwoFloat::new_div(a, b).data();

    assert!(
        no_overlap(hi, lo),
        "Overlapping bits in new_div({}, {})",
        a,
        b
    );

    assert_eq_ulp!(
        hi,
        a / b,
        10,
        "Incorrect result of new_div({}, {})",
        a,
        b
    );
});

// Test for negation operator

randomized_test!(neg_test, |rng: F64Rand| {
    let a = get_twofloat(rng);
    let (a_hi, a_lo) = a.data();

    let b = -a;
    let (b_hi, b_lo) = b.data();

    assert_eq!(a_hi, -b_hi, "Negation does not negate high word");
    assert_eq!(a_lo, -b_lo, "Negation does not negate low word");

    let c = -b;
    assert_eq!(c, a, "Double negation does not result in original value");

    let b2 = -&a;
    assert_eq!(b, b2, "Mismatch between -TwoFloat and -&TwoFloat");
});

// Tests for binary operators for TwoFloat and f64

macro_rules! diff_test {
    (%, $expected:ident, $result:ident, $lhs:ident, $rhs:ident) => {{
        let true_difference: f64 = (($expected - $result) / $expected).into();
        let differences: [f64; 3] = [
            true_difference.abs(),
            (1.0 - true_difference).abs(),
            (1.0 + true_difference).abs(),
        ];
        let min_difference = *differences
            .iter()
            .min_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap();
        assert!(
            min_difference < 1e-10,
            "Out of range result of {:?} {} {:?}",
            $lhs,
            stringify!($op),
            $rhs
        );
    }};
    ($op:tt, $expected:ident, $result:ident, $lhs:ident, $rhs:ident) => {{
        let difference = (($expected - $result) / $expected).abs();
        assert!(
            difference.abs() < 1e-10,
            "Out of range result of {:?} {} {:?}",
            $lhs,
            stringify!($op),
            $rhs
        );
    }};
}

macro_rules! op_test_f64_common {
    ($op:tt, $op_assign:tt) => {
        randomized_test!(ref_overloads_left_test, |rng: F64Rand| {
            let c = rng();
            let value = get_valid_twofloat(rng, |x, y| { ((x + y) $op c).is_finite() });
            let result1 = value $op c;
            if result1.is_valid() {
                let result2 = &value $op c;
                assert!(result2.is_valid(), "Result validity mismatch between TwoFloat {0} f64 and &TwoFloat {0} f64", stringify!($op));
                assert_eq!(result1, result2, "Mismatch between TwoFloat {0} f64 and &TwoFloat {0} f64", stringify!($op));

                let mut result3 = value;
                result3 $op_assign c;
                assert!(result3.is_valid(), "Result validity mismatch between TwoFloat {} f64 and TwoFloat {} f64", stringify!($op), stringify!($op_assign));
                assert_eq!(result1, result3, "Mismatch between TwoFloat {} f64 and TwoFloat {} f64", stringify!($op), stringify!($op_assign));
            }
        });

        randomized_test!(ref_overloads_right_test, |rng: F64Rand| {
            let c = rng();
            let value = get_valid_twofloat(rng, |x, y| { (c $op (x + y)).is_finite() });
            let result1 = c $op value;
            if result1.is_valid() {
                let result2 = c $op &value;
                assert!(result2.is_valid(), "Result validity mismatch between f64 {0} TwoFloat and &f64 {0} &TwoFloat", stringify!($op));
                assert_eq!(result1, result2, "Mismatch between f64 {0} TwoFloat and &TwoFloat {0} &TwoFloat", stringify!($op));
            }
        });

        #[test]
        fn value_left_test() {
            let mut rng = rand::thread_rng();
            let high_range = rand::distributions::Uniform::new_inclusive(-1e50, 1e50);
            let low_range = rand::distributions::Uniform::new_inclusive(-1.0, 1.0);

            loop {
                let a = rng.sample(high_range);
                let (b, c) = loop {
                    let b = rng.sample(high_range);
                    let c = rng.sample(low_range);
                    if no_overlap(b, c) { break (b, c); }
                };

                let value = TwoFloat::try_new(b, c).unwrap();
                let expected = a $op b;
                if (!expected.is_finite()) { continue; }
                let result = a $op value;
                if (!result.is_valid()) { continue; }

                let (hi, lo) = result.data();
                assert!(no_overlap(hi, lo), "Result of {} {} {:?} contained overlap", a, stringify!($op), value);

                diff_test!($op, expected, result, a, value);
                break;
            }
        }
    };
}

macro_rules! op_test_f64 {
    ($test_name:ident, $op:tt, $op_assign:tt, true) => {
        #[cfg(test)]
        mod $test_name {
            use super::*;

            op_test_f64_common!($op, $op_assign);

            #[test]
            fn reversible_test() {
                let mut rng = rand::thread_rng();
                let high_range = rand::distributions::Uniform::new_inclusive(-1e50, 1e50);
                let low_range = rand::distributions::Uniform::new_inclusive(-1.0, 1.0);
                loop {
                    let a = rng.sample(high_range);
                    let (b, c) = loop {
                        let b = rng.sample(high_range);
                        let c = rng.sample(low_range);
                        if no_overlap(b, c) { break (b, c); }
                    };

                    let value = TwoFloat::try_new(b, c).unwrap();
                    let expected = a $op value;
                    if (!expected.is_valid()) { continue; }

                    let result = value $op a;
                    let (hi, lo) = result.data();
                    assert!(no_overlap(hi, lo), "Result of {:?} {} {} contained overlap", value, stringify!($op), a);
                    assert_eq!(result, expected, "Operation {:?} {} {} gave different result to reversed", value, stringify!($op), a);
                    break;
                }
            }
        }
    };
    ($test_name:ident, $op:tt, $op_assign:tt, false) => {
        #[cfg(test)]
        mod $test_name {
            use super::*;

            op_test_f64_common!($op, $op_assign);

            #[test]
            fn value_right_test() {
                let mut rng = rand::thread_rng();
                let high_range = rand::distributions::Uniform::new_inclusive(-1e50, 1e50);
                let low_range = rand::distributions::Uniform::new_inclusive(-1.0, 1.0);
                loop {
                    let a = rng.sample(high_range);
                    let (b, c) = loop {
                        let b = rng.sample(high_range);
                        let c = rng.sample(low_range);
                        if no_overlap(b, c) { break (b, c); }
                    };

                    let value = TwoFloat::try_new(b, c).unwrap();
                    let expected = b $op a;
                    if (!expected.is_finite()) { continue; }
                    let result = value $op a;
                    if (!result.is_valid()) { continue; }

                    let (hi, lo) = result.data();
                    assert!(no_overlap(hi, lo), "Result of {:?} {} {} contained overlap", value, stringify!($op), a);

                    diff_test!($op, expected, result, value, a);
                    break;
                }
            }
        }
    };
}

op_test_f64!(add_f64_test, +, +=, true);
op_test_f64!(sub_f64_test, -, -=, false);
op_test_f64!(mul_f64_test, *, *=, true);
op_test_f64!(div_f64_test, /, /=, false);
op_test_f64!(rem_f64_test, %, %=, false);

// Tests for binary operators for TwoFloat and TwoFloat

macro_rules! op_test {
    ($test_name:ident, $op:tt, $op_assign:tt) => {
        #[cfg(test)]
        mod $test_name {
            use super::*;

            randomized_test!(ref_overloads_test, |rng: F64Rand| {
                let value1 = get_twofloat(rng);
                let a = value1.data().0;
                let value2 = get_valid_twofloat(rng, |x, y| { ((x + y) $op a).is_finite() });
                let result1 = value1 $op value2;
                if result1.is_valid() {
                    let result2 = &value1 $op value2;
                    assert!(result2.is_valid(), "Result validity mismatch between TwoFloat {0} TwoFloat and &TwoFloat {0} TwoFloat", stringify!($op));
                    assert_eq!(result1, result2, "Mismatch between TwoFloat {0} TwoFloat and &TwoFloat {0} TwoFloat", stringify!($op));

                    let result3 = value1 $op &value2;
                    assert!(result3.is_valid(), "Result validity mismatch between TwoFloat {0} TwoFloat and TwoFloat {0} &TwoFloat", stringify!($op));
                    assert_eq!(result1, result3, "Mismatch between TwoFloat {0} TwoFloat and TwoFloat {0} &TwoFloat", stringify!($op));

                    let result4 = &value1 $op &value2;
                    assert!(result4.is_valid(), "Result validity mismatch between TwoFloat {0} TwoFloat and &TwoFloat {0} &TwoFloat", stringify!($op));
                    assert_eq!(result1, result4, "Mismatch between TwoFloat {0} TwoFloat and &TwoFloat {0} &TwoFloat", stringify!($op));

                    let mut result5 = value1;
                    result5 $op_assign value2;
                    assert!(result5.is_valid(), "Result validity mismatch between TwoFloat {} TwoFloat and TwoFloat {} TwoFloat", stringify!($op), stringify!($op_assign));
                    assert_eq!(result1, result5, "Mismatch between TwoFloat {} TwoFloat and TwoFloat {} TwoFloat", stringify!($op), stringify!($op_assign));

                    let mut result6 = value1;
                    result6 $op_assign &value2;
                    assert!(result6.is_valid(), "Result validity mismatch between TwoFloat {} TwoFloat and TwoFloat {} &TwoFloat", stringify!($op), stringify!($op_assign));
                    assert_eq!(result1, result6, "Mismatch between TwoFloat {} TwoFloat and TwoFloat {} &TwoFloat", stringify!($op), stringify!($op_assign));
                }
            });

            #[test]
            fn value_test() {
                let mut rng = rand::thread_rng();
                let high_range = rand::distributions::Uniform::new_inclusive(-1e50, 1e50);
                let low_range = rand::distributions::Uniform::new_inclusive(-1.0, 1.0);

                loop {
                    let (a, b) = loop {
                        let x = rng.sample(high_range);
                        let y = rng.sample(low_range);
                        if no_overlap(x, y) { break (x, y); }
                    };
                    let (c, d) = loop {
                        let x = rng.sample(high_range);
                        let y = rng.sample(low_range);
                        if no_overlap(x, y) { break (x, y); }
                    };

                    let value1 = TwoFloat::try_new(a, b).unwrap();
                    let value2 = TwoFloat::try_new(c, d).unwrap();

                    let expected = a $op c;
                    if (!expected.is_finite()) { continue; }
                    let result = value1 $op value2;
                    if (!result.is_valid()) { continue; }

                    let (hi, lo) = result.data();
                    assert!(no_overlap(hi, lo), "Result of {:?} {} {:?} contained overlap", value1, stringify!($op), value2);

                    diff_test!($op, expected, result, value1, value2);
                    break;
                }
            }
        }
    };
}

op_test!(add_test, +, +=);
op_test!(sub_test, -, -=);
op_test!(mul_test, *, *=);
op_test!(div_test, /, /=);
op_test!(rem_test, %, %=);
