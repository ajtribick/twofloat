# Changelog

## Version 0.3.0

* Add specific implementations for `exp2` and `log2` functions.
* Add optional support for serde (feature "serde_support").

## Version 0.2.2

* Removed debugging code accidentally left in no_overlap function.

## Version 0.2.1

* Added `exp_m1`, `ln_1p` functions.

## Version 0.2.0

* Breaking change: prefer value-like arguments to improve ergonomics.
* Breaking change: use `ConversionError` type to represent failure of
  `try_into`.
* Breaking change: replaced `try_new` and `data` with conversions to/from
  `(f64, f64)` and `[f64; 2]`.
* Added `NAN` constant.

## Version 0.1.4

* Added `copysign`, `hypot`, `round`, `signum` functions.
* Added trigonometric functions `cos`, `sin`, `sin_cos`, `tan` and inverse
  functions `asin`, `acos`, `atan`, `atan2`.
* Added Euclidean division and remainder functions `div_euclid`,
  `rem_euclid`.

## Version 0.1.3

* Added `hi` and `lo` functions to extract individual words.
* Added angle conversion functions: `to_degrees`, `to_radians`.
* Added hyperbolic functions `cosh`, `sinh`, `tanh` and inverse functions
  `acosh`, `asinh`, `atanh`.
* Updated `is_valid` method to check for overlapping representations.
* Bugfix in integer truncation where low word fraction was zero.

## Version 0.1.2

* Added functions: `cbrt`, `exp`, `exp2`, `ln`, `log`, `log2`, `log10`, `powf`.
* Added mathematical constants.

## Version 0.1.1

* Added functions: `recip`, `max`, `min`, `fract`, `trunc`, `ceil`, `floor`,
  `sqrt`, `powi`.
* Added `%` and `%=` operators.

## Version 0.1.0

* Initial release.
