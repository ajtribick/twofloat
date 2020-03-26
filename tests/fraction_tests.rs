use twofloat::*;

pub mod common;
use common::*;

randomized_test!(fract_hi_fract_test, |rng: F64Rand| {
    let source = get_valid_twofloat(rng, |x, _| x.fract() != 0.0);
    let expected = source.hi().fract() + source.lo().fract();
    let result = source.fract();
    assert!(
        result.is_valid(),
        "fract({:?}) produced invalid value",
        source
    );
    assert!(
        result.hi().trunc() == 0.0
            || (result.hi().trunc().abs() == 1.0 && ((result.hi() >= 0.0) != (result.lo() >= 0.0))),
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

randomized_test!(fract_lo_fract_test, |rng: F64Rand| {
    let (a_fract, b) = get_valid_pair(rng, |x, y| y.fract() != 0.0 && no_overlap(x.trunc(), y));
    let a = a_fract.trunc();
    let source = TwoFloat::try_new(a, b).unwrap();
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
    println!("{:?}", result);
    assert!(
        result.hi().trunc() == 0.0
            || (result.hi().trunc().abs() == 1.0 && ((result.hi() >= 0.0) != (result.lo() >= 0.0))),
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

randomized_test!(fract_no_fract_test, |rng: F64Rand| {
    let (a_fract, b_fract) = get_valid_pair(rng, |x, y| no_overlap(x.trunc(), y.trunc()));
    let source = TwoFloat::try_new(a_fract.trunc(), b_fract.trunc()).unwrap();
    let expected = TwoFloat::from(0.0);
    let result = source.fract();
    assert_eq!(
        result, expected,
        "Non-zero fractional part of integer {:?}",
        source
    );
});

randomized_test!(trunc_hi_fract_test, |rng: F64Rand| {
    let source = get_valid_twofloat(rng, |x, _| x.fract() != 0.0);
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
});

randomized_test!(trunc_lo_fract_test, |rng: F64Rand| {
    let (a_fract, b) = get_valid_pair(rng, |x, y| y.fract() != 0.0 && no_overlap(x.trunc(), y));
    let a = a_fract.trunc();
    let source = TwoFloat::try_new(a, b).unwrap();
    let expected = match (a >= 0.0, b >= 0.0) {
        (true, false) => TwoFloat::new_add(a, b.trunc() - 1.0),
        (false, true) => TwoFloat::new_add(a, b.trunc() + 1.0),
        _ => TwoFloat::try_new(a, b.trunc()).unwrap(),
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

randomized_test!(trunc_no_fract_test, |rng: F64Rand| {
    let (a_fract, b_fract) = get_valid_pair(rng, |x, y| no_overlap(x.trunc(), y.trunc()));
    let source = TwoFloat::try_new(a_fract.trunc(), b_fract.trunc()).unwrap();
    let expected = source;
    let result = source.trunc();

    assert!(
        result.is_valid(),
        "trunc({:?}) produced invalid value",
        source
    );
    assert_eq!(
        result, expected,
        "Truncation of integer {:?} returned different value",
        source
    );
});

randomized_test!(ceil_hi_fract_test, |rng: F64Rand| {
    let source = get_valid_twofloat(rng, |x, _| x.fract() != 0.0);
    let expected = TwoFloat::from(source.hi().ceil());
    let result = source.ceil();

    assert!(
        result.is_valid(),
        "ceil({:?}) produced invalid value",
        source
    );
    assert_eq!(result, expected, "Incorrect value of ceil({:?})", source);
});

randomized_test!(ceil_lo_fract_test, |rng: F64Rand| {
    let (a_fract, b) = get_valid_pair(rng, |x, y| y.fract() != 0.0 && no_overlap(x.trunc(), y));
    let a = a_fract.trunc();
    let source = TwoFloat::try_new(a, b).unwrap();
    let expected = TwoFloat::new_add(a, b.ceil());
    let result = source.ceil();

    assert!(
        result.is_valid(),
        "ceil({:?}) produced invalid value",
        source
    );
    assert_eq!(result, expected, "Incorrect value of ceil({:?})", source);
});

randomized_test!(ceil_no_fract_test, |rng: F64Rand| {
    let (a_fract, b_fract) = get_valid_pair(rng, |x, y| no_overlap(x.trunc(), y.trunc()));
    let source = TwoFloat::try_new(a_fract.trunc(), b_fract.trunc()).unwrap();
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
});

randomized_test!(floor_hi_fract_test, |rng: F64Rand| {
    let source = get_valid_twofloat(rng, |x, _| x.fract() != 0.0);
    let expected = TwoFloat::from(source.hi().floor());
    let result = source.floor();

    assert!(
        result.is_valid(),
        "floor({:?}) produced invalid value",
        source
    );
    assert_eq!(result, expected, "Incorrect value of floor({:?})", source);
});

randomized_test!(floor_lo_fract_test, |rng: F64Rand| {
    let (a_fract, b) = get_valid_pair(rng, |x, y| y.fract() != 0.0 && no_overlap(x.trunc(), y));
    let a = a_fract.trunc();
    let source = TwoFloat::try_new(a, b).unwrap();
    let expected = TwoFloat::new_add(a, b.floor());
    let result = source.floor();
    assert!(
        result.is_valid(),
        "floor({:?}) produced invalid value",
        source
    );
    assert_eq!(result, expected, "Incorrect value of floor({:?})", source);
});

randomized_test!(floor_no_fract_test, |rng: F64Rand| {
    let (a_fract, b_fract) = get_valid_pair(rng, |x, y| no_overlap(x.trunc(), y.trunc()));
    let source = TwoFloat::try_new(a_fract.trunc(), b_fract.trunc()).unwrap();
    let expected = source;
    let result = source.floor();
    assert!(
        result.is_valid(),
        "floor({:?}) produced invalid value",
        source
    );
    assert_eq!(
        result, expected,
        "Floor of integer value {:?} returned different value",
        source
    );
});
