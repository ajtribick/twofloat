#![macro_use]

macro_rules! polynomial {
    ($x:ident, $coeff:expr) => ($coeff);
    ($x:ident, $coeff:expr, $($coeffs:expr),+) => ($coeff + $x * polynomial!($x, $($coeffs),+));
}
