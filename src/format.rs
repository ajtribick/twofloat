use core::fmt;

use crate::TwoFloat;

impl fmt::Display for TwoFloat {
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
                    "{:+.*} {} {:.*}",
                    p,
                    self.hi,
                    sign_char,
                    p,
                    libm::fabs(self.lo)
                ),
                None => write!(f, "{:+} {} {}", self.hi, sign_char, libm::fabs(self.lo)),
            }
        } else {
            match f.precision() {
                Some(p) => write!(
                    f,
                    "{:.*} {} {:.*}",
                    p,
                    self.hi,
                    sign_char,
                    p,
                    libm::fabs(self.lo)
                ),
                None => write!(f, "{} {} {}", self.hi, sign_char, libm::fabs(self.lo)),
            }
        }
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
        let value = TwoFloat { hi: 1.0, lo: 0.3 };
        assert_eq!(format!("{}", value), "1 + 0.3");
        assert_eq!(format!("{}", -value), "-1 - 0.3");
        assert_eq!(format!("{:+}", value), "+1 + 0.3");
        assert_eq!(format!("{:.2}", value), "1.00 + 0.30");
        assert_eq!(format!("{:.2}", -value), "-1.00 - 0.30");
        assert_eq!(format!("{:+.2}", value), "+1.00 + 0.30");
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
