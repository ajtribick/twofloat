use crate::base::TwoFloat;

impl TwoFloat {
    /// Hyperbolic cosine function.
    ///
    /// This is a convenience method that computes the value by calling the
    /// exponential function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(2.0);
    /// let b = a.cosh();
    /// let c = 2.0f64.cosh();
    ///
    /// assert!((b - c).abs() < 1e-10);
    pub fn cosh(self) -> Self {
        self.exp() / 2.0 + (-self).exp() / 2.0
    }

    /// Hyperbolic sine function.
    ///
    /// This is a convenience method that computes the value by calling the
    /// exponential function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(2.0);
    /// let b = a.sinh();
    /// let c = 2.0f64.sinh();
    ///
    /// assert!((b - c).abs() < 1e-10);
    pub fn sinh(self) -> Self {
        self.exp() / 2.0 - (-self).exp() / 2.0
    }

    /// Hyperbolic tangent function.
    ///
    /// This is a convenience method that computes the value by calling the
    /// exponential function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(2.0);
    /// let b = a.tanh();
    /// let c = 2.0f64.tanh();
    ///
    /// assert!((b - c).abs() < 1e-10);
    pub fn tanh(self) -> Self {
        let e_plus = self.exp();
        let e_minus = (-self).exp();
        (e_plus - e_minus) / (e_plus + e_minus)
    }

    /// Inverse hyperbolic cosine function.
    ///
    /// This is a convenience method that computes the value by calling the
    /// `sqrt` and `ln` functions.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(2.0);
    /// let b = a.acosh();
    /// let c = 2.0f64.acosh();
    ///
    /// assert!((b - c).abs() < 1e-10);
    pub fn acosh(self) -> Self {
        (self + (self * self - 1.0).sqrt()).ln()
    }

    /// Inverse hyperbolic sine function.
    ///
    /// This is a convenience method that computes the value by calling the
    /// `sqrt` and `ln` functions.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(2.0);
    /// let b = a.asinh();
    /// let c = 2.0f64.asinh();
    ///
    /// assert!((b - c).abs() < 1e-10);
    pub fn asinh(self) -> Self {
        (self + (self * self + 1.0).sqrt()).ln()
    }

    /// Inverse hyperbolic tangent function.
    ///
    /// This is a convenience method that computes the value by calling the
    /// `ln` function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(0.5);
    /// let b = a.atanh();
    /// let c = 0.5f64.atanh();
    ///
    /// assert!((b - c).abs() < 1e-10);
    pub fn atanh(self) -> Self {
        ((1.0 + self) / (1.0 - self)).ln() / 2.0
    }
}
