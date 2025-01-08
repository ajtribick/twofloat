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
    /// assert!((b - e2).abs() / e2 < 1e-30);
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

            let k = libm::trunc((FRAC_1_LN_2 * self).hi - 0.5);
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
    /// # use core::{convert::TryFrom};
    /// let a = TwoFloat::from(2f64.powi(-20));
    ///
    /// let b = a.exp_m1();
    /// let c = a.exp() - 1.0;
    ///
    /// // Exact Result
    /// // res = 9.5367477115374544678824955687428e-7;
    /// let res = TwoFloat::try_from((9.5367477115374552e-07, -7.0551613072428143e-23)).unwrap();
    ///
    /// assert!(((b-res)/res) == 0.0);
    /// assert!(((c-res)/res).abs() < 1e-22);
    /// ```
    pub fn exp_m1(self) -> Self {
        if self < -LN_2 || self > LN_FRAC_3_2 {
            self.exp() - 1.0
        } else {
            let x = self.abs();
            let r = polynomial!(x, 1.0, FRAC_FACT[2..15]);
            if self < 0.0 {
                self * r * self.exp()
            } else {
                self * r
            }
        }
    }

    /// Returns `2^(self)`.
    ///
    /// where self = k + r * n,  k > 0 and n = 2^9 = 512
    /// The taylor series for the small value of r converges very fast
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(0.5).exp2();
    /// let b = TwoFloat::from(2).sqrt();
    /// let c = (TwoFloat::from(0.5)*twofloat::consts::LN_2).exp();
    /// let res = twofloat::consts::SQRT_2;
    ///
    /// assert!((a - res).abs() < 1e-29);
    /// assert!((b - res).abs() < 1e-31);
    /// assert!((c - res).abs() < 1e-30);
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
            let r = (self - k) * LN_2 / 512.0;
            //let x = self * LN_2;
            let mut r1 = polynomial!(r, FRAC_FACT[..12]);

            // Recover rescaling of r
            r1 = r1 * r1; // 2^(r * 2)
            r1 = r1 * r1; // 2^(r * 4)
            r1 = r1 * r1; // 2^(r * 8)
            r1 = r1 * r1; // 2^(r * 16)
            r1 = r1 * r1; // 2^(r * 32)
            r1 = r1 * r1; // 2^(r * 64)
            r1 = r1 * r1; // 2^(r * 128)
            r1 = r1 * r1; // 2^(r * 256)
            r1 = r1 * r1; // 2^(r * 512)

            //let r1 = polynomial!(r, 1.0, EXP2_COEFFS);
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
    /// assert!((a - 1.0).abs() < 1e-29);
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
    /// Uses Newton–Raphson iteration which depends on the `expm1` function
    ///
    /// # Example
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(-0.5);
    /// let b = a.ln_1p();
    /// let c = -twofloat::consts::LN_2;//0.1f64.ln_1p();
    /// assert!((b - c).abs() < 1e-29);
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
    /// assert!(a - 6.0 == 0.0, "{}", a);
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
    /// assert!((a - 2.0).abs() < 1e-30, "{}", a);
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
