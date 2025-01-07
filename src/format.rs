use core::fmt;
use num_bigfloat::BigFloat;

use crate::TwoFloat;

impl fmt::Display for TwoFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.pretty_str())
    }
}

impl TwoFloat {
    // Compute number using BigFloat assuming a mantisse of size 53*2-1
    pub fn pretty_str(&self) -> String {
        let mut num: BigFloat = 0.0.into();
        for float in [self.hi, self.lo] {
            let (mut m, e) = libm::frexp(float);
            let mut f2: BigFloat = match m.signum() {
                1.0 => 2f64.powi(e),
                -1.0 => -(2f64.powi(e)),
                _ => panic!("Not Implemented"),
            }
            .into();
            let mut b: f64;
            while m != 0.0 {
                m *= 2.0;
                f2 /= num_bigfloat::TWO;
                (m, b) = libm::modf(m);
                if b.abs() == 1.0 {
                    num += f2;
                }
            }
        }
        // Format String to output by reducing the significant digits to 32
        let mut num_str = format!("{}", num);
        if !num.is_zero() {
            match num_str.find("e") {
                Some(41) | None => num_str.replace_range(33..41, ""), // Positive
                Some(42) => num_str.replace_range(34..42, ""),        // Negative
                _ => panic!("BigFloat should have 40 significant digits"),
            };
        }
        num_str
    }
}

impl fmt::LowerExp for TwoFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign_char = if self.lo().is_sign_positive() {
            '+'
        } else {
            '-'
        };
        if f.sign_plus() {
            match f.precision() {
                Some(p) => write!(
                    f,
                    "{:+.*e} {} {:.*e}",
                    p,
                    self.hi,
                    sign_char,
                    p,
                    libm::fabs(self.lo)
                ),
                None => write!(f, "{:+e} {} {:e}", self.hi, sign_char, libm::fabs(self.lo)),
            }
        } else {
            match f.precision() {
                Some(p) => write!(
                    f,
                    "{:.*e} {} {:.*e}",
                    p,
                    self.hi,
                    sign_char,
                    p,
                    libm::fabs(self.lo)
                ),
                None => write!(f, "{:e} {} {:e}", self.hi, sign_char, libm::fabs(self.lo)),
            }
        }
    }
}

impl fmt::UpperExp for TwoFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign_char = if self.lo().is_sign_positive() {
            '+'
        } else {
            '-'
        };
        if f.sign_plus() {
            match f.precision() {
                Some(p) => write!(
                    f,
                    "{:+.*E} {} {:.*E}",
                    p,
                    self.hi,
                    sign_char,
                    p,
                    libm::fabs(self.lo)
                ),
                None => write!(f, "{:+E} {} {:E}", self.hi, sign_char, libm::fabs(self.lo)),
            }
        } else {
            match f.precision() {
                Some(p) => write!(
                    f,
                    "{:.*E} {} {:.*E}",
                    p,
                    self.hi,
                    sign_char,
                    p,
                    libm::fabs(self.lo)
                ),
                None => write!(f, "{:E} {} {:E}", self.hi, sign_char, libm::fabs(self.lo)),
            }
        }
    }
}

#[cfg(all(feature = "std", test))]
mod test {
    use crate::TwoFloat;

    #[test]
    fn display_test() {
        let value = TwoFloat { hi: 1.0, lo: 3e-17 };
        assert_eq!(format!("{}", value), "1.0000000000000000300000000000000");
        assert_eq!(
            format!("{}", 100.0 * value),
            "1.0000000000000000300000000000000e+2"
        );
    }

    #[test]
    fn lowerexp_test() {
        let value = TwoFloat { hi: 1.0, lo: -0.3 };
        assert_eq!(format!("{:e}", value), "1e0 - 3e-1");
        assert_eq!(format!("{:e}", -value), "-1e0 + 3e-1");
        assert_eq!(format!("{:+e}", value), "+1e0 - 3e-1");
        assert_eq!(format!("{:.2e}", value), "1.00e0 - 3.00e-1");
        assert_eq!(format!("{:.2e}", -value), "-1.00e0 + 3.00e-1");
        assert_eq!(format!("{:+.2e}", value), "+1.00e0 - 3.00e-1");
    }

    #[test]
    fn upperexp_test() {
        let value = TwoFloat { hi: 1.0, lo: 0.3 };
        assert_eq!(format!("{:E}", value), "1E0 + 3E-1");
        assert_eq!(format!("{:E}", -value), "-1E0 - 3E-1");
        assert_eq!(format!("{:+E}", value), "+1E0 + 3E-1");
        assert_eq!(format!("{:.2E}", value), "1.00E0 + 3.00E-1");
        assert_eq!(format!("{:.2E}", -value), "-1.00E0 - 3.00E-1");
        assert_eq!(format!("{:+.2E}", value), "+1.00E0 + 3.00E-1");
    }
}
