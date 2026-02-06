use hexf::hexf64;

use crate::TwoFloat;

/// A constant 0.
pub const ZERO: TwoFloat = TwoFloat { hi: 0.0, lo: 0.0 };

/// A constant 1.
pub const ONE: TwoFloat = TwoFloat { hi: 1.0, lo: 0.0 };

/// Euler's number (e)
pub const E: TwoFloat = TwoFloat {
    hi: hexf64!("0x1.5bf0a8b145769p1"),
    lo: hexf64!("0x1.4d57ee2b1013ap-53"),
};

/// 1/π
pub const FRAC_1_PI: TwoFloat = TwoFloat {
    hi: hexf64!("0x1.45f306dc9c883p-2"),
    lo: hexf64!("-0x1.6b01ec5417056p-56"),
};

/// 2/π
pub const FRAC_2_PI: TwoFloat = TwoFloat {
    hi: hexf64!("0x1.45f306dc9c883p-1"),
    lo: hexf64!("-0x1.6b01ec5417056p-55"),
};

/// 2/sqrt(π)
pub const FRAC_2_SQRT_PI: TwoFloat = TwoFloat {
    hi: hexf64!("0x1.20dd750429b6dp0"),
    lo: hexf64!("0x1.1ae3a914fed8p-56"),
};

/// 1/sqrt(2)
pub const FRAC_1_SQRT_2: TwoFloat = TwoFloat {
    hi: hexf64!("0x1.6a09e667f3bcdp-1"),
    lo: hexf64!("-0x1.bdd3413b26456p-55"),
};

/// π/2
pub const FRAC_PI_2: TwoFloat = TwoFloat {
    hi: hexf64!("0x1.921fb54442d18p0"),
    lo: hexf64!("0x1.1a62633145c07p-54"),
};

/// π/3
pub const FRAC_PI_3: TwoFloat = TwoFloat {
    hi: hexf64!("0x1.0c152382d7366p0"),
    lo: hexf64!("-0x1.ee6913347c2a6p-54"),
};

/// π/4
pub const FRAC_PI_4: TwoFloat = TwoFloat {
    hi: hexf64!("0x1.921fb54442d18p-1"),
    lo: hexf64!("0x1.1a62633145c07p-55"),
};

/// π/6
pub const FRAC_PI_6: TwoFloat = TwoFloat {
    hi: hexf64!("0x1.0c152382d7366p-1"),
    lo: hexf64!("-0x1.ee6913347c2a6p-55"),
};

/// π/8
pub const FRAC_PI_8: TwoFloat = TwoFloat {
    hi: hexf64!("0x1.921fb54442d18p-2"),
    lo: hexf64!("0x1.1a62633145c07p-56"),
};

/// ln(2)
pub const LN_2: TwoFloat = TwoFloat {
    hi: hexf64!("0x1.62e42fefa39efp-1"),
    lo: hexf64!("0x1.abc9e3b39803fp-56"),
};

/// ln(10)
pub const LN_10: TwoFloat = TwoFloat {
    hi: hexf64!("0x1.26bb1bbb55516p1"),
    lo: hexf64!("-0x1.f48ad494ea3e9p-53"),
};

/// log<sub>2</sub>(e)
pub const LOG2_E: TwoFloat = TwoFloat {
    hi: hexf64!("0x1.71547652b82fep0"),
    lo: hexf64!("0x1.777d0ffda0d24p-56"),
};

/// log<sub>10</sub>(e)
pub const LOG10_E: TwoFloat = TwoFloat {
    hi: hexf64!("0x1.bcb7b1526e50ep-2"),
    lo: hexf64!("0x1.95355baaafad3p-57"),
};

/// log<sub>10</sub>(2)
pub const LOG10_2: TwoFloat = TwoFloat {
    hi: hexf64!("0x1.34413509f79ffp-2"),
    lo: hexf64!("-0x1.9dc1da994fd21p-59"),
};

/// log<sub>2</sub>(10)
pub const LOG2_10: TwoFloat = TwoFloat {
    hi: hexf64!("0x1.a934f0979a371p1"),
    lo: hexf64!("0x1.7f2495fb7fa6dp-53"),
};

/// Archimedes' constant (π)
pub const PI: TwoFloat = TwoFloat {
    hi: hexf64!("0x1.921fb54442d18p1"),
    lo: hexf64!("0x1.1a62633145c07p-53"),
};

/// sqrt(2)
pub const SQRT_2: TwoFloat = TwoFloat {
    hi: hexf64!("0x1.6a09e667f3bcdp0"),
    lo: hexf64!("-0x1.bdd3413b26456p-54"),
};

/// The full circle constant (τ)
pub const TAU: TwoFloat = TwoFloat {
    hi: hexf64!("0x1.921fb54442d18p2"),
    lo: hexf64!("0x1.1a62633145c07p-52"),
};

#[cfg(test)]
mod tests {
    use super::{
        E, FRAC_1_PI, FRAC_1_SQRT_2, FRAC_2_PI, FRAC_2_SQRT_PI, FRAC_PI_2, FRAC_PI_3, FRAC_PI_4,
        FRAC_PI_6, FRAC_PI_8, LN_10, LN_2, LOG10_2, LOG10_E, LOG2_10, LOG2_E, PI, SQRT_2, TAU,
    };

    macro_rules! const_check {
        ($name:ident) => {
            #[cfg(test)]
            #[allow(non_snake_case)]
            mod $name {
                use super::$name;

                #[test]
                fn valid_test() {
                    assert!($name.is_valid());
                }

                #[test]
                fn value_test() {
                    assert_eq!($name.hi, core::f64::consts::$name);
                }
            }
        };
        ($name:ident, $($names:ident),+) => {
            const_check! { $name }
            const_check! { $($names),+ }
        };
        ($($names:ident,)+) => {
            const_check! { $($names),+ }
        };
        (#[cfg($feature:tt)] $name:ident) => {
            #[cfg(test)]
            #[allow(non_snake_case)]
            mod $name {
                use super::$name;

                #[test]
                fn valid_test() {
                    assert!($name.is_valid());
                }

                #[cfg($feature)]
                #[test]
                fn value_test() {
                    assert_eq!($name.hi, core::f64::consts::$name);
                }
            }
        };
        (#[cfg($feature:tt)] $name:ident, $($names:ident),+) => {
            const_check! { #[cfg($feature)] $name }
            const_check! { #[cfg($feature)] $($names),+ }
        };
        (#[cfg($feature:tt)] $($names:ident,)+) => {
            const_check! { #[cfg($feature)] $($names),+ }
        }
    }

    const_check! {
        E, FRAC_1_PI, FRAC_2_PI, FRAC_2_SQRT_PI, FRAC_1_SQRT_2, FRAC_PI_2,
        FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8, LN_2, LN_10, LOG2_10, LOG2_E,
        LOG10_2, LOG10_E, PI, SQRT_2, TAU
    }
}
