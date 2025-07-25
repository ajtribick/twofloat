use num_traits::{Float, One};
use twofloat::TwoFloat;

pub mod common;

#[test]
fn tan_test() {
    let value = TwoFloat::one();
    let expected = TwoFloat::tan(value);
    let actual = Float::tan(value);
    assert_eq!(expected, actual);
}
