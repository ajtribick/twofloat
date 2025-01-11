use crate::TwoFloat;

impl TwoFloat {
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
    /// ```
    pub fn sqrt(self) -> Self {
        if self.hi < 0.0 || (self.hi == 0.0 && self.lo < 0.0) {
            Self::NAN
        } else if self.hi == 0.0 && self.lo == 0.0 {
            Self { hi: 0.0, lo: 0.0 }
        } else {
            let x = libm::sqrt(self.hi).recip();
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
    /// ```
    pub fn cbrt(self) -> Self {
        let mut x = Self::from(libm::cbrt(self.hi));
        let mut x2 = x * x;
        x -= (x2 * x - self) / (3.0 * x2);
        x2 = x * x;
        x - (x2 * x - self) / (3.0 * x2)
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
    /// ```
    pub fn hypot(self, other: Self) -> Self {
        (self * self + other * other).sqrt()
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
    /// ```
    pub fn powf(self, y: Self) -> Self {
        //let mut x = Self::from(libm::log(self.hi));
        //let (n_integer, n_fractional) = libm::modf(y.hi);
        //let x_integer = self.powi(n_integer as i32);

        //let mut y = y;
        //y.hi = n_fractional;

        match (self == 0.0, y == 0.0) {
            (true, true) => Self::NAN,
            (true, false) => Self::from(0.0),
            (false, true) => Self::from(1.0),
            (false, false) => {
                if self.is_sign_positive() {
                    (y * self.ln()).exp()
                } else if libm::modf(y.hi).0 != 0.0 || libm::modf(y.lo).0 != 0.0 {
                    Self::NAN
                } else {
                    let abs_result = (y * self.abs().ln()).exp();
                    //let abs_result = self.powi(y.hi as i32) * self.powi(y.lo as i32);
                    let low_trunc = if libm::trunc(y.lo) == 0.0 {
                        libm::trunc(y.hi)
                    } else {
                        libm::trunc(y.lo)
                    };

                    if low_trunc % 2.0 == 0.0 {
                        abs_result
                    } else {
                        -abs_result
                    }
                }
            }
        }
        //x * x_integer
    }
}
