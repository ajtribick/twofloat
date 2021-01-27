use crate::{TwoFloat, consts::LN_2};

// 1/ln(2)
const FRAC_1_LN_2: TwoFloat = TwoFloat {
    hi: 1.4426950408889634,
    lo: 2.0355273740931033e-17,
};

// ln(10)
const LN_10: TwoFloat = TwoFloat {
    hi: 2.302585092994046,
    lo: -2.1707562233822494e-16,
};

// ln(3/2)
const LN_FRAC_3_2: TwoFloat = TwoFloat {
    hi: 0.4054651081081644,
    lo: -2.8811380259626426e-18,
};

// limits
const EXP_UPPER_LIMIT: f64 = 709.782712893384;
const EXP_LOWER_LIMIT: f64 = -745.1332191019412;

// Coefficients for polynomial approximation of x*(exp(x)+1)/(exp(x)-1)
const EXP_COEFFS: [TwoFloat; 6] = [
    TwoFloat {
        hi: 0.16666666666666666,
        lo: 8.301559840894034e-18,
    },
    TwoFloat {
        hi: -0.0027777777777776512,
        lo: 1.1664268064351513e-19,
    },
    TwoFloat {
        hi: 6.613756613123634e-05,
        lo: 3.613966532258593e-21,
    },
    TwoFloat {
        hi: -1.6534390027595268e-06,
        lo: 2.6408090483313454e-23,
    },
    TwoFloat {
        hi: 4.175167193059256e-08,
        lo: -2.949837910669653e-24,
    },
    TwoFloat {
        hi: -1.0456596683715461e-09,
        lo: -9.375356618962057e-26,
    },
];

const EXP_M1_COEFFS: [TwoFloat; 12] = [
    TwoFloat {
        hi: 0.5,
        lo: 2.4147853280441852e-17,
    },
    TwoFloat {
        hi: 0.1666666666666666,
        lo: -1.0126242119486759e-17,
    },
    TwoFloat {
        hi: 0.04166666666666422,
        lo: -3.1228374568246144e-18,
    },
    TwoFloat {
        hi: 0.00833333333333542,
        lo: 5.220576901966033e-19,
    },
    TwoFloat {
        hi: 0.0013888888889601945,
        lo: -6.710675996026077e-20,
    },
    TwoFloat {
        hi: 0.00019841269843008256,
        lo: 7.744211626999651e-22,
    },
    TwoFloat {
        hi: 2.4801586443947164e-05,
        lo: 1.2547780925434967e-21,
    },
    TwoFloat {
        hi: 2.755731053267803e-06,
        lo: -1.0646797628167905e-23,
    },
    TwoFloat {
        hi: 2.755775448074009e-07,
        lo: -8.322155420550663e-24,
    },
    TwoFloat {
        hi: 2.505957492948909e-08,
        lo: -5.793840740452254e-25,
    },
    TwoFloat {
        hi: 2.0819091856857293e-09,
        lo: -3.914736871893147e-26,
    },
    TwoFloat {
        hi: 1.412762092583865e-10,
        lo: 2.8362941933894766e-27,
    },
];

// Coefficients for polynomial approximation of 2^x on [-0.5, 0.5]
const EXP2_COEFFS: [TwoFloat; 14] = [
    TwoFloat {
        hi: 0.6931471805599453,
        lo: 2.3190643482818175e-17,
    },
    TwoFloat {
        hi: 0.24022650695910072,
        lo: -9.493874028535377e-18,
    },
    TwoFloat {
        hi: 0.05550410866482158,
        lo: -3.192006150784052e-18,
    },
    TwoFloat {
        hi: 0.009618129107628477,
        lo: 2.7893564000728505e-19,
    },
    TwoFloat {
        hi: 0.0013333558146428454,
        lo: 6.081607270943552e-20,
    },
    TwoFloat {
        hi: 0.00015403530393381622,
        lo: 1.481577195325289e-21,
    },
    TwoFloat {
        hi: 1.5252733804038298e-05,
        lo: -5.2457376024944265e-22,
    },
    TwoFloat {
        hi: 1.3215486790126266e-06,
        lo: 5.182514493196138e-23,
    },
    TwoFloat {
        hi: 1.0178086030302029e-07,
        lo: -9.783640823461531e-25,
    },
    TwoFloat {
        hi: 7.054911635035306e-09,
        lo: -1.8596402662109374e-25,
    },
    TwoFloat {
        hi: 4.445527244701503e-10,
        lo: 2.2736481131821772e-26,
    },
    TwoFloat {
        hi: 2.567837336021261e-11,
        lo: -5.1428171225883105e-28,
    },
    TwoFloat {
        hi: 1.3720884847209076e-12,
        lo: 8.060868050272103e-29,
    },
    TwoFloat {
        hi: 6.793159072191384e-14,
        lo: 5.621628913936963e-30,
    },
];

fn mul_pow2(mut x: f64, mut y: i32) -> f64 {
    loop {
        if y < -1074 {
            x *= f64::from_bits(1u64);
            y += 1074;
        } else if y < -1022 {
            return x * f64::from_bits(1u64 << (y + 1074));
        } else if y < 1024 {
            return x * f64::from_bits(((y + 1023) as u64) << 52);
        } else {
            x *= f64::from_bits(0x7fe << 52);
            y -= 1023;
        }
    }
}

impl TwoFloat {
    /// Returns `e^(self)`, (the exponential function).
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(2.0);
    /// let b = a.exp();
    /// let e2 = twofloat::consts::E * twofloat::consts::E;
    ///
    /// assert!((b - e2).abs() / e2 < 1e-16);
    pub fn exp(self) -> Self {
        if self.hi <= EXP_LOWER_LIMIT {
            Self::from(0.0)
        } else if self.hi >= EXP_UPPER_LIMIT {
            Self {
                hi: f64::INFINITY,
                lo: 0.0,
            }
        } else if self.hi == 0.0 {
            Self::from(1.0)
        } else {
            // reduce value to range |r| <= ln(2)/2
            // where self = k*ln(2) + r

            let k = ((FRAC_1_LN_2 * self).hi + 0.5).trunc();
            let r = self - LN_2 * k;

            // Now approximate the function
            //
            // R(r^2) = r*(exp(r)+1)/(exp(r)-1) = 2 + P1*r^2 + P2*r^4 + ...
            //
            // using a polynomial obtained by the Remez algorithm on the
            // interval [0, ln(2)/2], then:
            //
            // exp(r) = 1 + 2*r/(R-r) = 1 + r + (r*R1) / (2-R1)
            //
            // where R1 = r - (P1*r^2 + P2*r^4 + ...)

            let rr = r * r;
            let r1 = r - rr * polynomial!(rr, EXP_COEFFS);

            let exp_r = 1.0 - ((r * r1) / (r1 - 2.0) - r);

            // then scale back

            if k == 0.0 {
                exp_r
            } else {
                Self {
                    hi: mul_pow2(exp_r.hi, k as i32),
                    lo: mul_pow2(exp_r.lo, k as i32),
                }
            }
        }
    }

    /// Returns `e^(self) - 1` in a way that provides additional accuracy
    /// when the value is close to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(0.05);
    /// let b = a.exp_m1();
    /// let c = 0.05f64.exp_m1();
    ///
    /// assert!((b - c).abs() < 1e-16);
    pub fn exp_m1(self) -> Self {
        if self < -LN_2 || self > LN_FRAC_3_2 {
            self.exp() - 1.0
        } else {
            let r = polynomial!(self, 1.0, EXP_M1_COEFFS);
            self * r
        }
    }

    /// Returns `2^(self)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(0.5).exp2();
    /// let b = TwoFloat::from(2).sqrt();
    ///
    /// assert!((a - b).abs() < 1e-15);
    pub fn exp2(self) -> Self {
        if self < -1074.0 {
            Self::from(0.0)
        } else if self >= 1023.0 {
            Self {
                hi: f64::INFINITY,
                lo: f64::INFINITY,
            }
        } else {
            let k = self.hi.round();
            let r = self - k;
            let r1 = polynomial!(r, 1.0, EXP2_COEFFS);
            if k == 0.0 {
                r1
            } else {
                Self {
                    hi: mul_pow2(r1.hi, k as i32),
                    lo: mul_pow2(r1.lo, k as i32),
                }
            }
        }
    }

    /// Returns the natural logarithm of the value.
    ///
    /// Uses Newton–Raphson iteration which depends on the `exp` function, so
    /// may not be fully accurate to the full precision of a `TwoFloat`.
    ///
    /// # Example
    ///
    /// ```
    /// let a = twofloat::consts::E.ln();
    /// assert!((a - 1.0).abs() < 1e-11);
    pub fn ln(self) -> Self {
        if self == 1.0 {
            Self::from(0.0)
        } else if self <= 0.0 {
            Self::NAN
        } else {
            let mut x = Self::from(self.hi.ln());
            x += self * (-x).exp() - 1.0;
            x + self * (-x).exp() - 1.0
        }
    }

    /// Returns the natural logarithm of `1 + self`.
    ///
    /// Uses Newton–Raphson iteration which depends on the `expm1` function,
    /// so may not be fully accurate to the full precision of a `TwoFloat`.
    ///
    /// # Example
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(0.1);
    /// let b = a.ln_1p();
    /// let c = 0.1f64.ln_1p();
    /// assert!((b - c).abs() < 1e-10);
    pub fn ln_1p(self) -> Self {
        if self == 0.0 {
            Self::from(0.0)
        } else if self <= -1.0 {
            Self::NAN
        } else {
            let mut x = Self::from(self.hi.ln_1p());
            let mut e = x.exp_m1();
            x -= (e - self) / (e + 1.0);
            e = x.exp_m1();
            x - (e - self) / (e + 1.0)
        }
    }

    /// Returns the logarithm of the number with respect to an arbitrary base.
    ///
    /// This is a convenience method that computes `self.ln() / base.ln()`, no
    /// additional accuracy is provided.
    ///
    /// # Examples
    ///
    /// let a = TwoFloat::from(81.0);
    /// let b = TwoFloat::from(3.0);
    /// let c = TwoFloat::log(a, b);
    ///
    /// assert!((c - 4.0).abs() < 1e-12);
    pub fn log(self, base: Self) -> Self {
        self.ln() / base.ln()
    }

    /// Returns the base 2 logarithm of the number.
    ///
    /// Uses Newton–Raphson iteration which depends on the `exp2` function,
    /// so may not be fully accurate to the full precision of a `TwoFloat`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(64.0).log2();
    ///
    /// assert!((a - 6.0).abs() < 1e-12, "{}", a);
    pub fn log2(self) -> Self {
        if self == 1.0 {
            Self::from(1.0)
        } else if self <= 0.0 {
            Self::NAN
        } else {
            let mut x = Self::from(self.hi.log2());
            x += (self * (-x).exp2() - 1.0) * FRAC_1_LN_2;
            x + (self * (-x).exp2() - 1.0) * FRAC_1_LN_2
        }
    }

    /// Returns the base 10 logarithm of the number.
    ///
    /// This is a convenience method that computes `self.ln() / LN_10`, no
    /// additional accuracy is provided.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(100.0).log10();
    ///
    /// assert!((a - 2.0).abs() < 1e-12);
    pub fn log10(self) -> Self {
        self.ln() / LN_10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exp_test() {
        assert_eq!(
            TwoFloat::from(-1000.0).exp(),
            0.0,
            "Large negative exponent produced non-zero value"
        );
        assert!(
            !TwoFloat::from(1000.0).exp().is_valid(),
            "Large positive exponent produced valid value"
        );
        assert_eq!(
            TwoFloat::from(0.0).exp(),
            TwoFloat::from(1.0),
            "exp(0) did not return 1"
        );
    }

    #[test]
    fn ln_test() {
        assert!(
            !TwoFloat::from(0.0).ln().is_valid(),
            "ln(0) produced valid result"
        );
        assert!(
            !TwoFloat::from(-5.0).ln().is_valid(),
            "ln(negative) produced valid result"
        );
        assert_eq!(
            TwoFloat::from(1.0).ln(),
            TwoFloat::from(0.0),
            "ln(1) did not return 0"
        );
    }
}
