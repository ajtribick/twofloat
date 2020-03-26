use crate::base::*;
use crate::consts::LN_2;

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

// limits
const EXP_UPPER_LIMIT: f64 = 709.782712893384;
const EXP_LOWER_LIMIT: f64 = -745.1332191019412;

// Coefficients for polynomial approximation of x*(exp(x)+1)/(exp(x)-1)

const P1: TwoFloat = TwoFloat {
    hi: 0.16666666666666666,
    lo: 8.301559840894034e-18,
};
const P2: TwoFloat = TwoFloat {
    hi: -0.0027777777777776512,
    lo: 1.1664268064351513e-19,
};
const P3: TwoFloat = TwoFloat {
    hi: 6.613756613123634e-05,
    lo: 3.613966532258593e-21,
};
const P4: TwoFloat = TwoFloat {
    hi: -1.6534390027595268e-06,
    lo: 2.6408090483313454e-23,
};
const P5: TwoFloat = TwoFloat {
    hi: 4.175167193059256e-08,
    lo: -2.949837910669653e-24,
};
const P6: TwoFloat = TwoFloat {
    hi: -1.0456596683715461e-09,
    lo: -9.375356618962057e-26,
};

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
    /// Note that this function returns an approximate value, in particular
    /// the low word of the core polynomial approximation is not guaranteed.
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
    pub fn exp(&self) -> TwoFloat {
        if self.hi <= EXP_LOWER_LIMIT {
            TwoFloat::from(0.0)
        } else if self.hi >= EXP_UPPER_LIMIT {
            TwoFloat {
                hi: std::f64::INFINITY,
                lo: 0.0,
            }
        } else if self.hi == 0.0 {
            TwoFloat::from(1.0)
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

            let rr = &r * &r;
            let r1 = &r - &rr * (P1 + &rr * (P2 + &rr * (P3 + &rr * (P4 + &rr * (P5 + &rr * P6)))));

            let exp_r = 1.0 - ((&r * &r1) / (&r1 - 2.0) - &r);

            // then scale back

            if k == 0.0 {
                exp_r
            } else {
                TwoFloat {
                    hi: mul_pow2(exp_r.hi, k as i32),
                    lo: mul_pow2(exp_r.lo, k as i32),
                }
            }
        }
    }

    /// Returns `2^(self)`.
    ///
    /// This is a convenience method that computes `(self * LN_2).exp()`, no
    /// additional accuracy is provided.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(6.0).exp2();
    ///
    /// assert!((a - 64.0).abs() < 1e-15);
    pub fn exp2(&self) -> TwoFloat {
        (self * LN_2).exp()
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
    pub fn ln(&self) -> TwoFloat {
        if *self == 1.0 {
            TwoFloat::from(0.0)
        } else if *self <= 0.0 {
            TwoFloat {
                hi: std::f64::NAN,
                lo: std::f64::NAN,
            }
        } else {
            let mut x = TwoFloat::from(self.hi.ln());
            x += self * (-x).exp() - 1.0;
            x + self * (-x).exp() - 1.0
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
    /// let c = a.log(&b);
    ///
    /// assert!((c - 4.0).abs() < 1e-12);
    pub fn log(&self, base: &TwoFloat) -> TwoFloat {
        self.ln() / base.ln()
    }

    /// Returns the base 2 logarithm of the number.
    ///
    /// This is a convenience method that computes `self.ln() / LN_2`, no
    /// additional accuracy is provided.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(64.0).log2();
    ///
    /// assert!((a - 6.0).abs() < 1e-12, "{}", a);
    pub fn log2(&self) -> TwoFloat {
        self.ln() / LN_2
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
    pub fn log10(&self) -> TwoFloat {
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
