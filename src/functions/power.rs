use crate::base::TwoFloat;

impl TwoFloat {
    /// Takes the reciprocal (inverse) of the number, `1/x`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::new_add(67.2, 5.7e-53);
    /// let b = a.recip();
    /// let difference = b.recip() - a;
    ///
    /// assert!(difference.abs() < 1e-16);
    pub fn recip(self) -> Self {
        1.0 / self
    }

    /// Returns the square root of the number, using equation 4 from Karp &
    /// Markstein (1997).
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(2.0);
    /// let b = a.sqrt();
    ///
    /// assert!(b * b - a < 1e-16);
    pub fn sqrt(self) -> Self {
        if self.hi < 0.0 || (self.hi == 0.0 && self.lo < 0.0) {
            Self::NAN
        } else if self.hi == 0.0 && self.lo == 0.0 {
            Self { hi: 0.0, lo: 0.0 }
        } else {
            let x = self.hi.sqrt().recip();
            let y = self.hi * x;
            Self::new_add(y, (self - Self::new_mul(y, y)).hi * (x * 0.5))
        }
    }

    /// Returns the cube root of the number, using Newton-Raphson iteration.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::new_add(1.4e53, 0.21515);
    /// let b = a.cbrt();
    ///
    /// assert!(b.powi(3) - a < 1e-16);
    pub fn cbrt(self) -> Self {
        let mut x = Self::from(self.hi.cbrt());
        let mut x2 = &x * &x;
        x -= (&x2 * &x - self) / (3.0 * &x2);
        x2 = &x * &x;
        x - (&x2 * &x - self) / (3.0 * &x2)
    }

    /// Calculates the length of the hypotenuse of a right-angle triangle
    /// given legs of length `self` and `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(3.0);
    /// let b = TwoFloat::from(4.0);
    /// let c = TwoFloat::hypot(a, b);
    ///
    /// assert!((c - 5.0).abs() < 1e-10);
    pub fn hypot(self, other: Self) -> Self {
        (self * self + other * other).sqrt()
    }

    /// Raises the number to an integer power. Returns a NAN value for 0^0.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(2.0).powi(3);
    /// let b = TwoFloat::from(0.0).powi(0);
    ///
    /// assert!(a - TwoFloat::from(8.0) <= 1e-16);
    /// assert!(!b.is_valid());
    pub fn powi(self, n: i32) -> Self {
        match n {
            0 => {
                if self.hi == 0.0 && self.lo == 0.0 {
                    Self::NAN
                } else {
                    Self::from(1.0)
                }
            }
            1 => self.clone(),
            -1 => self.recip(),
            _ => {
                let mut result = Self::from(1.0);
                let mut n_pos = n.abs();
                let mut value = self.clone();
                while n_pos > 0 {
                    if (n_pos & 1) != 0 {
                        result *= &value;
                    }
                    value *= value;
                    n_pos >>= 1;
                }
                if n > 0 {
                    result
                } else {
                    result.recip()
                }
            }
        }
    }

    /// Returns the value raised to the power `y`.
    ///
    /// This method is quite inaccurate, where possible `powi`, `sqrt` or
    /// `cbrt` should be preferred.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(-5.0);
    /// let b = TwoFloat::from(3.0);
    /// let c = a.powf(b);
    ///
    /// assert!((c + 125.0).abs() < 1e-9, "{}", c);
    pub fn powf(self, y: Self) -> Self {
        match (self == 0.0, y == 0.0) {
            (true, true) => Self::NAN,
            (true, false) => Self::from(0.0),
            (false, true) => Self::from(1.0),
            (false, false) => {
                if self.is_sign_positive() {
                    (y * self.ln()).exp()
                } else if self.hi.fract() != 0.0 || self.lo.fract() != 0.0 {
                    Self::NAN
                } else {
                    let abs_result = (y * self.abs().ln()).exp();
                    let low_trunc = if self.lo.trunc() == 0.0 {
                        self.hi.trunc()
                    } else {
                        self.lo.trunc()
                    };

                    if low_trunc % 2.0 == 0.0 {
                        abs_result
                    } else {
                        -abs_result
                    }
                }
            }
        }
    }
}
