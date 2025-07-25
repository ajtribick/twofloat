# twofloat

[![Crate](https://img.shields.io/crates/v/twofloat)](https://crates.io/crates/twofloat)

**A double-double arithmetic library for Rust.**

This library provides an implementation of double-double arithmetic for the
Rust language. Note that this is not the same as the IEEE quadruple-precision
floating-point format. Instead, higher precision is obtained by representing
the value as the sum of two non-overlapping `f64` values.

Operator traits are implemented both for reference and value types where
appropriate. The code indicates the source of the algorithms used.

Mathematical constants are available in the `twofloat::consts` module, which
provides the same set of constants as `std::f64::consts`.

Please note that the implementation of the mathematical functions (`exp`,
`powf`, etc.) is very preliminary. In particular, they are calculated using
operations at the same precision as the result, so they will not return values
which are correct to the full precision of the `TwoFloat` type. This may be
addressed in future releases.

## Optional features

* `math_funcs` - does nothing, left for compatibility. This will be removed in
  v0.9.
* `serde` - enable serialization/deserialization with Serde.
* `std` - use std mathematical functions instead of libm.

## Known issues

* The MinGW `fma` implementation appears to give incorrect results in some
  cases, so the libm implementation is always used on this platform.

## References

* Mioara Joldeș, Jean-Michel Muller, Valentina Popescu. Tight and rigourous
  error bounds for basic building blocks of double-word arithmetic. ACM
  Transactions on Mathematical Software, Association for Computing Machinery,
  2017, 44 (2), pp. 1-27. 10.1145/3121432. hal-01351529v3

* Alan H. Karp, Peter Markstein. High Precision Division and Square Root. ACM
  Transactions on Mathematical Software, Association for Computing Machinery,
  1997, 23 (4), pp. 561-589. 10.1145/279232.279237.

* S. Chevillard, M. Joldeș and C. Lauter. Sollya: an environment for the
  development of numerical codes. Mathematical Software - ICMS 2010, pp.
  28–31.
