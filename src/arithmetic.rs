use crate::twofloat::TwoFloat;

// Joldes et al. (2017) Algorithm 1
fn fast_two_sum(a: f64, b: f64) -> (f64, f64) {
    let s = a + b;
    let z = s - a;
    (s, b - z)
}

// Joldes et al. (2017) Algorithm 2
fn two_sum(a: f64, b: f64) -> (f64, f64) {
    let s = a + b;
    let aa = s - b;
    let bb = s - aa;
    let da = a - aa;
    let db = b - bb;
    (s, da + db)
}

// Joldes et al. (2017) Algorithm 2 for negative numbers
fn two_diff(a: f64, b: f64) -> (f64, f64) {
    let s = a - b;
    let aa = s + b;
    let bb = s - aa;
    let da = a - aa;
    let db = b + bb;
    (s, da - db)
}

// Joldes et al. (2017) Algorithm 3
fn two_prod(a: f64, b: f64) -> (f64, f64) {
    let p = a * b;
    (p, a.mul_add(b, -p))
}

impl TwoFloat {
    /// Creates a TwoFloat by adding two f64 values
    pub fn new_add(x: f64, y: f64) -> TwoFloat {
        let (a, b) = two_sum(x, y);
        TwoFloat { hi: a, lo: b }
    }

    /// Creates a TwoFloat by subtracting two f64 values
    pub fn new_sub(x: f64, y: f64) -> TwoFloat {
        let (a, b) = two_diff(x, y);
        TwoFloat { hi: a, lo: b }
    }

    /// Creates a TwoFloat by multiplying two f64 values
    pub fn new_mul(x: f64, y: f64) -> TwoFloat {
        let (a, b) = two_prod(x, y);
        TwoFloat { hi: a, lo: b }
    }

    /// Creates a TwoFloat by dividing two f64 values
    pub fn new_div(x: f64, y: f64) -> TwoFloat {
        let th = x / y;
        let (ph, pl) = two_prod(th, y);
        let dh = x - ph;
        let d = dh - pl;
        let tl = d / y;
        let (a, b) = fast_two_sum(th, tl);
        TwoFloat { hi: a, lo: b }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::*;

    fn get_valid_pair<F : Fn(f64, f64) -> bool>(rng: F64Rand, pred: F) -> (f64, f64) {
        loop {
            let a = rng();
            let b = rng();
            if pred(a, b) { return (a, b); };
        }
    }

    randomized_test!(fast_two_sum_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { (a + b).is_finite() });
        let (hi, lo) = if a.abs() >= b.abs() { fast_two_sum(a, b) } else { fast_two_sum(b, a) };

        assert_eq_ulp!(hi, a + b, 1, "Incorrect result of fast_two_sum({}, {})", a, b);
        assert!(no_overlap(hi, lo).unwrap_or(false), "Overlapping bits in two_sum({}, {})", a, b);
    });

    randomized_test!(two_sum_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { (a + b).is_finite() });
        let (hi, lo) = two_sum(a, b);
        
        assert_eq_ulp!(hi, a + b, 1, "Incorrect result of two_sum({}, {})", a, b);
        assert!(no_overlap(hi, lo).unwrap_or(false), "Overlapping bits in two_sum({}, {})", a, b);
    });

    randomized_test!(two_diff_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { (a - b).is_finite() });
        let (hi, lo) = two_diff(a, b);

        assert_eq_ulp!(hi, a - b, 1, "Incorrect resut of two_diff({}, {})", a, b);
        assert!(no_overlap(hi, lo).unwrap_or(false), "Overlapping bits in two_diff({}, {})", a, b);
    });

    randomized_test!(two_prod_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { (a * b).is_finite() });
        let (hi, lo) = two_prod(a, b);

        assert_eq_ulp!(hi, a * b, 1, "Incorrect result of two_prod({}, {})", a, b);
        assert!(no_overlap(hi, lo).unwrap_or(false), "Overlapping bits in two_prod({}, {})", a, b);
    });

    randomized_test!(new_add_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { (a + b).is_finite() });
        let expected = two_sum(a, b);
        let actual = TwoFloat::new_add(a, b);
        assert_eq!(actual.hi, expected.0, "Incorrect result of new_add({}, {})", a, b);
        assert_eq!(actual.lo, expected.1, "Incorrect result of new_add({}, {})", a, b);
    });

    randomized_test!(new_sub_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { (a - b).is_finite() });
        let expected = two_diff(a, b);
        let actual = TwoFloat::new_sub(a, b);
        assert_eq!(actual.hi, expected.0, "Incorrect result of new_sub({}, {})", a, b);
        assert_eq!(actual.lo, expected.1, "Incorrect result of new_sub({}, {})", a, b);
    });

    randomized_test!(new_mul_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { (a * b).is_finite() });
        let expected = two_prod(a, b);
        let actual = TwoFloat::new_mul(a, b);
        assert_eq!(actual.hi, expected.0, "Incorrect result of new_mul({}, {})", a, b);
        assert_eq!(actual.lo, expected.1, "Incorrect result of new_mul({}, {})", a, b);
    });

    randomized_test!(new_div_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { (a / b).is_finite() });
        let actual = TwoFloat::new_div(a, b);
        let ef = |a: f64, b: f64| -> u64 { let ab = a.to_bits(); let bb = b.to_bits(); if ab > bb { ab - bb } else { bb - ab }};
        assert_eq_ulp!(actual.hi, a / b, 1, "Incorrect result of new_div({}, {}) - {}", a, b, ef(actual.hi, a / b));
        assert!(no_overlap(actual.hi, actual.lo).unwrap_or(false), "Overlapping bits in new_div({}, {})", a, b);
    });
}
