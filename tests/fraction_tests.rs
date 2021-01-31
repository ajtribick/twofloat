#![allow(clippy::float_cmp)]

use core::convert::TryFrom;
use twofloat::{no_overlap, TwoFloat};

#[macro_use]
pub mod common;

use common::*;

// fract() tests

#[test]
fn fract_hi_fract_test() {
    repeated_test(|| {
        let source = get_valid_twofloat(|x, _| x.fract() != 0.0);
        let expected = source.hi().fract() + source.lo().fract();
        let result = source.fract();
        assert!(
            result.is_valid(),
            "fract({:?}) produced invalid value",
            source
        );
        assert!(
            result.hi().trunc() == 0.0
                || (result.hi().trunc().abs() == 1.0
                    && ((result.hi() >= 0.0) != (result.lo() >= 0.0))),
            "Fractional part of {:?} is greater than one",
            source
        );
        assert!(
            result.lo().trunc() == 0.0,
            "Non-zero integer part of low word of fract({:?}",
            source
        );
        assert_eq_ulp!(
            result.hi(),
            expected,
            1,
            "Mismatch in fractional part of {:?}",
            source
        );
    });
}

#[test]
fn fract_lo_fract_test() {
    repeated_test(|| {
        let (a_fract, b) = get_valid_pair(|x, y| y.fract() != 0.0 && no_overlap(x.trunc(), y));
        let a = a_fract.trunc();
        let source = TwoFloat::try_from((a, b)).unwrap();
        let expected = match (a >= 0.0, b >= 0.0) {
            (true, false) => 1.0 + b.fract(),
            (false, true) => -1.0 + b.fract(),
            _ => b.fract(),
        };
        let result = source.fract();
        assert!(
            result.is_valid(),
            "fract({:?}) produced invalid value",
            source
        );
        assert!(
            result.hi().trunc() == 0.0
                || (result.hi().trunc().abs() == 1.0
                    && ((result.hi() >= 0.0) != (result.lo() >= 0.0))),
            "Fractional part of {:?} is greater than one",
            source
        );
        assert!(
            result.lo().trunc() == 0.0,
            "Non-zero integer part of low word of fract({:?}",
            source
        );
        assert_eq_ulp!(
            result.hi(),
            expected,
            1,
            "Mismatch in fractional part of {:?}",
            source
        );
    });
}

#[test]
fn fract_no_lo_word_test() {
    repeated_test(|| {
        let a = random_float();
        let source = TwoFloat::from(a);
        let expected = a.fract();
        let result = source.fract();

        assert!(
            result.is_valid(),
            "fract({:?}) produced invalid value",
            source
        );
        assert_eq!(
            result, expected,
            "fract({:?}) produced incorrect value",
            source
        );
    })
}

#[test]
fn fract_no_fract_test() {
    repeated_test(|| {
        let (a_fract, b_fract) = get_valid_pair(|x, y| no_overlap(x.trunc(), y.trunc()));
        let source = TwoFloat::try_from((a_fract.trunc(), b_fract.trunc())).unwrap();
        let expected = TwoFloat::from(0.0);
        let result = source.fract();
        assert_eq!(
            result, expected,
            "Non-zero fractional part of integer {:?}",
            source
        );
    });
}

// trunc() tests

#[test]
fn trunc_hi_fract_test() {
    repeated_test(|| {
        let source = get_valid_twofloat(|x, _| x.fract() != 0.0);
        let expected = TwoFloat::from(source.hi().trunc());
        let result = source.trunc();

        assert!(
            result.is_valid(),
            "trunc({:?}) produced invalid value",
            source
        );
        assert!(
            result.hi().fract() == 0.0,
            "Fractional part remains in high word after truncating {:?}",
            source
        );
        assert!(
            result.lo().fract() == 0.0,
            "Fractional part remains in low word after truncating {:?}",
            source
        );
        assert_eq!(result, expected, "Incorrect value of trunc({:?})", source);
    })
}

#[test]
fn trunc_lo_fract_test() {
    repeated_test(|| {
        let (a_fract, b) = get_valid_pair(|x, y| y.fract() != 0.0 && no_overlap(x.trunc(), y));
        let a = a_fract.trunc();
        let source = TwoFloat::try_from((a, b)).unwrap();
        let expected = if a.is_sign_positive() {
            TwoFloat::new_add(a, b.floor())
        } else {
            TwoFloat::new_add(a, b.ceil())
        };

        let result = source.trunc();
        assert!(
            result.is_valid(),
            "trunc({:?}) produced invalid value",
            source
        );
        assert!(
            result.hi().fract() == 0.0,
            "Fractional part remains in high word after truncating {:?}",
            source
        );
        assert!(
            result.lo().fract() == 0.0,
            "Fractional part remains in low word after truncating {:?}",
            source
        );
        assert_eq!(result, expected, "Incorrect value in trunc({:?})", source);
    });
}

#[test]
fn trunc_no_lo_word_test() {
    repeated_test(|| {
        let a = random_float();
        let source = TwoFloat::from(a);
        let result = source.trunc();

        assert!(
            result.is_valid(),
            "trunc({:?}) produced invalid value",
            source
        );
        assert_eq!(
            result,
            a.trunc(),
            "trunc({:?}) produced incorrect value",
            source
        );
    })
}

#[test]
fn trunc_no_lo_fract_test() {
    repeated_test(|| {
        let (a, b_fract) = get_valid_pair(|x, y| no_overlap(x, y.trunc()));
        let b = b_fract.trunc();
        let source = TwoFloat::try_from((a, b)).unwrap();
        let result = source.trunc();

        assert!(
            result.is_valid(),
            "trunc({:?} produced invalid value",
            source
        );
        assert_eq!(
            result.lo(),
            b,
            "trunc({:?}) changed integer low word",
            source
        );
        assert_eq!(
            result.hi(),
            a.trunc(),
            "trunc({:?}) returned incorrect high word",
            source
        );
    })
}

#[test]
fn trunc_no_fract_test() {
    repeated_test(|| {
        let (a_fract, b_fract) = get_valid_pair(|x, y| no_overlap(x.trunc(), y.trunc()));
        let source = TwoFloat::try_from((a_fract.trunc(), b_fract.trunc())).unwrap();
        let expected = source;
        let result = source.trunc();

        assert!(
            result.is_valid(),
            "trunc({:?}) produced invalid value",
            source
        );
        assert_eq!(
            result, expected,
            "trunc({:?}) returned different value",
            source
        );
    })
}

// ceil() tests

#[test]
fn ceil_hi_fract_test() {
    repeated_test(|| {
        let source = get_valid_twofloat(|x, _| x.fract() != 0.0);
        let expected = TwoFloat::from(source.hi().ceil());
        let result = source.ceil();

        assert!(
            result.is_valid(),
            "ceil({:?}) produced invalid value",
            source
        );
        assert_eq!(result, expected, "Incorrect value of ceil({:?})", source);
    })
}

#[test]
fn ceil_lo_fract_test() {
    repeated_test(|| {
        let (a_fract, b) = get_valid_pair(|x, y| y.fract() != 0.0 && no_overlap(x.trunc(), y));
        let a = a_fract.trunc();
        let source = TwoFloat::try_from((a, b)).unwrap();
        let expected = TwoFloat::new_add(a, b.ceil());
        let result = source.ceil();

        assert!(
            result.is_valid(),
            "ceil({:?}) produced invalid value",
            source
        );
        assert_eq!(result, expected, "Incorrect value of ceil({:?})", source);
    })
}

#[test]
fn ceil_no_lo_word_test() {
    repeated_test(|| {
        let a = random_float();
        let source = TwoFloat::from(a);
        let result = source.ceil();

        assert!(
            result.is_valid(),
            "ceil({:?}) produced invalid value",
            source
        );
        assert_eq!(
            result,
            a.ceil(),
            "ceil({:?}) produced incorrect value",
            source
        );
    })
}

#[test]
fn ceil_no_lo_fract_test() {
    repeated_test(|| {
        let (a, b_fract) = get_valid_pair(|x, y| no_overlap(x, y.trunc()));
        let b = b_fract.trunc();
        let source = TwoFloat::try_from((a, b)).unwrap();
        let result = source.ceil();

        assert!(
            result.is_valid(),
            "ceil({:?} produced invalid value",
            source
        );
        assert_eq!(
            result.lo(),
            b,
            "ceil({:?}) changed integer low word",
            source
        );
        assert_eq!(
            result.hi(),
            a.ceil(),
            "ceil({:?}) returned incorrect high word",
            source
        );
    })
}

#[test]
fn ceil_no_fract_test() {
    repeated_test(|| {
        let (a_fract, b_fract) = get_valid_pair(|x, y| no_overlap(x.trunc(), y.trunc()));
        let source = TwoFloat::try_from((a_fract.trunc(), b_fract.trunc())).unwrap();
        let expected = source;
        let result = source.ceil();

        assert!(
            result.is_valid(),
            "ceil({:?}) produced invalid value",
            source
        );
        assert_eq!(
            result, expected,
            "Ceil of integer {:?} returned different value",
            source
        );
    })
}

// floor() tests

#[test]
fn floor_hi_fract_test() {
    repeated_test(|| {
        let source = get_valid_twofloat(|x, _| x.fract() != 0.0);
        let expected = TwoFloat::from(source.hi().floor());
        let result = source.floor();

        assert!(
            result.is_valid(),
            "floor({:?}) produced invalid value",
            source
        );
        assert_eq!(result, expected, "Incorrect value of floor({:?})", source);
    })
}

#[test]
fn floor_lo_fract_test() {
    repeated_test(|| {
        let (a_fract, b) = get_valid_pair(|x, y| y.fract() != 0.0 && no_overlap(x.trunc(), y));
        let a = a_fract.trunc();
        let source = TwoFloat::try_from((a, b)).unwrap();
        let expected = TwoFloat::new_add(a, b.floor());
        let result = source.floor();
        assert!(
            result.is_valid(),
            "floor({:?}) produced invalid value",
            source
        );
        assert_eq!(result, expected, "Incorrect value of floor({:?})", source);
    });
}

#[test]
fn floor_no_lo_word_test() {
    repeated_test(|| {
        let a = random_float();
        let source = TwoFloat::from(a);
        let result = source.floor();

        assert!(
            result.is_valid(),
            "ceil({:?}) produced invalid value",
            source
        );
        assert_eq!(
            result,
            a.floor(),
            "ceil({:?}) produced incorrect value",
            source
        );
    });
}

#[test]
fn floor_no_lo_fract_test() {
    repeated_test(|| {
        let (a, b_fract) = get_valid_pair(|x, y| no_overlap(x, y.trunc()));
        let b = b_fract.trunc();
        let source = TwoFloat::try_from((a, b)).unwrap();
        let result = source.floor();

        assert!(
            result.is_valid(),
            "floor({:?} produced invalid value",
            source
        );
        assert_eq!(
            result.lo(),
            b,
            "floor({:?}) changed integer low word",
            source
        );
        assert_eq!(
            result.hi(),
            a.floor(),
            "floor({:?}) returned incorrect high word",
            source
        );
    })
}

#[test]
fn floor_no_fract_test() {
    repeated_test(|| {
        let (a_fract, b_fract) = get_valid_pair(|x, y| no_overlap(x.trunc(), y.trunc()));
        let source = TwoFloat::try_from((a_fract.trunc(), b_fract.trunc())).unwrap();
        let expected = source;
        let result = source.floor();
        assert!(
            result.is_valid(),
            "floor({:?}) produced invalid value",
            source
        );
        assert_eq!(
            result, expected,
            "floor({:?}) returned different value",
            source
        );
    });
}

// round() tests

#[test]
fn round_hi_fract_test() {
    repeated_test(|| {
        let source = get_valid_twofloat(|x, _| x.fract() != 0.0 && x.fract().abs() != 0.5);
        let expected = TwoFloat::from(source.hi().round());
        let result = source.round();

        assert!(
            result.is_valid(),
            "round({:?}) produced invalid value",
            source
        );
        assert_eq!(result, expected, "Incorrect value of round({:?})", source);
    });
}

#[test]
fn round_hi_half_test() {
    repeated_test(|| {
        let (a, b) = get_valid_pair(|x, y| {
            let x_half = x.trunc() + 0.5;
            x_half.fract().abs() == 0.5 && no_overlap(x_half, y)
        });
        let source = TwoFloat::try_from((a.trunc() + 0.5, b)).unwrap();
        let expected = if source.hi().is_sign_positive() == source.lo().is_sign_positive() {
            TwoFloat::from(source.hi().round())
        } else {
            TwoFloat::from(source.hi().trunc())
        };
        let result = source.round();

        assert!(
            result.is_valid(),
            "round({:?}) produced invalid value",
            source
        );
        assert_eq!(result, expected, "Incorrect value of round({:?})", source);
    });
}

#[test]
fn round_lo_fract_test() {
    repeated_test(|| {
        let (a_fract, b) = get_valid_pair(|x, y| {
            y.fract() != 0.0 && y.fract().abs() != 0.5 && no_overlap(x.trunc(), y)
        });
        let a = a_fract.trunc();
        let source = TwoFloat::try_from((a, b)).unwrap();
        let expected = TwoFloat::new_add(a, b.round());
        let result = source.round();
        assert!(
            result.is_valid(),
            "round({:?}) produced invalid value",
            source
        );
        assert_eq!(result, expected, "Incorrect value of round({:?})", source);
    });
}

#[test]
fn round_lo_half_test() {
    repeated_test(|| {
        let (a, b) = get_valid_pair(|x, y| {
            let y_half = y.trunc() + 0.5;
            y_half.fract().abs() == 0.5 && no_overlap(x, y_half)
        });
        let source = TwoFloat::try_from((a, b.trunc() + 0.5)).unwrap();
        let expected = if source.hi().is_sign_positive() == source.lo().is_sign_positive() {
            TwoFloat::new_add(source.hi(), source.lo().round())
        } else {
            TwoFloat::new_add(source.hi(), source.lo().trunc())
        };

        let result = source.round();

        assert!(
            result.is_valid(),
            "round({:?}) produced invalid value",
            source
        );
        assert_eq!(result, expected, "Incorrect value of round({:?})", source);
    });
}

#[test]
fn round_no_lo_word_test() {
    repeated_test(|| {
        let a = random_float();
        let source = TwoFloat::from(a);
        let result = source.round();

        assert!(
            result.is_valid(),
            "round({:?}) produced invalid value",
            source
        );
        assert_eq!(
            result,
            a.round(),
            "round({:?}) produced incorrect value",
            source
        );
    })
}

#[test]
fn round_no_lo_fract_test() {
    repeated_test(|| {
        let (a, b_fract) = get_valid_pair(|x, y| no_overlap(x, y.trunc()));
        let b = b_fract.trunc();
        let source = TwoFloat::try_from((a, b)).unwrap();
        let result = source.round();

        assert!(
            result.is_valid(),
            "round({:?} produced invalid value",
            source
        );
        assert_eq!(
            result.lo(),
            b,
            "round({:?}) changed integer low word",
            source
        );
        assert_eq!(
            result.hi(),
            a.round(),
            "round({:?}) returned incorrect high word",
            source
        );
    });
}

#[test]
fn round_no_fract_test() {
    repeated_test(|| {
        let (a_fract, b_fract) = get_valid_pair(|x, y| no_overlap(x.trunc(), y.trunc()));
        let source = TwoFloat::try_from((a_fract.trunc(), b_fract.trunc())).unwrap();
        let expected = source;
        let result = source.round();
        assert!(
            result.is_valid(),
            "round({:?}) produced invalid value",
            source
        );
        assert_eq!(
            result, expected,
            "round({:?}) returned different value",
            source
        );
    })
}
