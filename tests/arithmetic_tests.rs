#![allow(clippy::float_cmp)]
#![allow(clippy::needless_borrows_for_generic_args)]

use core::{convert::TryFrom, fmt::Debug};

use rand::Rng;

use twofloat::{no_overlap, TwoFloat};

#[macro_use]
pub mod common;

use common::{
    get_twofloat, get_valid_ddouble, get_valid_pair, get_valid_twofloat, random_float,
    repeated_test,
};

// Tests for construction from two f64 values

#[test]
fn new_add_test() {
    repeated_test(|| {
        let (a, b) = get_valid_pair(|x, y| (x + y).is_finite());
        let expected = a + b;
        let result = TwoFloat::new_add(a, b);

        assert!(
            result.is_valid(),
            "Result of new_add({:.e}, {:.e}) was invalid",
            a,
            b
        );
        assert_eq!(
            expected,
            result.hi(),
            "Result of new_add({:.e}, {:.e}) had unexpected high word",
            a,
            b
        );
    })
}

#[test]
fn new_sub_test() {
    repeated_test(|| {
        let (a, b) = get_valid_pair(|x, y| (x - y).is_finite());
        let expected = a - b;
        let result = TwoFloat::new_sub(a, b);

        assert!(
            result.is_valid(),
            "Result of new_sub({:.e}, {:.e}) was invalid",
            a,
            b
        );
        assert_eq!(
            expected,
            result.hi(),
            "Result of new_sub({:.e}, {:.e}) had unexpected high word",
            a,
            b
        );
    })
}

#[test]
fn new_mul_test() {
    let (a, b) = get_valid_pair(|x, y| (x * y).is_finite());
    let expected = a * b;
    let result = TwoFloat::new_mul(a, b);

    assert!(
        result.is_valid(),
        "Result of new_mul({:.e}, {:.e}) was invalid",
        a,
        b
    );
    assert_eq!(
        expected,
        result.hi(),
        "Result of new_mul({:.e}, {:.e}) had unexpected high word",
        a,
        b
    );
}

#[test]
fn new_div_test() {
    repeated_test(|| {
        let (a, b) = get_valid_pair(|x, y| (x / y).is_finite());
        let result = TwoFloat::new_div(a, b);

        assert!(
            result.is_valid(),
            "Result of new_div({:.e}, {:.e}) was invalid",
            a,
            b
        );

        assert_eq_ulp!(
            result.hi(),
            a / b,
            10,
            "Incorrect result of new_div({:.e}, {:.e})",
            a,
            b
        );
    });
}

#[test]
fn div_precision_test() {
    repeated_test(|| {
        // Limit the exponent to be less then 1e16 away from the maximal exponent
        // to allow a complete division
        let expected = get_valid_ddouble(|x| x.hi().log10().abs() < 290.0);
        let one: TwoFloat = 1.into();
        // Use f64/TwoFloat
        let result = 1.0 / (1.0 / expected);
        let difference = ((result - expected) / expected).abs();
        assert!(
            difference < 1e-31,
            "1/(1/a) != a (diff {}) ({:.e}, {:.e}) for f64/TwoFloat",
            difference,
            expected,
            result,
        );

        // Use TwoFloat/TwoFloat
        let result = one / (one / expected);
        let difference = ((result - expected) / expected).abs();
        assert!(
            difference < 1e-31,
            "1/(1/a) != a (diff {}) ({:.e}, {:.e}) for TwoFloat/TwoFloat",
            difference,
            expected,
            result,
        );
    })
}

// Test for negation operator

#[test]
fn neg_test() {
    repeated_test(|| {
        let a = get_twofloat();
        let b = -a;

        assert_eq!(a.hi(), -b.hi(), "Negation does not negate high word");
        assert_eq!(a.lo(), -b.lo(), "Negation does not negate low word");

        let c = -b;
        assert_eq!(c, a, "Double negation does not result in original value");

        let b2 = -&a;
        assert_eq!(b, b2, "Mismatch between -TwoFloat and -&TwoFloat");
    });
}

// Helpers for binary operators

enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

impl Operator {
    fn symbol(&self) -> char {
        match self {
            Self::Add => '+',
            Self::Sub => '-',
            Self::Mul => '*',
            Self::Div => '/',
            Self::Rem => '%',
        }
    }

    fn symbol_assign(&self) -> &str {
        match self {
            Self::Add => "+=",
            Self::Sub => "-=",
            Self::Mul => "*=",
            Self::Div => "/=",
            Self::Rem => "%=",
        }
    }

    fn apply<L, R, O>(&self, lhs: L, rhs: R) -> O
    where
        L: num_traits::NumOps<R, O>,
    {
        match self {
            Self::Add => lhs + rhs,
            Self::Sub => lhs - rhs,
            Self::Mul => lhs * rhs,
            Self::Div => lhs / rhs,
            Self::Rem => lhs % rhs,
        }
    }

    fn apply_assign<L, R>(&self, lhs: &mut L, rhs: R)
    where
        L: num_traits::NumAssignOps<R>,
    {
        match self {
            Self::Add => *lhs += rhs,
            Self::Sub => *lhs -= rhs,
            Self::Mul => *lhs *= rhs,
            Self::Div => *lhs /= rhs,
            Self::Rem => *lhs %= rhs,
        }
    }
}

fn diff_test<L, R>(op: &Operator, expected: f64, result: TwoFloat, lhs: L, rhs: R)
where
    L: Debug,
    R: Debug,
{
    match op {
        Operator::Rem => {
            let true_difference: f64 = ((expected - result) / expected).into();
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
                lhs,
                op.symbol(),
                rhs
            );
        }
        _ => {
            let difference = ((expected - result) / expected).abs();
            assert!(
                difference.abs() < 1e-10,
                "Out of range result of {:?} {} {:?}",
                lhs,
                op.symbol(),
                rhs
            );
        }
    }
}

// Tests for binary operators for TwoFloat and f64

fn f64_twofloat_left(op: Operator) {
    let mut rng = rand::thread_rng();
    let high_range = rand::distributions::Uniform::new_inclusive(-1e50, 1e50);
    let low_range = rand::distributions::Uniform::new_inclusive(-1.0, 1.0);

    repeated_test(|| loop {
        let a = rng.sample(high_range);
        let (b, c) = loop {
            let b = rng.sample(high_range);
            let c = rng.sample(low_range);
            if no_overlap(b, c) {
                break (b, c);
            }
        };

        let value = TwoFloat::try_from((b, c)).unwrap();
        let expected = op.apply(a, b);
        if !expected.is_finite() {
            continue;
        }
        let result = op.apply(a, value);
        assert!(
            result.is_valid(),
            "Result of {} {} {:?} was invalid",
            a,
            op.symbol(),
            value
        );

        diff_test(&op, expected, result, a, value);
        break;
    });
}

fn f64_twofloat_right(op: Operator) {
    let mut rng = rand::thread_rng();
    let high_range = rand::distributions::Uniform::new_inclusive(-1e50, 1e50);
    let low_range = rand::distributions::Uniform::new_inclusive(-1.0, 1.0);

    repeated_test(|| loop {
        let a = rng.sample(high_range);
        let (b, c) = loop {
            let b = rng.sample(high_range);
            let c = rng.sample(low_range);
            if no_overlap(b, c) {
                break (b, c);
            }
        };

        let value = TwoFloat::try_from((b, c)).unwrap();
        let expected = op.apply(b, a);
        if !expected.is_finite() {
            continue;
        }
        let result = op.apply(value, a);
        assert!(
            result.is_valid(),
            "Result of {:?} {} {} was invalid",
            value,
            op.symbol(),
            a
        );

        diff_test(&op, expected, result, value, a);
        break;
    });
}

fn f64_twofloat_reversible(op: Operator) {
    if !matches!(op, Operator::Add | Operator::Mul) {
        return;
    }

    let mut rng = rand::thread_rng();
    let high_range = rand::distributions::Uniform::new_inclusive(-1e50, 1e50);
    let low_range = rand::distributions::Uniform::new_inclusive(-1.0, 1.0);

    repeated_test(|| loop {
        let a = rng.sample(high_range);
        let (b, c) = loop {
            let b = rng.sample(high_range);
            let c = rng.sample(low_range);
            if no_overlap(b, c) {
                break (b, c);
            }
        };

        let value = TwoFloat::try_from((b, c)).unwrap();
        let expected = op.apply(a, value);
        if !expected.is_valid() {
            continue;
        }

        let result = op.apply(value, a);
        assert!(
            result.is_valid(),
            "Result of {:?} {} {} was invalid",
            value,
            op.symbol(),
            a
        );
        assert_eq!(
            result,
            expected,
            "Operation {:?} {} {} gave different result to reversed",
            value,
            op.symbol(),
            a
        );
        break;
    });
}

fn f64_twofloat_assign(op: Operator) {
    repeated_test(|| {
        let c = random_float();
        let value = get_valid_twofloat(|x, y| op.apply(x + y, c).is_finite());
        let result1: TwoFloat = op.apply(value, c);
        if result1.is_valid() {
            let result2 = op.apply(&value, c);
            assert!(
                result2.is_valid(),
                "Result validity mismatch between TwoFloat {0} f64 and &TwoFloat {0} f64",
                op.symbol()
            );
            assert_eq!(
                result1,
                result2,
                "Mismatch between TwoFloat {0} f64 and &TwoFloat {0} f64",
                op.symbol()
            );

            let mut result3 = value;
            op.apply_assign(&mut result3, c);
            assert!(
                result3.is_valid(),
                "Result validity mismatch between TwoFloat {} f64 and TwoFloat {} f64",
                op.symbol(),
                op.symbol_assign()
            );
            assert_eq!(
                result1,
                result3,
                "Mismatch between TwoFloat {} f64 and TwoFloat {} f64",
                op.symbol(),
                op.symbol_assign()
            );
        }
    });
}

// Tests for binary operators for TwoFloat and TwoFloat

fn twofloat_op(op: Operator) {
    let mut rng = rand::thread_rng();
    let high_range = rand::distributions::Uniform::new_inclusive(-1e50, 1e50);
    let low_range = rand::distributions::Uniform::new_inclusive(-1.0, 1.0);

    repeated_test(|| loop {
        let (a, b) = loop {
            let x = rng.sample(high_range);
            let y = rng.sample(low_range);
            if no_overlap(x, y) {
                break (x, y);
            }
        };
        let (c, d) = loop {
            let x = rng.sample(high_range);
            let y = rng.sample(low_range);
            if no_overlap(x, y) {
                break (x, y);
            }
        };

        let value1 = TwoFloat::try_from((a, b)).unwrap();
        let value2 = TwoFloat::try_from((c, d)).unwrap();

        let expected = op.apply(a, c);
        if !expected.is_finite() {
            continue;
        }
        let result = op.apply(value1, value2);

        assert!(
            result.is_valid(),
            "Result of {:?} {} {:?} was invalid",
            value1,
            op.symbol(),
            value2
        );

        diff_test(&op, expected, result, value1, value2);
        break;
    });
}

fn twofloat_assign_op(op: Operator) {
    repeated_test(|| {
        let value1 = get_twofloat();
        let a = value1.hi();
        let value2 = get_valid_twofloat(|x, y| op.apply(x + y, a).is_finite());
        let result1 = op.apply(value1, value2);
        if result1.is_valid() {
            let mut result5 = value1;
            op.apply_assign(&mut result5, value2);
            assert!(
                result5.is_valid(),
                "Result validity mismatch between TwoFloat {} TwoFloat and TwoFloat {} TwoFloat",
                op.symbol(),
                op.symbol_assign()
            );
            assert_eq!(
                result1,
                result5,
                "Mismatch between TwoFloat {} TwoFloat and TwoFloat {} TwoFloat",
                op.symbol(),
                op.symbol_assign()
            );
        }
    });
}

macro_rules! op_test {
    ($name:ident($op:expr);) => {
        mod $name {
            use super::Operator;

            #[test]
            fn f64_twofloat_left() {
                super::f64_twofloat_left($op);
            }

            #[test]
            fn f64_twofloat_right() {
                super::f64_twofloat_right($op);
            }

            #[test]
            fn f64_twofloat_reversible() {
                super::f64_twofloat_reversible($op);
            }

            #[test]
            fn f64_twofloat_assign() {
                super::f64_twofloat_assign($op);
            }

            #[test]
            fn twofloat_op() {
                super::twofloat_op($op);
            }

            #[test]
            fn twofloat_assign_op() {
                super::twofloat_assign_op($op);
            }
        }
    };
    ($name:ident($op:expr); $($names:ident($ops:expr);)+) => {
        op_test! { $name($op); }
        op_test! { $($names($ops);)+ }
    };
}

op_test! {
    add_test(Operator::Add);
    sub_test(Operator::Sub);
    mul_test(Operator::Mul);
    div_test(Operator::Div);
    rem_test(Operator::Rem);
}
