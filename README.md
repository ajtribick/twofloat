# twofloat
**A double-double arithmetic library for Rust.**

This library provides an implementation of double-double arithmetic for the
Rust language. Note that this is not the same as the IEEE quadruple-precision
floating-point format. Instead, higher precision is obtained by representing
the value as the sum of two non-overlapping `f64` values.

Currently the provided API is very basic, I hope to be able to provide more
mathematical functions (square root, logarithms, exponentiation and
trigonometry) in future releases.

## References

* Mioara Joldes, Jean-Michel Muller, Valentina Popescu. Tight and rigourous
  error bounds for basic building blocks of double-word arithmetic. ACM
  Transactions on Mathematical Software, Association for Computing Machinery,
  2017, 44 (2), pp.1 - 27. 10.1145/3121432. hal-01351529v3
