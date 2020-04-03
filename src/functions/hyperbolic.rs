use crate::base::*;

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
    pub fn cosh(&self) -> TwoFloat {
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
    pub fn sinh(&self) -> TwoFloat {
        self.exp() / 2.0 - (-self).exp() / 2.0
    }

    /// Hyperbolic tangent function.
    ///
    /// This is a convenience method that computes the value by calling the
    /// `sinh` and `cosh` functions.
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
    pub fn tanh(&self) -> TwoFloat {
        self.sinh() / self.cosh()
    }
}
