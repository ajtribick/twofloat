use crate::base::*;

const DEG_PER_RAD: TwoFloat = TwoFloat {
    hi: 57.29577951308232,
    lo: -1.9878495670576283e-15,
};

const RAD_PER_DEG: TwoFloat = TwoFloat {
    hi: 0.017453292519943295,
    lo: 2.9486522708701687e-19,
};

impl TwoFloat {
    /// Converts degrees to radians.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(90.0);
    /// let b = a.to_radians();
    ///
    /// assert!((b - twofloat::consts::FRAC_PI_2).abs() < 1e-16);
    pub fn to_radians(&self) -> TwoFloat {
        self * &RAD_PER_DEG
    }

    /// Converts radians to degrees.
    ///
    /// # Examples
    ///
    /// ```
    /// let a = twofloat::consts::PI;
    /// let b = a.to_degrees();
    ///
    /// assert!((b - 180.0).abs() < 1e-16);
    pub fn to_degrees(&self) -> TwoFloat {
        self * &DEG_PER_RAD
    }
}
