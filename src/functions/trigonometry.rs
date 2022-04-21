use core::convert::TryFrom;

use hexf::hexf64;

use crate::{
    consts::{FRAC_PI_2, FRAC_PI_4, PI},
    TwoFloat,
};

// Polynomial coefficients of sin(x)-x on [0,pi/4]
const SIN_COEFFS: [TwoFloat; 7] = [
    TwoFloat {
        hi: hexf64!("-0x1.5555555555555p-3"),
        lo: hexf64!("-0x1.3a26e9901c14ap-57"),
    },
    TwoFloat {
        hi: hexf64!("0x1.1111111111105p-7"),
        lo: hexf64!("-0x1.487cfb2f402fap-63"),
    },
    TwoFloat {
        hi: hexf64!("-0x1.a01a01a017e07p-13"),
        lo: hexf64!("-0x1.22340ff667d3fp-67"),
    },
    TwoFloat {
        hi: hexf64!("0x1.71de3a526314fp-19"),
        lo: hexf64!("0x1.1ddd0a161cfa7p-75"),
    },
    TwoFloat {
        hi: hexf64!("-0x1.ae6451ad6a8ebp-26"),
        lo: hexf64!("0x1.cb014c3ddfd85p-84"),
    },
    TwoFloat {
        hi: hexf64!("0x1.612010f363e7dp-33"),
        lo: hexf64!("0x1.0dba7b1b83a01p-88"),
    },
    TwoFloat {
        hi: hexf64!("-0x1.aa6c431516f76p-41"),
        lo: hexf64!("0x1.df71e9b9b179bp-95"),
    },
];

// Polynomial coefficients of cos(x)-1+x^2/2 on [0,pi/4]
const COS_COEFFS: [TwoFloat; 7] = [
    TwoFloat {
        hi: hexf64!("0x1.5555555555555p-5"),
        lo: hexf64!("0x1.4b27f9ddea57ap-59"),
    },
    TwoFloat {
        hi: hexf64!("-0x1.6c16c16c16c0fp-10"),
        lo: hexf64!("0x1.1e7208a68629bp-64"),
    },
    TwoFloat {
        hi: hexf64!("0x1.a01a01a018bcdp-16"),
        lo: hexf64!("0x1.adea7f883a49cp-71"),
    },
    TwoFloat {
        hi: hexf64!("-0x1.27e4fb75e002ap-22"),
        lo: hexf64!("-0x1.a26582a390382p-76"),
    },
    TwoFloat {
        hi: hexf64!("0x1.1eed8c87a5a51p-29"),
        lo: hexf64!("-0x1.551d13b8d9c61p-85"),
    },
    TwoFloat {
        hi: hexf64!("-0x1.93931ca96bc22p-37"),
        lo: hexf64!("0x1.25124fcc17b3fp-91"),
    },
    TwoFloat {
        hi: hexf64!("0x1.aabaa8059719cp-45"),
        lo: hexf64!("0x1.4cf2f15ef56d1p-99"),
    },
];

// Polynomial coefficients of tan(x)-x on [0,pi/4]
const TAN_COEFFS: [TwoFloat; 14] = [
    TwoFloat {
        hi: hexf64!("0x1.555555555530fp-2"),
        lo: hexf64!("-0x1.38ef22c4b8238p-56"),
    },
    TwoFloat {
        hi: hexf64!("0x1.111111112c40ap-3"),
        lo: hexf64!("0x1.db464d0cd9cb4p-57"),
    },
    TwoFloat {
        hi: hexf64!("0x1.ba1ba1a984e9fp-5"),
        lo: hexf64!("0x1.b2454b6b23d17p-61"),
    },
    TwoFloat {
        hi: hexf64!("0x1.664f4b43a4fefp-6"),
        lo: hexf64!("0x1.bb1ac07d3ba2fp-61"),
    },
    TwoFloat {
        hi: hexf64!("0x1.226ded039b30dp-7"),
        lo: hexf64!("-0x1.9110570c2853ap-63"),
    },
    TwoFloat {
        hi: hexf64!("0x1.d6ddaf4100a51p-9"),
        lo: hexf64!("-0x1.654e37a706894p-65"),
    },
    TwoFloat {
        hi: hexf64!("0x1.7d2be8d9e1761p-10"),
        lo: hexf64!("-0x1.d33168adc5b21p-64"),
    },
    TwoFloat {
        hi: hexf64!("0x1.395c8b79b1e68p-11"),
        lo: hexf64!("-0x1.5c77d3711fefdp-66"),
    },
    TwoFloat {
        hi: hexf64!("0x1.c3c79cdabdf3ep-13"),
        lo: hexf64!("0x1.7af98e21b704bp-69"),
    },
    TwoFloat {
        hi: hexf64!("0x1.399dadec87c3ap-13"),
        lo: hexf64!("-0x1.793a97fd365d5p-68"),
    },
    TwoFloat {
        hi: hexf64!("-0x1.6a82ab57290c9p-15"),
        lo: hexf64!("0x1.48e1069bffaafp-73"),
    },
    TwoFloat {
        hi: hexf64!("0x1.b3221d6d8c4b6p-14"),
        lo: hexf64!("-0x1.049e004213205p-69"),
    },
    TwoFloat {
        hi: hexf64!("-0x1.b4a2a3d0229eap-15"),
        lo: hexf64!("0x1.cb2115b70d6e3p-69"),
    },
    TwoFloat {
        hi: hexf64!("0x1.7917a05c89f91p-16"),
        lo: hexf64!("-0x1.9ff84f5cc7024p-70"),
    },
];

// Polynomial coefficients of asin(x)-x on [0,0.5]
const ASIN_COEFFS: [TwoFloat; 10] = [
    TwoFloat {
        hi: hexf64!("0x1.5555555505a93p-3"),
        lo: hexf64!("0x1.d240d1c705854p-58"),
    },
    TwoFloat {
        hi: hexf64!("0x1.333333830962bp-4"),
        lo: hexf64!("-0x1.af55ce0405fecp-62"),
    },
    TwoFloat {
        hi: hexf64!("0x1.6db6bb3abd092p-5"),
        lo: hexf64!("-0x1.cfdfea864322ap-61"),
    },
    TwoFloat {
        hi: hexf64!("0x1.f1ce012f15aafp-6"),
        lo: hexf64!("0x1.5fe81afd0c561p-72"),
    },
    TwoFloat {
        hi: hexf64!("0x1.6e1af8b2e827ep-6"),
        lo: hexf64!("-0x1.b3283f59c2f09p-60"),
    },
    TwoFloat {
        hi: hexf64!("0x1.20d826a6a4d9fp-6"),
        lo: hexf64!("-0x1.2408819e30e3ep-61"),
    },
    TwoFloat {
        hi: hexf64!("0x1.8d6db633c567p-7"),
        lo: hexf64!("0x1.58838200dd463p-61"),
    },
    TwoFloat {
        hi: hexf64!("0x1.3c047f5666c57p-6"),
        lo: hexf64!("-0x1.c0881063edf9dp-62"),
    },
    TwoFloat {
        hi: hexf64!("-0x1.401c69a113918p-7"),
        lo: hexf64!("-0x1.838c090a26969p-64"),
    },
    TwoFloat {
        hi: hexf64!("0x1.119827a2d86aap-5"),
        lo: hexf64!("-0x1.fa4bf3377ba39p-59"),
    },
];

// Polynomial coefficients of atan(x) - x on [0, 7/16]
const ATAN_COEFFS: [TwoFloat; 15] = [
    TwoFloat {
        hi: hexf64!("-0x1.5555555555555p-2"),
        lo: hexf64!("-0x1.5381cace077adp-56"),
    },
    TwoFloat {
        hi: hexf64!("0x1.9999999999998p-3"),
        lo: hexf64!("0x1.4577ef010e069p-57"),
    },
    TwoFloat {
        hi: hexf64!("-0x1.24924924923d5p-3"),
        lo: hexf64!("-0x1.431a104911639p-57"),
    },
    TwoFloat {
        hi: hexf64!("0x1.c71c71c71501dp-4"),
        lo: hexf64!("-0x1.5849ad667389fp-61"),
    },
    TwoFloat {
        hi: hexf64!("-0x1.745d17445bf06p-4"),
        lo: hexf64!("0x1.bb4a56bf72341p-58"),
    },
    TwoFloat {
        hi: hexf64!("0x1.3b13b109e4298p-4"),
        lo: hexf64!("-0x1.e54aeaea9366cp-59"),
    },
    TwoFloat {
        hi: hexf64!("-0x1.11110c8317554p-4"),
        lo: hexf64!("0x1.fe1cb3fb72cafp-62"),
    },
    TwoFloat {
        hi: hexf64!("0x1.e1e14573d9e46p-5"),
        lo: hexf64!("-0x1.9e0fa521514a9p-59"),
    },
    TwoFloat {
        hi: hexf64!("-0x1.af20ae13002ecp-5"),
        lo: hexf64!("0x1.ac847d2d89e0cp-59"),
    },
    TwoFloat {
        hi: hexf64!("0x1.85cf5eca1206ap-5"),
        lo: hexf64!("0x1.1e116d4ec0f01p-62"),
    },
    TwoFloat {
        hi: hexf64!("-0x1.622b3e8cca965p-5"),
        lo: hexf64!("-0x1.facb65280deecp-60"),
    },
    TwoFloat {
        hi: hexf64!("0x1.3d3ea913f5499p-5"),
        lo: hexf64!("-0x1.5fa025bf396bbp-59"),
    },
    TwoFloat {
        hi: hexf64!("-0x1.07d293dcdabe9p-5"),
        lo: hexf64!("-0x1.7b32fa28e715p-59"),
    },
    TwoFloat {
        hi: hexf64!("0x1.5f9188357ee62p-6"),
        lo: hexf64!("-0x1.a21e25eaaf1d8p-66"),
    },
    TwoFloat {
        hi: hexf64!("-0x1.09daee4762a73p-7"),
        lo: hexf64!("0x1.fbd4cc667e59dp-61"),
    },
];

const ATAN_FRAC_1_2: TwoFloat = TwoFloat {
    hi: hexf64!("0x1.dac670561bb4fp-2"),
    lo: hexf64!("0x1.a2b7f222f65e2p-56"),
};

const ATAN_FRAC_3_2: TwoFloat = TwoFloat {
    hi: hexf64!("0x1.f730bd281f69bp-1"),
    lo: hexf64!("0x1.007887af0cbbdp-56"),
};

fn quadrant(value: TwoFloat) -> (TwoFloat, i8) {
    if value.abs() < FRAC_PI_4 {
        (value, 0)
    } else {
        let quotient = (value / FRAC_PI_2).round();
        let remainder = value - quotient * FRAC_PI_2;
        match i8::try_from(quotient % 4.0) {
            Ok(quadrant) if quadrant >= 0 => (remainder, quadrant),
            Ok(quadrant) if quadrant >= -4 => (remainder, 4 + quadrant),
            _ => (TwoFloat::NAN, 0),
        }
    }
}

fn restricted_sin(x: TwoFloat) -> TwoFloat {
    let x2 = x * x;
    x * polynomial!(x2, 1.0, SIN_COEFFS)
}

fn restricted_cos(x: TwoFloat) -> TwoFloat {
    let x2 = x * x;
    polynomial!(x2, 1.0, -0.5, COS_COEFFS)
}

fn restricted_tan(x: TwoFloat) -> TwoFloat {
    let x2 = x * x;
    x * polynomial!(x2, 1.0, TAN_COEFFS)
}

fn restricted_asin(x: TwoFloat) -> TwoFloat {
    let x2 = x * x;
    x * polynomial!(x2, 1.0, ASIN_COEFFS)
}

fn restricted_atan(x: TwoFloat) -> TwoFloat {
    let x2 = x * x;
    x * polynomial!(x2, 1.0, ATAN_COEFFS)
}

impl TwoFloat {
    /// Computes the sine of the value (in radians).
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(2.5);
    /// let b = a.sin();
    /// let c = 2.5f64.sin();
    ///
    /// assert!((b - c).abs() < 1e-10);
    pub fn sin(self) -> Self {
        if !self.is_valid() {
            return Self::NAN;
        }
        let (x, quadrant) = quadrant(self);
        match quadrant {
            0 => restricted_sin(x),
            1 => restricted_cos(x),
            2 => -restricted_sin(x),
            _ => -restricted_cos(x),
        }
    }

    /// Computes the cosine of the value (in radians)
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(2.5);
    /// let b = a.cos();
    /// let c = 2.5f64.cos();
    ///
    /// assert!((b - c).abs() < 1e-10);
    pub fn cos(self) -> Self {
        if !self.is_valid() {
            return Self::NAN;
        }
        let (x, quadrant) = quadrant(self);
        match quadrant {
            0 => restricted_cos(x),
            1 => -restricted_sin(x),
            2 => -restricted_cos(x),
            _ => restricted_sin(x),
        }
    }

    /// Simultaneously computes the sine and cosine of the value. Returns a
    /// tuple with the sine as the first element and the cosine as the second
    /// element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(2.5);
    /// let (s, c) = a.sin_cos();
    ///
    /// assert!((s - 2.5f64.sin()).abs() < 1e-10);
    /// assert!((c - 2.5f64.cos()).abs() < 1e-10);
    pub fn sin_cos(self) -> (Self, Self) {
        if !self.is_valid() {
            return (Self::NAN, Self::NAN);
        }
        let (x, quadrant) = quadrant(self);
        let s = restricted_sin(x);
        let c = restricted_cos(x);
        match quadrant {
            0 => (s, c),
            1 => (c, -s),
            2 => (-s, -c),
            _ => (-c, s),
        }
    }

    /// Computes the tangent of the value (in radians).
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(2.5);
    /// let b = a.tan();
    /// let c = 2.5f64.tan();
    ///
    /// assert!((b - c).abs() < 1e-10);
    pub fn tan(self) -> Self {
        if !self.is_valid() {
            return self;
        }
        let (x, quadrant) = quadrant(self);
        match quadrant {
            0 | 2 => restricted_tan(x),
            _ => -1.0 / restricted_tan(x),
        }
    }

    /// Computes the arcsine of the value. Return value is in radians in the
    /// range [-π/2, π/2] or an invalid value if the input value is outside
    /// the range [-1, 1].
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(0.7);
    /// let b = a.asin();
    /// let c = 0.7f64.asin();
    ///
    /// assert!((b - c).abs() < 1e-10);
    pub fn asin(self) -> Self {
        let abs_val = self.abs();
        if !self.is_valid() || abs_val > 1.0 {
            Self::NAN
        } else if abs_val <= 0.5 {
            restricted_asin(self)
        } else {
            let result = FRAC_PI_2 - 2.0 * restricted_asin(((1.0 - self.abs()) / 2.0).sqrt());
            if self.is_sign_positive() {
                result
            } else {
                -result
            }
        }
    }

    /// Computes the arccosine of the value. Return value is in radians in
    /// the range [0, π] or an invalid value if the input value is outside
    /// the range [-1, 1].
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(-0.8);
    /// let b = a.acos();
    /// let c = (-0.8f64).acos();
    ///
    /// assert!((b - c).abs() < 1e-10);
    pub fn acos(self) -> Self {
        let x = self.asin();
        if x.is_valid() {
            FRAC_PI_2 - x
        } else {
            x
        }
    }

    /// Computes the arctangent of the value. Return value is in radians in
    /// the range [-π/2, π/2].
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(3.5);
    /// let b = a.atan();
    /// let c = 3.5f64.atan();
    ///
    /// assert!((b - c).abs() < 1e-10);
    pub fn atan(self) -> Self {
        if !self.is_valid() {
            Self::NAN
        } else if self.hi.is_infinite() {
            if self.hi.is_sign_positive() {
                FRAC_PI_2
            } else {
                -FRAC_PI_2
            }
        } else {
            let x = self.abs();
            let k = 4.0 * x + 0.25;
            if k <= 2.0 {
                return restricted_atan(self);
            }

            let result = if k < 3.0 {
                ATAN_FRAC_1_2 + restricted_atan((x - 0.5) / (1.0 + 0.5 * x))
            } else if k < 5.0 {
                FRAC_PI_4 + restricted_atan((x - 1.0) / (1.0 + x))
            } else if k < 10.0 {
                ATAN_FRAC_3_2 + restricted_atan((x - 1.5) / (1.0 + 1.5 * x))
            } else {
                FRAC_PI_2 - restricted_atan(x.recip())
            };

            if self.is_sign_positive() {
                result
            } else {
                -result
            }
        }
    }

    /// Computes the four quadrant arctangent of `self` (y) and `other` (x)
    /// in radians.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let y = TwoFloat::from(-1.0);
    /// let x = TwoFloat::from(-1.0);
    /// let theta = TwoFloat::atan2(y, x);
    ///
    /// assert!((theta + 3.0 * twofloat::consts::FRAC_PI_4).abs() < 1e-10);
    pub fn atan2(self, other: Self) -> Self {
        if self.hi == 0.0 {
            if other.hi.is_sign_positive() {
                Self::from(0.0)
            } else if self.hi.is_sign_positive() {
                PI
            } else {
                -PI
            }
        } else if other.hi == 0.0 {
            if self.hi.is_sign_positive() {
                FRAC_PI_2
            } else {
                -FRAC_PI_2
            }
        } else {
            let a = (self / other).atan();
            if other.hi.is_sign_positive() {
                a
            } else if self.hi.is_sign_positive() {
                a + PI
            } else {
                a - PI
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::quadrant;
    use crate::{
        consts::{FRAC_PI_2, FRAC_PI_4, PI},
        TwoFloat,
    };

    const THRESHOLD: f64 = 1e-10;

    #[test]
    fn quadrant_test() {
        assert_eq!(0, quadrant(TwoFloat::from(0.5)).1);
        assert_eq!(0, quadrant(TwoFloat::from(-0.5)).1);

        assert_eq!(1, quadrant(TwoFloat::from(2.0)).1);
        assert_eq!(3, quadrant(TwoFloat::from(-2.0)).1);

        assert_eq!(2, quadrant(TwoFloat::from(3.14)).1);
        assert_eq!(2, quadrant(TwoFloat::from(-3.14)).1);

        assert_eq!(3, quadrant(TwoFloat::from(4.0)).1);
        assert_eq!(1, quadrant(TwoFloat::from(-4.0)).1);

        assert_eq!(0, quadrant(TwoFloat::from(6.0)).1);
        assert_eq!(0, quadrant(TwoFloat::from(-6.0)).1);
    }

    #[test]
    fn sin_test() {
        assert_eq!(0.0, TwoFloat::from(0.0).sin());

        assert!((0.5f64.sin() - TwoFloat::from(0.5).sin()).abs() < THRESHOLD);
        assert!((1.4f64.sin() - TwoFloat::from(1.4).sin()).abs() < THRESHOLD);
        assert!((3.0f64.sin() - TwoFloat::from(3.0).sin()).abs() < THRESHOLD);
        assert!((4.0f64.sin() - TwoFloat::from(4.0).sin()).abs() < THRESHOLD);
        assert!((6.0f64.sin() - TwoFloat::from(6.0).sin()).abs() < THRESHOLD);

        assert!((0.5f64.sin() + TwoFloat::from(-0.5).sin()).abs() < THRESHOLD);
        assert!((1.4f64.sin() + TwoFloat::from(-1.4).sin()).abs() < THRESHOLD);
        assert!((3.0f64.sin() + TwoFloat::from(-3.0).sin()).abs() < THRESHOLD);
        assert!((4.0f64.sin() + TwoFloat::from(-4.0).sin()).abs() < THRESHOLD);
        assert!((6.0f64.sin() + TwoFloat::from(-6.0).sin()).abs() < THRESHOLD);
    }

    #[test]
    fn cos_test() {
        assert_eq!(1.0, TwoFloat::from(0.0).cos());

        assert!((0.5f64.cos() - TwoFloat::from(0.5).cos()).abs() < THRESHOLD);
        assert!((1.4f64.cos() - TwoFloat::from(1.4).cos()).abs() < THRESHOLD);
        assert!((3.0f64.cos() - TwoFloat::from(3.0).cos()).abs() < THRESHOLD);
        assert!((4.0f64.cos() - TwoFloat::from(4.0).cos()).abs() < THRESHOLD);
        assert!((6.0f64.cos() - TwoFloat::from(6.0).cos()).abs() < THRESHOLD);

        assert!((0.5f64.cos() - TwoFloat::from(-0.5).cos()).abs() < THRESHOLD);
        assert!((1.4f64.cos() - TwoFloat::from(-1.4).cos()).abs() < THRESHOLD);
        assert!((3.0f64.cos() - TwoFloat::from(-3.0).cos()).abs() < THRESHOLD);
        assert!((4.0f64.cos() - TwoFloat::from(-4.0).cos()).abs() < THRESHOLD);
        assert!((6.0f64.cos() - TwoFloat::from(-6.0).cos()).abs() < THRESHOLD);
    }

    #[test]
    fn tan_test() {
        assert_eq!(0.0, TwoFloat::from(0.0).tan());

        assert!((0.5f64.tan() - TwoFloat::from(0.5).tan()).abs() < THRESHOLD);
        assert!((1.4f64.tan() - TwoFloat::from(1.4).tan()).abs() < THRESHOLD);
        assert!((3.0f64.tan() - TwoFloat::from(3.0).tan()).abs() < THRESHOLD);
        assert!((4.0f64.tan() - TwoFloat::from(4.0).tan()).abs() < THRESHOLD);
        assert!((6.0f64.tan() - TwoFloat::from(6.0).tan()).abs() < THRESHOLD);

        assert!((0.5f64.tan() + TwoFloat::from(-0.5).tan()).abs() < THRESHOLD);
        assert!((1.4f64.tan() + TwoFloat::from(-1.4).tan()).abs() < THRESHOLD);
        assert!((3.0f64.tan() + TwoFloat::from(-3.0).tan()).abs() < THRESHOLD);
        assert!((4.0f64.tan() + TwoFloat::from(-4.0).tan()).abs() < THRESHOLD);
        assert!((6.0f64.tan() + TwoFloat::from(-6.0).tan()).abs() < THRESHOLD);
    }

    #[test]
    fn asin_test() {
        assert_eq!(0.0, TwoFloat::from(0.0).asin());
        assert!((0.25f64.asin() - TwoFloat::from(0.25).asin()) < THRESHOLD);
        assert!((0.75f64.asin() - TwoFloat::from(0.75).asin()) < THRESHOLD);
        assert!((TwoFloat::from(1.0).asin() - FRAC_PI_2).abs() < THRESHOLD);

        assert!((0.25f64.asin() + TwoFloat::from(-0.25).asin()) < THRESHOLD);
        assert!((0.75f64.asin() + TwoFloat::from(-0.75).asin()) < THRESHOLD);
        assert!((TwoFloat::from(-1.0).asin() + FRAC_PI_2).abs() < THRESHOLD);
    }

    #[test]
    fn acos_test() {
        assert!((TwoFloat::from(0.0).acos() - FRAC_PI_2).abs() < THRESHOLD);

        assert!((0.25f64.acos() - TwoFloat::from(0.25).acos()) < THRESHOLD);
        assert!((0.75f64.acos() - TwoFloat::from(0.75).acos()) < THRESHOLD);
        assert_eq!(0.0, TwoFloat::from(1.0).acos());

        assert!((0.25f64.asin() - TwoFloat::from(-0.25).acos()) < THRESHOLD);
        assert!((0.75f64.asin() - TwoFloat::from(-0.75).acos()) < THRESHOLD);
        assert!((TwoFloat::from(-1.0).acos() - PI).abs() < THRESHOLD);
    }

    #[test]
    fn atan_test() {
        assert_eq!(0.0, TwoFloat::from(0.0).atan());

        assert!((0.25f64.atan() - TwoFloat::from(0.25).atan()).abs() < THRESHOLD);
        assert!((0.5f64.atan() - TwoFloat::from(0.5).atan()).abs() < THRESHOLD);
        assert!((FRAC_PI_4 - TwoFloat::from(1.0).atan()).abs() < THRESHOLD);
        assert!((2.25f64.atan() - TwoFloat::from(2.25).atan()).abs() < THRESHOLD);
        assert!((10.0f64.atan() - TwoFloat::from(10.0).atan()).abs() < THRESHOLD);

        assert!((0.25f64.atan() + TwoFloat::from(-0.25).atan()).abs() < THRESHOLD);
        assert!((0.5f64.atan() + TwoFloat::from(-0.5).atan()).abs() < THRESHOLD);
        assert!((FRAC_PI_4 + TwoFloat::from(-1.0).atan()).abs() < THRESHOLD);
        assert!((2.25f64.atan() + TwoFloat::from(-2.25).atan()).abs() < THRESHOLD);
        assert!((10.0f64.atan() + TwoFloat::from(-10.0).atan()).abs() < THRESHOLD);
    }

    #[test]
    fn atan2_test() {
        assert_eq!(0.0, TwoFloat::from(0.0).atan2(TwoFloat::from(0.0)));
        assert_eq!(0.0, TwoFloat::from(0.0).atan2(TwoFloat::from(1.0)));
        assert_eq!(PI, TwoFloat::from(0.0).atan2(TwoFloat::from(-1.0)));
        assert_eq!(-PI, TwoFloat::from(-0.0).atan2(TwoFloat::from(-1.0)));
        assert_eq!(FRAC_PI_2, TwoFloat::from(1.0).atan2(TwoFloat::from(0.0)));
        assert_eq!(-FRAC_PI_2, TwoFloat::from(-1.0).atan2(TwoFloat::from(0.0)));
        assert!(
            (0.73f64.atan2(0.21f64) - TwoFloat::from(0.73).atan2(TwoFloat::from(0.21))).abs()
                < THRESHOLD
        );
    }
}
