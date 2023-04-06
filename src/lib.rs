/*!
# twofloat

This library provides an implementation of double-double arithmetic for the
Rust language. Note that this is not the same as the IEEE quadruple-precision
floating-point format. Instead, higher precision is obtained by representing
the value as the sum of two non-overlapping `f64` values.

## Usage

The basic type is `TwoFloat` which represents the sum of two non-overlapping
`f64` values, which may be initialized from a single `f64` or by calling a
constructor that performs an arithmetic operation on a pair of `f64` values.

```
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
Mathematical functions are provided if the `math_funcs` feature is enabled
(this is enabled by default), though the implementations should be regarded
as preliminary.

Operations on non-finite values are not supported. At the moment this is not
automatically checked. The `is_valid()` method is provided for this purpose.

If the `serde` feature is enabled, serialization and deserialization is
possible through the Serde library.

## Known issues

This library depends on the `std` implementations of the floating point
mathematical functions, which in turn depend on the C standard library
implementation, which may have variations between platforms.

## References

* Mioara Joldes, Jean-Michel Muller, Valentina Popescu. Tight and rigourous
  error bounds for basic building blocks of double-word arithmetic. ACM
  Transactions on Mathematical Software, Association for Computing Machinery,
  2017, 44 (2), pp.1 - 27. 10.1145/3121432. hal-01351529v3

* Alan H. Karp, Peter Markstein. High Precision Division and Square Root. ACM
  Transactions on Mathematical Software, Association for Computing Machinery,
  1997, 23 (4), pp. 561-589. 10.1145/279232.279237.

* S. Chevillard, M. Joldeș and C. Lauter. Sollya: an environment for the
  development of numerical codes. Mathematical Software - ICMS 2010, pp.
  28–31.
*/

#![forbid(unsafe_code)]
// Disable irrelevant lints
#![allow(clippy::approx_constant)]
#![allow(clippy::excessive_precision)]
#![allow(clippy::float_cmp)]
#![allow(clippy::suspicious_arithmetic_impl)]
#![allow(clippy::suspicious_op_assign_impl)]
#![cfg_attr(not(feature = "std"), no_std)]

use core::fmt;

mod math_util;

#[macro_use]
mod ops_util;

#[cfg(test)]
#[macro_use]
mod test_util;

mod arithmetic;
mod base;

/// Basic mathematical constants.
///
/// Values determined using Sollya.
pub mod consts;

mod convert;
mod format;
mod functions;
mod num_integration;

pub use base::no_overlap;

#[cfg(feature = "serde")]
mod serde_helper {
    use super::{TwoFloat, TwoFloatError};

    #[derive(serde::Deserialize)]
    #[serde(rename = "TwoFloat")]
    pub(super) struct TwoFloatDeserializeHelper {
        hi: f64,
        lo: f64,
    }

    impl core::convert::TryFrom<TwoFloatDeserializeHelper> for TwoFloat {
        type Error = TwoFloatError;

        fn try_from(value: TwoFloatDeserializeHelper) -> Result<Self, Self::Error> {
            TwoFloat::try_from((value.hi, value.lo))
        }
    }
}

/// Represents a two-word floating point type, represented as the sum of two
/// non-overlapping f64 values.
#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(try_from = "serde_helper::TwoFloatDeserializeHelper")
)]
pub struct TwoFloat {
    pub(crate) hi: f64,
    pub(crate) lo: f64,
}

/// The error type for `TwoFloat` operations.
#[non_exhaustive]
#[derive(Debug)]
pub enum TwoFloatError {
    /// Indicates invalid conversion to/from `TwoFloat`
    ConversionError,
    ParseError,
}

impl fmt::Display for TwoFloatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ConversionError => f.pad("invalid TwoFloat conversion"),
            Self::ParseError => f.pad("parsing not supported"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TwoFloatError {}
