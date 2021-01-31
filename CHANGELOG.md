# Changelog

## Version 0.5

* Add operator overloads for `&f64`.
* Breaking change: use `serde` as the feature flag name.
* Breaking change: use helper struct for Serde serialization.
* Integrate with `num_traits` crate.
* Add `FromStr` implementation.

## Version 0.4.1

* Internals now checked with clippy.

## Version 0.4.0

* Breaking change: update error handling to make it more future-proof.

## Version 0.3.1

* Support Default trait.

## Version 0.3

* Breaking change: add specific implementations for `exp2` and `log2`
  functions.
* Breaking change: update formatting, support exponential formats and
  precision specifiers.
* Add optional support for serde (feature `serde_support`).
* Add conversions to/from `i128` and `u128`.
* Mathematical functions are optional but enabled by default (feature
  "math_funcs").

## Version 0.2.2

* Remove debugging code accidentally left in no_overlap function.

## Version 0.2.1

* Add `exp_m1`, `ln_1p` functions.

## Version 0.2

* Breaking change: prefer value-like arguments to improve ergonomics.
* Breaking change: use `ConversionError` type to represent failure of
  `try_into`.
* Breaking change: replace `try_new` and `data` with conversions to/from
  `(f64, f64)` and `[f64; 2]`.
* Add `NAN` constant.

## Version 0.1.4

* Add `copysign`, `hypot`, `round`, `signum` functions.
* Add trigonometric functions `cos`, `sin`, `sin_cos`, `tan` and inverse
  functions `asin`, `acos`, `atan`, `atan2`.
* Add Euclidean division and remainder functions `div_euclid`, `rem_euclid`.

## Version 0.1.3

* Add `hi` and `lo` functions to extract individual words.
* Add angle conversion functions: `to_degrees`, `to_radians`.
* Add hyperbolic functions `cosh`, `sinh`, `tanh` and inverse functions
  `acosh`, `asinh`, `atanh`.
* Update `is_valid` method to check for overlapping representations.
* Bugfix in integer truncation where low word fraction was zero.

## Version 0.1.2

* Add functions: `cbrt`, `exp`, `exp2`, `ln`, `log`, `log2`, `log10`, `powf`.
* Add mathematical constants.

## Version 0.1.1

* Add functions: `recip`, `max`, `min`, `fract`, `trunc`, `ceil`, `floor`,
  `sqrt`, `powi`.
* Add `%` and `%=` operators.

## Version 0.1

* Initial release.
