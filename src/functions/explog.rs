use hexf::hexf64;

use crate::{consts::LN_2, TwoFloat};

// 1/ln(2)
const FRAC_1_LN_2: TwoFloat = TwoFloat {
    hi: hexf64!("0x1.71547652b82fep0"),
    lo: hexf64!("0x1.777d0ffda0d24p-56"),
};

// ln(10)
const LN_10: TwoFloat = TwoFloat {
    hi: hexf64!("0x1.26bb1bbb55516p1"),
    lo: hexf64!("-0x1.f48ad494ea3e9p-53"),
};

// ln(3/2)
const LN_FRAC_3_2: TwoFloat = TwoFloat {
    hi: hexf64!("0x1.9f323ecbf984cp-2"),
    lo: hexf64!("-0x1.a92e513217f5cp-59"),
};

// limits
const EXP_UPPER_LIMIT: f64 = hexf64!("0x1.62e42fefa39efp9"); // ln(0x1.0p1024)
const EXP_LOWER_LIMIT: f64 = hexf64!("-0x1.74385446d71c3p9"); // ln(0x1.0p-1074)

const EXP_M1_COEFFS: [TwoFloat; 12] = [
    TwoFloat {
        hi: hexf64!("0x1.0p-1"),
        lo: hexf64!("0x1.bd730351a9755p-56"),
    },
    TwoFloat {
        hi: hexf64!("0x1.5555555555553p-3"),
        lo: hexf64!("-0x1.7597a71b9af89p-57"),
    },
    TwoFloat {
        hi: hexf64!("0x1.55555555553f5p-5"),
        lo: hexf64!("-0x1.ccd976a7f775cp-59"),
    },
    TwoFloat {
        hi: hexf64!("0x1.11111111115c4p-7"),
        lo: hexf64!("0x1.342b20ac16f97p-61"),
    },
    TwoFloat {
        hi: hexf64!("0x1.6c16c16c6709ep-10"),
        lo: hexf64!("-0x1.3ce71843eff0cp-64"),
    },
    TwoFloat {
        hi: hexf64!("0x1.a01a01a0b696cp-13"),
        lo: hexf64!("0x1.d41bdeddcef57p-71"),
    },
    TwoFloat {
        hi: hexf64!("0x1.a01a00aeb2858p-16"),
        lo: hexf64!("0x1.7b3bc0a8a9fafp-70"),
    },
    TwoFloat {
        hi: hexf64!("0x1.71de32b050a9dp-19"),
        lo: hexf64!("-0x1.9be0c6cec6271p-77"),
    },
    TwoFloat {
        hi: hexf64!("0x1.27e62dc06cd67p-22"),
        lo: hexf64!("-0x1.41f2a2a0cba43p-77"),
    },
    TwoFloat {
        hi: hexf64!("0x1.ae852d1420eefp-26"),
        lo: hexf64!("-0x1.669f123719ab2p-81"),
    },
    TwoFloat {
        hi: hexf64!("0x1.1e22aadda1973p-29"),
        lo: hexf64!("-0x1.83b25ef3d0968p-85"),
    },
    TwoFloat {
        hi: hexf64!("0x1.36ab6f77c95d8p-33"),
        lo: hexf64!("0x1.c16dc2dc455f1p-89"),
    },
];

// Coefficients for polynomial approximation of 2^x on [-0.5, 0.5]
const EXP2_COEFFS: [TwoFloat; 14] = [
    TwoFloat {
        hi: hexf64!("0x1.62e42fefa39efp-1"),
        lo: hexf64!("0x1.abcab7ae0b156p-56"),
    },
    TwoFloat {
        hi: hexf64!("0x1.ebfbdff82c58fp-3"),
        lo: hexf64!("-0x1.5e431ae1ed823p-57"),
    },
    TwoFloat {
        hi: hexf64!("0x1.c6b08d704a0cp-5"),
        lo: hexf64!("-0x1.d70e953766cd4p-59"),
    },
    TwoFloat {
        hi: hexf64!("0x1.3b2ab6fba4e77p-7"),
        lo: hexf64!("0x1.494f1fd2611efp-62"),
    },
    TwoFloat {
        hi: hexf64!("0x1.5d87fe78a6736p-10"),
        lo: hexf64!("0x1.1f321edc1a3bbp-64"),
    },
    TwoFloat {
        hi: hexf64!("0x1.430912f86c78cp-13"),
        lo: hexf64!("0x1.bfc77bb3c115bp-70"),
    },
    TwoFloat {
        hi: hexf64!("0x1.ffcbfc5887f1ap-17"),
        lo: hexf64!("-0x1.3d15db905a7ddp-71"),
    },
    TwoFloat {
        hi: hexf64!("0x1.62c0223a5a6dbp-20"),
        lo: hexf64!("0x1.f538d80a3aae8p-75"),
    },
    TwoFloat {
        hi: hexf64!("0x1.b5253d488bccap-24"),
        lo: hexf64!("-0x1.2ec9fd0f44ecfp-80"),
    },
    TwoFloat {
        hi: hexf64!("0x1.e4cf5169221d1p-28"),
        lo: hexf64!("-0x1.cc6cb479cd318p-83"),
    },
    TwoFloat {
        hi: hexf64!("0x1.e8ca77bf9238ep-32"),
        lo: hexf64!("0x1.c257a7e383648p-86"),
    },
    TwoFloat {
        hi: hexf64!("0x1.c3bd1cd9ae17dp-36"),
        lo: hexf64!("-0x1.45f6fa8d3cb45p-91"),
    },
    TwoFloat {
        hi: hexf64!("0x1.8235651fc7049p-40"),
        lo: hexf64!("0x1.98bc0cb4f5bc4p-94"),
    },
    TwoFloat {
        hi: hexf64!("0x1.31efcab273719p-44"),
        lo: hexf64!("0x1.c814aa232482ap-98"),
    },
];

const FRAC_FACT: [TwoFloat; 21] = [
    TwoFloat {
        // 1/0!
        hi: hexf64!("0x1.0000000000000p+0"),
        lo: hexf64!("0x0.0p+0"),
    },
    TwoFloat {
        // 1/1!
        hi: hexf64!("0x1.0000000000000p+0"),
        lo: hexf64!("0x0.0p+0"),
    },
    TwoFloat {
        // 1/2!
        hi: hexf64!("0x1.0000000000000p-1"),
        lo: hexf64!("0x0.0p+0"),
    },
    TwoFloat {
        // 1/3!
        hi: hexf64!("0x1.5555555555555p-3"),
        lo: hexf64!("0x1.5555555555555p-57"),
    },
    TwoFloat {
        // 1/4!
        hi: hexf64!("0x1.5555555555555p-5"),
        lo: hexf64!("0x1.5555555555555p-59"),
    },
    TwoFloat {
        // 1/5!
        hi: hexf64!("0x1.1111111111111p-7"),
        lo: hexf64!("0x1.1111111111111p-63"),
    },
    TwoFloat {
        // 1/6!
        hi: hexf64!("0x1.6c16c16c16c17p-10"),
        lo: hexf64!("-0x1.f49f49f49f49fp-65"),
    },
    TwoFloat {
        // 1/7!
        hi: hexf64!("0x1.a01a01a01a01ap-13"),
        lo: hexf64!("0x1.a01a01a01a01ap-73"),
    },
    TwoFloat {
        // 1/8!
        hi: hexf64!("0x1.a01a01a01a01ap-16"),
        lo: hexf64!("0x1.a01a01a01a01ap-76"),
    },
    TwoFloat {
        // 1/9!
        hi: hexf64!("0x1.71de3a556c734p-19"),
        lo: hexf64!("-0x1.c154f8ddc6c00p-73"),
    },
    TwoFloat {
        // 1/10!
        hi: hexf64!("0x1.27e4fb7789f5cp-22"),
        lo: hexf64!("0x1.cbbc05b4fa99ap-76"),
    },
    TwoFloat {
        // 1/11!
        hi: hexf64!("0x1.ae64567f544e4p-26"),
        lo: hexf64!("-0x1.c062e06d1f209p-80"),
    },
    TwoFloat {
        // 1/12!
        hi: hexf64!("0x1.1eed8eff8d898p-29"),
        lo: hexf64!("-0x1.2aec959e14c06p-83"),
    },
    TwoFloat {
        // 1/13!
        hi: hexf64!("0x1.6124613a86d09p-33"),
        lo: hexf64!("0x1.f28e0cc748ebep-87"),
    },
    TwoFloat {
        // 1/14!
        hi: hexf64!("0x1.93974a8c07c9dp-37"),
        lo: hexf64!("0x1.05d6f8a2efd1fp-92"),
    },
    TwoFloat {
        // 1/15!
        hi: hexf64!("0x1.ae7f3e733b81fp-41"),
        lo: hexf64!("0x1.1d8656b0ee8cbp-97"),
    },
    TwoFloat {
        // 1/16!
        hi: hexf64!("0x1.ae7f3e733b81fp-45"),
        lo: hexf64!("0x1.1d8656b0ee8cbp-101"),
    },
    TwoFloat {
        // 1/17!
        hi: hexf64!("0x1.952c77030ad4ap-49"),
        lo: hexf64!("0x1.ac981465ddc6cp-103"),
    },
    TwoFloat {
        // 1/18!
        hi: hexf64!("0x1.6827863b97d97p-53"),
        lo: hexf64!("0x1.eec01221a8b0bp-107"),
    },
    TwoFloat {
        // 1/19!
        hi: hexf64!("0x1.2f49b46814157p-57"),
        lo: hexf64!("0x1.2650f61dbdcb4p-112"),
    },
    TwoFloat {
        // 1/20!
        hi: hexf64!("0x1.e542ba4020225p-62"),
        lo: hexf64!("0x1.ea72b4afe3c2fp-120"),
    },
];

fn mul_pow2(mut x: f64, mut y: i32) -> f64 {
    loop {
        if y < -1074 {
            x *= hexf64!("0x1.0p-1074");
            y += 1074;
        } else if y < -1022 {
            return x * f64::from_bits(1u64 << (y + 1074));
        } else if y < 1024 {
            return x * f64::from_bits(((y + 1023) as u64) << 52);
        } else {
            x *= hexf64!("0x1.0p1023");
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
    /// ```
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
            // reduce value to range |n*r| <= ln(2)/2 ~ 0.347
            // where self = k*ln(2) + n*r
            // Therefore:
            //    exp(self) = 2^k * exp(r)^n
            // We can increase the value of `n` making the convergence of the
            // remaining exponential function faster

            let k = libm::trunc((FRAC_1_LN_2 * self).hi + 0.5);
            // n = 512 is chosen;
            let r = (self - LN_2 * k) / 512.0;
            // TODO: Redefine EPSILON ?
            let eps = Self::from(1e-32) / 512.0;

            let x = r;
            let mut p = x * x;
            let mut r = x + p * 0.5;
            // Step
            p *= x;
            let mut t = p * FRAC_FACT[3];
            let mut i = 3;
            loop {
                r += t;
                p *= x;
                i += 1;
                t = p * FRAC_FACT[i];
                if i >= 20 || t.abs() <= eps {
                    //dbg!(i, k);
                    break;
                }
            }
            r += 1.0 + t;

            // Recover rescaling of r
            r = r * r; // exp(r)^2
            r = r * r; // exp(r)^4
            r = r * r; // exp(r)^8
            r = r * r; // exp(r)^16
            r = r * r; // exp(r)^32
            r = r * r; // exp(r)^64
            r = r * r; // exp(r)^128
            r = r * r; // exp(r)^256
            r = r * r; // exp(r)^512

            if k == 0.0 {
                r
            } else {
                Self {
                    hi: mul_pow2(r.hi, k as i32),
                    lo: mul_pow2(r.lo, k as i32),
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
    /// ```
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
    /// ```
    pub fn exp2(self) -> Self {
        if self < -1074.0 {
            Self::from(0.0)
        } else if self >= 1023.0 {
            Self {
                hi: f64::INFINITY,
                lo: f64::INFINITY,
            }
        } else {
            let k = libm::round(self.hi);
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
    /// ```
    pub fn ln(self) -> Self {
        if self == 1.0 {
            Self::from(0.0)
        } else if self <= 0.0 {
            Self::NAN
        } else {
            let mut x = Self::from(libm::log(self.hi));
            x += self * (-x).exp() - 1.0;
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
    /// ```
    pub fn ln_1p(self) -> Self {
        if self == 0.0 {
            Self::from(0.0)
        } else if self <= -1.0 {
            Self::NAN
        } else {
            let mut x = Self::from(libm::log1p(self.hi));
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
    /// ```
    pub fn log2(self) -> Self {
        if self == 1.0 {
            Self::from(1.0)
        } else if self <= 0.0 {
            Self::NAN
        } else {
            let mut x = Self::from(libm::log2(self.hi));
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
    /// ```
    pub fn log10(self) -> Self {
        self.ln() / LN_10
    }
}

#[cfg(test)]
mod tests {
    use crate::TwoFloat;

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
