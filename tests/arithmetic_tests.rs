extern crate twofloat;

use twofloat::TwoFloat;

#[test]
fn add_sub_near_one_test() {
    let a = TwoFloat::new_add(1.0, 1.0e-300);
    let b: f64 = (a - 1.0).into();
    assert_eq!(b, 1.0e-300);
}

#[test]
fn add_sub_high_test() {
    let a = TwoFloat::new_add(1.0e300, 15.125);
    let b = TwoFloat::new_add(1.0e300, -12.5);
    let c: f64 = (a - b).into();
    assert_eq!(c, 27.625);
}

#[test]
fn cube_root() {
    // solve cube root of 7 with Newton-Raphson method
    let mut x = TwoFloat::from(1.91);
    let solved = loop {
        let old_x = x;
        x = x - (x * x * x - 7.0) / (3.0 * x * x);
        if (x - old_x).abs() < 1e-50 { break x; };
    };

    assert!(solved - 7f64.cbrt() < std::f64::EPSILON);
}
