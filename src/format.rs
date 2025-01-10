use core::fmt;
use rug::float::Round;
use rug::ops::DivAssignRound;
use rug::Float;

use crate::TwoFloat;

impl fmt::Display for TwoFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let num = self.to_rug_float();
        if f.sign_plus() {
            match f.precision() {
                Some(p) => write!(f, "{:+.*}", p, num,),
                None => write!(f, "{:+.32}", num),
            }
        } else {
            match f.precision() {
                Some(p) => write!(f, "{:.*}", p, num),
                None => write!(f, "{:.32}", num),
            }
        }
    }
}

impl TwoFloat {
    /// Compute number using BigFloat assuming a mantisse of size 53*2-1
    fn to_rug_float(&self) -> Float {
        let p = 106; // precision
        let mut num: Float = Float::with_val(p, 0.0);

        for float in [self.hi, self.lo] {
            let (mut m, e) = libm::frexp(float);
            let mut f2: Float = match m.signum() {
                1.0 => Float::with_val(p, 2f64.powi(e)),
                -1.0 => -Float::with_val(p, 2f64.powi(e)),
                _ => panic!("Not Implemented"),
            };
            let mut b: f64;
            while m != 0.0 {
                m *= 2.0;
                f2.div_assign_round(2, Round::Nearest);
                (m, b) = libm::modf(m);
                if b.abs() == 1.0 {
                    num += f2.clone();
                }
            }
        }
        // Format String to output by reducing the significant digits to 32
        num
    }
}

impl fmt::LowerExp for TwoFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let num = self.to_rug_float();
        if f.sign_plus() {
            match f.precision() {
                Some(p) => write!(f, "{:+.*e}", p, num,),
                None => write!(f, "{:+.32e}", num),
            }
        } else {
            match f.precision() {
                Some(p) => write!(f, "{:.*e}", p, num),
                None => write!(f, "{:.32e}", num),
            }
        }
    }
}

impl fmt::UpperExp for TwoFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let num = self.to_rug_float();
        if f.sign_plus() {
            match f.precision() {
                Some(p) => write!(f, "{:+.*E}", p, num,),
                None => write!(f, "{:+.32E}", num),
            }
        } else {
            match f.precision() {
                Some(p) => write!(f, "{:.*E}", p, num),
                None => write!(f, "{:.32E}", num),
            }
        }
    }
}

#[cfg(all(feature = "std", test))]
mod test {
    use crate::TwoFloat;

    #[test]
    fn display_test() {
        let value = TwoFloat { hi: 1.0, lo: 0.4 };
        assert_eq!(format!("{:.29}", value), "1.4000000000000000222044604925");
        assert_eq!(format!("{:.29}", -value), "-1.4000000000000000222044604925");
        assert_eq!(format!("{:+.10}", value), "+1.400000000");
        assert_eq!(format!("{:.2}", value), "1.4");
        assert_eq!(format!("{:.2}", -value), "-1.4");
        assert_eq!(format!("{:+.2}", value), "+1.4");
    }

    #[test]
    fn lowerexp_test() {
        let value = TwoFloat { hi: 1.0, lo: -0.3 };
        assert_eq!(
            format!("{:e}", value),
            "7.0000000000000001110223024625157e-1"
        );
        assert_eq!(
            format!("{:e}", -value),
            "-7.0000000000000001110223024625157e-1"
        );
        assert_eq!(
            format!("{:+e}", value),
            "+7.0000000000000001110223024625157e-1"
        );
        assert_eq!(format!("{:.2e}", value), "7.0e-1");
        assert_eq!(format!("{:.2e}", -value), "-7.0e-1");
        assert_eq!(format!("{:+.2e}", value), "+7.0e-1");
    }

    #[test]
    fn upperexp_test() {
        let value = TwoFloat { hi: 1.0, lo: 0.3 };
        assert_eq!(
            format!("{:E}", value),
            "1.2999999999999999888977697537484E0"
        );
        assert_eq!(
            format!("{:E}", -value),
            "-1.2999999999999999888977697537484E0"
        );
        assert_eq!(
            format!("{:+E}", value),
            "+1.2999999999999999888977697537484E0"
        );
        assert_eq!(format!("{:.2E}", value), "1.3E0");
        assert_eq!(format!("{:.2E}", -value), "-1.3E0");
        assert_eq!(format!("{:+.2E}", value), "+1.3E0");
    }

    #[test]
    fn precision_test() {
        // Check representation of TwoFloat with a mantissa representing 2^-200
        let a = TwoFloat::new_add(2f64.powi(-200), 0.0);
        assert_eq!(format!("{}", a), "6.2230152778611417071440640537801e-61");
    }
}
