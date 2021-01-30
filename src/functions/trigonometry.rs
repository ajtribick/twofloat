use core::convert::TryFrom;

use crate::{
    consts::{FRAC_PI_2, FRAC_PI_4, PI},
    TwoFloat,
};

// Polynomial coefficients of sin(x)-x on [0,pi/4]
const SIN_COEFFS: [TwoFloat; 7] = [
    TwoFloat {
        hi: -0.16666666666666666,
        lo: -8.51510705987379e-18,
    },
    TwoFloat {
        hi: 0.008333333333333312,
        lo: -1.3912016738387687e-19,
    },
    TwoFloat {
        hi: -0.00019841269841246198,
        lo: -7.681619205684898e-21,
    },
    TwoFloat {
        hi: 2.75573192105007e-06,
        lo: 2.955758642485038e-23,
    },
    TwoFloat {
        hi: -2.505210410444575e-08,
        lo: 9.269533560586216e-26,
    },
    TwoFloat {
        hi: 1.605827759011912e-10,
        lo: 3.404451553732099e-27,
    },
    TwoFloat {
        hi: -7.574792323977277e-13,
        lo: 4.727692438769333e-29,
    },
];

// Polynomial coefficients of cos(x)-1+x^2/2 on [0,pi/4]
const COS_COEFFS: [TwoFloat; 7] = [
    TwoFloat {
        hi: 0.041666666666666664,
        lo: 2.2440014013613353e-18,
    },
    TwoFloat {
        hi: -0.0013888888888888872,
        lo: 6.065718226973696e-20,
    },
    TwoFloat {
        hi: 2.4801587301569693e-05,
        lo: 7.112363744916272e-22,
    },
    TwoFloat {
        hi: -2.7557319214749576e-07,
        lo: -2.1630593346449427e-23,
    },
    TwoFloat {
        hi: 2.0876754247413408e-09,
        lo: -3.4443669660877135e-26,
    },
    TwoFloat {
        hi: -1.1470281608989357e-11,
        lo: 4.623853450729046e-28,
    },
    TwoFloat {
        hi: 4.737645013072795e-14,
        lo: 2.0519566094121702e-30,
    },
];

// Polynomial coefficients of tan(x)-x on [0,pi/4]
const TAN_COEFFS: [TwoFloat; 14] = [
    TwoFloat {
        hi: 0.333333333333301,
        lo: -1.6964192869147454e-17,
    },
    TwoFloat {
        hi: 0.133333333336424,
        lo: 1.2882344203768942e-17,
    },
    TwoFloat {
        hi: 0.053968253847554985,
        lo: 7.3568315143778935e-19,
    },
    TwoFloat {
        hi: 0.02186949110053143,
        lo: 7.506482205636934e-19,
    },
    TwoFloat {
        hi: 0.008863201837095791,
        lo: -1.6985713823531061e-19,
    },
    TwoFloat {
        hi: 0.0035924221451762235,
        lo: -3.783119522648438e-20,
    },
    TwoFloat {
        hi: 0.0014540539618521297,
        lo: -9.893196667739264e-20,
    },
    TwoFloat {
        hi: 0.000597689634752774,
        lo: -1.844774602740589e-20,
    },
    TwoFloat {
        hi: 0.00021542536600071578,
        lo: 2.5078450231357865e-21,
    },
    TwoFloat {
        hi: 0.00014954373126927091,
        lo: -4.9925826785968525e-21,
    },
    TwoFloat {
        hi: -4.3214610451232346e-05,
        lo: 1.3602116927481075e-22,
    },
    TwoFloat {
        hi: 0.00010374385393487309,
        lo: -1.7246199044466566e-21,
    },
    TwoFloat {
        hi: -5.2050985346847035e-05,
        lo: 3.038262431960992e-21,
    },
    TwoFloat {
        hi: 2.2476452033043005e-05,
        lo: -1.3763291484895173e-21,
    },
];

// Polynomial coefficients of asin(x)-x on [0,0.5]
const ASIN_COEFFS: [TwoFloat; 10] = [
    TwoFloat {
        hi: 0.16666666665760896,
        lo: 6.318909167472426e-18,
    },
    TwoFloat {
        hi: 0.07500000116177634,
        lo: -3.6535510436559885e-19,
    },
    TwoFloat {
        hi: 0.04464279716731413,
        lo: -7.858342616855284e-19,
    },
    TwoFloat {
        hi: 0.030383587986160417,
        lo: 2.910903680786144e-22,
    },
    TwoFloat {
        hi: 0.022345297689414505,
        lo: -1.4743699989653656e-18,
    },
    TwoFloat {
        hi: 0.017629659409590933,
        lo: -4.947235313007659e-19,
    },
    TwoFloat {
        hi: 0.012128557172711413,
        lo: 5.8362891350143e-19,
    },
    TwoFloat {
        hi: 0.019288181641610027,
        lo: -3.799209584634637e-19,
    },
    TwoFloat {
        hi: -0.009769012047538925,
        lo: -8.206627214129968e-20,
    },
    TwoFloat {
        hi: 0.033397748388563755,
        lo: -3.4307997519760322e-18,
    },
];

// Polynomial coefficients of atan(x) - x on [0, 7/16]
const ATAN_COEFFS: [TwoFloat; 15] = [
    TwoFloat {
        hi: -0.3333333333333333,
        lo: -1.8404711392378294e-17,
    },
    TwoFloat {
        hi: 0.19999999999999996,
        lo: 8.821841116322433e-18,
    },
    TwoFloat {
        hi: -0.1428571428571376,
        lo: -8.757692135391525e-18,
    },
    TwoFloat {
        hi: 0.11111111111068835,
        lo: -5.832462234788162e-19,
    },
    TwoFloat {
        hi: -0.090909090887859,
        lo: 6.0077049965516354e-18,
    },
    TwoFloat {
        hi: 0.0769230762073313,
        lo: -3.2884708888145337e-18,
    },
    TwoFloat {
        hi: -0.06666664969948971,
        lo: 4.320817735439753e-19,
    },
    TwoFloat {
        hi: 0.05882323803853899,
        lo: -2.805787242057251e-18,
    },
    TwoFloat {
        hi: -0.05262788771506685,
        lo: 2.9037477653961078e-18,
    },
    TwoFloat {
        hi: 0.04758423341314784,
        lo: 2.4230908396440816e-19,
    },
    TwoFloat {
        hi: -0.043233511131193926,
        lo: -1.7170865975388194e-18,
    },
    TwoFloat {
        hi: 0.038726167898553494,
        lo: -2.3827075835715594e-18,
    },
    TwoFloat {
        hi: -0.03220490340648736,
        lo: -2.5695532509697203e-18,
    },
    TwoFloat {
        hi: 0.021458037392206093,
        lo: -2.213497023050699e-20,
    },
    TwoFloat {
        hi: -0.00811325678548267,
        lo: 8.602995904603601e-19,
    },
];

const ATAN_FRAC_1_2: TwoFloat = TwoFloat {
    hi: 0.4636476090008061,
    lo: 2.2698777452961687e-17,
};
const ATAN_FRAC_3_2: TwoFloat = TwoFloat {
    hi: 0.982793723247329,
    lo: 1.3903311031230998e-17,
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
