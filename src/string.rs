#![cfg(feature = "string_convert")]

use core::{cmp::Ordering, fmt, str::FromStr};

use lazy_static::lazy_static;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{one, FromPrimitive, ToPrimitive};

use crate::{TwoFloat, TwoFloatError};

lazy_static! {
    static ref TEN: BigInt = BigInt::from(10);
}

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
                    self.lo.abs()
                ),
                None => write!(f, "{:+} {} {}", self.hi, sign_char, self.lo.abs()),
            }
        } else {
            match f.precision() {
                Some(p) => write!(f, "{:.*} {} {:.*}", p, self.hi, sign_char, p, self.lo.abs()),
                None => write!(f, "{} {} {}", self.hi, sign_char, self.lo.abs()),
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
                    self.lo.abs()
                ),
                None => write!(f, "{:+e} {} {:e}", self.hi, sign_char, self.lo.abs()),
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
                    self.lo.abs()
                ),
                None => write!(f, "{:e} {} {:e}", self.hi, sign_char, self.lo.abs()),
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
                    self.lo.abs()
                ),
                None => write!(f, "{:+E} {} {:E}", self.hi, sign_char, self.lo.abs()),
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
                    self.lo.abs()
                ),
                None => write!(f, "{:E} {} {:E}", self.hi, sign_char, self.lo.abs()),
            }
        }
    }
}

fn parse_rational(s: &str) -> Result<BigRational, TwoFloatError> {
    let mut point_pos = None;
    let mut exponent_pos = None;
    for (pos, b) in s.bytes().enumerate() {
        match b {
            b'.' if point_pos.is_some() => return Err(TwoFloatError::ParseError),
            b'.' => point_pos = Some(pos),
            b'E' | b'e' => {
                exponent_pos = Some(pos);
                break;
            }
            _ => (),
        }
    }

    let (v, exponent) = match exponent_pos {
        Some(e) => (
            &s[..e],
            s[e + 1..].parse().map_err(|_| TwoFloatError::ParseError)?,
        ),
        None => (s, 0),
    };

    let value = match point_pos {
        Some(p) => {
            BigRational::new(
                v[..p].parse().map_err(|_| TwoFloatError::ParseError)?,
                one(),
            ) + BigRational::new(
                v[p + 1..].parse().map_err(|_| TwoFloatError::ParseError)?,
                TEN.pow((v.len() - p - 1) as u32),
            )
        }
        None => BigRational::new(v.parse().map_err(|_| TwoFloatError::ParseError)?, one()),
    };

    Ok(match exponent.cmp(&0) {
        Ordering::Less => value * BigRational::new(one(), TEN.pow((-exponent) as u32)),
        Ordering::Equal => value,
        Ordering::Greater => value * TEN.pow(exponent as u32),
    })
}

impl FromStr for TwoFloat {
    type Err = TwoFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = parse_rational(s)?;
        let hi = value.to_f64().ok_or(TwoFloatError::ParseError)?;
        let lo = (value - BigRational::from_f64(hi).ok_or(TwoFloatError::ParseError)?)
            .to_f64()
            .ok_or(TwoFloatError::ParseError)?;

        Ok(Self { hi, lo })
    }
}

#[cfg(test)]
mod tests {
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
