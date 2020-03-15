/*!
# twofloat

This library provides an implementation of double-double arithmetic for the
Rust language. Please be aware that this is not the same as the IEEE
quadruple-precision floating-point format.

## Usage

The basic type is `TwoFloat` which represents the sum of two non-overlapping
`f64` values, which may be initialized from a single `f64` or by calling a
constructor that performs an arithmetic operation on a pair of `f64` values.

```.rust
extern crate twofloat;
use twofloat::TwoFloat;

let a = TwoFloat::from(3.4);
let b = TwoFloat::new_add(1.0, 1.0e-300);
let c = TwoFloat::new_sub(1.0, 1.0e-300);
let d = TwoFloat::new_mul(5.0, 0.7);
let e = TwoFloat::new_div(1.0, 7.0);
```

Basic arithmetic operators and comparisons are available, together with the
utility functions `abs()`, `is_positive_sign()` and `is_negative_sign()`.

Operations on non-finite values are not supported but at the moment this is
not automatically checked. The `is_valid()` method is provided for this
purpose.

## References

* Mioara Joldes, Jean-Michel Muller, Valentina Popescu. Tight and rigourous
  error bounds for basic building blocks of double-word arithmetic. ACM
  Transactions on Mathematical Software, Association for Computing Machinery,
  2017, 44 (2), pp.1 - 27. 10.1145/3121432. hal-01351529v3
*/

#[cfg(test)]
mod test_util;

mod arithmetic;
mod base;

/// Basic mathematical constants.
///
/// Values determined using Sollya.
pub mod consts;

mod convert;
mod functions;

pub use base::{no_overlap, TwoFloat};
