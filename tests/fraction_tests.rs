use twofloat::*;

pub mod common;
use common::*;

randomized_test!(fract_hi_fract_test, |rng: F64Rand| {
    let source = get_valid_twofloat(rng, |x, _| { x.fract() != 0.0 });
    let (a, b) = source.data();
    let expected = a.fract() + b.fract();
    let result = source.fract();
    let (hi, lo) = result.data();
    assert!(
        no_overlap(hi, lo),
        "Overlap in fract({:?})",
        source
    );
    assert!(
        hi.trunc() == 0.0
            || (hi.trunc().abs() == 1.0 && ((hi >= 0.0) != (lo >= 0.0))),
        "Fractional part of {:?} is greater than one",
        source
    );
    assert!(
        lo.trunc() == 0.0,
        "Non-zero integer part of low word of fract({:?}",
        source
    );
    assert_eq_ulp!(
        hi,
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
    let (hi, lo) = result.data();
    assert!(
        no_overlap(hi, lo),
        "Overlap in fract({:?})",
        source
    );
    println!("{:?}", result);
    assert!(
        hi.trunc() == 0.0
            || (hi.trunc().abs() == 1.0 && ((hi >= 0.0) != (lo >= 0.0))),
        "Fractional part of {:?} is greater than one",
        source
    );
    assert!(
        lo.trunc() == 0.0,
        "Non-zero integer part of low word of fract({:?}",
        source
    );
    assert_eq_ulp!(
        hi,
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
    let source = get_valid_twofloat(rng, |x, _| { x.fract() != 0.0 });
    let expected = TwoFloat::from(source.data().0.trunc());
    let result = source.trunc();
    let (hi, lo) = result.data();

    assert!(
        no_overlap(hi, lo),
        "Overlap in trunc({:?})",
        source
    );
    assert!(
        hi.fract() == 0.0,
        "Fractional part remains in high word after truncating {:?}",
        source
    );
    assert!(
        lo.fract() == 0.0,
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
    let (hi, lo) = result.data();
    assert!(
        no_overlap(hi, lo),
        "Overlap in trunc({:?})",
        source
    );
    assert!(
        hi.fract() == 0.0,
        "Fractional part remains in high word after truncating {:?}",
        source
    );
    assert!(
        lo.fract() == 0.0,
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
    let (hi, lo) = result.data();
    assert!(no_overlap(hi, lo), "Overlap in trunc({:?})", source);
    assert_eq!(
        result, expected,
        "Truncation of integer {:?} returned different value",
        source
    );
});

randomized_test!(ceil_hi_fract_test, |rng: F64Rand| {
    let source = get_valid_twofloat(rng, |x, _| { x.fract() != 0.0 });
    let expected = TwoFloat::from(source.data().0.ceil());
    let result = source.ceil();
    let (hi, lo) = result.data();

    assert!(
        no_overlap(hi, lo),
        "ceil({:?}) contained overlap",
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
    let (hi, lo) = result.data();

    assert!(
        no_overlap(hi, lo),
        "ceil({:?}) contained overlap",
        source
    );
    assert_eq!(result, expected, "Incorrect value of ceil({:?})", source);
});

randomized_test!(ceil_no_fract_test, |rng: F64Rand| {
    let (a_fract, b_fract) = get_valid_pair(rng, |x, y| no_overlap(x.trunc(), y.trunc()));
    let source = TwoFloat::try_new(a_fract.trunc(), b_fract.trunc()).unwrap();
    let expected = source;
    let result = source.ceil();
    let (hi, lo) = result.data();
    assert!(
        no_overlap(hi, lo),
        "ceil({:?}) contained overlap",
        source
    );
    assert_eq!(
        result, expected,
        "Ceil of integer {:?} returned different value",
        source
    );
});

randomized_test!(floor_hi_fract_test, |rng: F64Rand| {
    let source = get_valid_twofloat(rng, |x, _| { x.fract() != 0.0 });
    let expected = TwoFloat::from(source.data().0.floor());
    let result = source.floor();
    let (hi, lo) = result.data();

    assert!(
        no_overlap(hi, lo),
        "floor({:?}) contained overlap",
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
    let (hi, lo) = result.data();
    assert!(
        no_overlap(hi, lo),
        "floor({:?}) contained overlap",
        source
    );
    assert_eq!(result, expected, "Incorrect value of floor({:?})", source);
});

randomized_test!(floor_no_fract_test, |rng: F64Rand| {
    let (a_fract, b_fract) = get_valid_pair(rng, |x, y| no_overlap(x.trunc(), y.trunc()));
    let source = TwoFloat::try_new(a_fract.trunc(), b_fract.trunc()).unwrap();
    let expected = source;
    let result = source.floor();
    let (hi, lo) = result.data();
    assert!(
        no_overlap(hi, lo),
        "floor({:?}) contained overlap",
        source
    );
    assert_eq!(
        result, expected,
        "Floor of integer value {:?} returned different value",
        source
    );
});
