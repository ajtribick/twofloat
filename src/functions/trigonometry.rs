use std::convert::TryFrom;

use crate::base::*;
use crate::consts::*;

const DEG_PER_RAD: TwoFloat = TwoFloat {
    hi: 57.29577951308232,
    lo: -1.9878495670576283e-15,
};

const RAD_PER_DEG: TwoFloat = TwoFloat {
    hi: 0.017453292519943295,
    lo: 2.9486522708701687e-19,
};

// Polynomial coefficients of sin(x)-x on [0,pi/4]
const S1: TwoFloat = TwoFloat { hi: -0.16666666666666666, lo: -8.51510705987379e-18 };
const S2: TwoFloat = TwoFloat { hi: 0.008333333333333312, lo: -1.3912016738387687e-19 };
const S3: TwoFloat = TwoFloat { hi: -0.00019841269841246198, lo: -7.681619205684898e-21 };
const S4: TwoFloat = TwoFloat { hi: 2.75573192105007e-06, lo: 2.955758642485038e-23 };
const S5: TwoFloat = TwoFloat { hi: -2.505210410444575e-08, lo: 9.269533560586216e-26 };
const S6: TwoFloat = TwoFloat { hi: 1.605827759011912e-10, lo: 3.404451553732099e-27 };
const S7: TwoFloat = TwoFloat { hi: -7.574792323977277e-13, lo: 4.727692438769333e-29 };

// Polynomial coefficients of cos(x)-1+x^2/2 on [0,pi/4]
const C1: TwoFloat = TwoFloat { hi: 0.041666666666666664, lo: 2.2440014013613353e-18 };
const C2: TwoFloat = TwoFloat { hi: -0.0013888888888888872, lo: 6.065718226973696e-20 };
const C3: TwoFloat = TwoFloat { hi: 2.4801587301569693e-05, lo: 7.112363744916272e-22 };
const C4: TwoFloat = TwoFloat { hi: -2.7557319214749576e-07, lo: -2.1630593346449427e-23 };
const C5: TwoFloat = TwoFloat { hi: 2.0876754247413408e-09, lo: -3.4443669660877135e-26 };
const C6: TwoFloat = TwoFloat { hi: -1.1470281608989357e-11, lo: 4.623853450729046e-28 };
const C7: TwoFloat = TwoFloat { hi: 4.737645013072795e-14, lo: 2.0519566094121702e-30 };

fn quadrant(value: &TwoFloat) -> (TwoFloat, i8) {
    if value.abs() < FRAC_PI_4 {
        (*value, 0)
    } else {
        let quotient = (value / FRAC_PI_2).round();
        let remainder = value - &quotient * FRAC_PI_2;
        match i8::try_from(quotient % 4.0) {
            Ok(quadrant) => (remainder, quadrant.abs()),
            _ => (TwoFloat { hi: std::f64::NAN, lo: std::f64::NAN }, 0)
        }
    }
}

fn restricted_sin(x: &TwoFloat) -> TwoFloat {
    let x2 = x * x;
    x * (1.0 + x2 * (S1 + x2 * (S2 + x2 * (S3 + x2 * (S4 + x2 * (S5 + x2 * (S6 + x2 * S7)))))))
}

fn restricted_cos(x: &TwoFloat) -> TwoFloat {
    let x2 = x * x;
    1.0 + x2 * (-0.5 + x2 * (C1 + x2 * (C2 + x2 * (C3 + x2 * (C4 + x2 * (C5 + x2 * (C6 + x2 * C7)))))))
}

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

    /// Computes the sine of the value (in radians).
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(2.5);
    /// let b = a.sin();
    /// let c = 2.5f64.sin();
    ///
    /// assert!((b - c).abs() < 1e-10);
    pub fn sin(&self) -> TwoFloat {
        if !self.is_valid() { return *self; }
        let (x, quadrant) = quadrant(self);
        match quadrant {
            0 => restricted_sin(&x),
            1 => restricted_cos(&x),
            2 => -restricted_sin(&x),
            _ => -restricted_cos(&x)
        }
    }

    /// Computes the cosine of the value (in radians)
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(2.5);
    /// let b = a.cos();
    /// let c = 2.5f64.cos();
    ///
    /// assert!((b - c).abs() < 1e-10);
    pub fn cos(&self) -> TwoFloat {
        if !self.is_valid() { return *self; }
        let (x, quadrant) = quadrant(self);
        match quadrant {
            0 => restricted_cos(&x),
            1 => -restricted_sin(&x),
            2 => -restricted_cos(&x),
            _ => restricted_sin(&x)
        }
    }

    /// Simultaneously computes the sine and cosine of the value. Returns a
    /// tuple with the sine as the first element and the cosine as the second
    /// element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(2.5);
    /// let (s, c) = a.sin_cos();
    ///
    /// assert!((s - 2.5f64.sin()).abs() < 1e-10);
    /// assert!((c - 2.5f64.cos()).abs() < 1e-10);
    pub fn sin_cos(&self) -> (TwoFloat, TwoFloat) {
        if !self.is_valid() { return (*self, *self); }
        let (x, quadrant) = quadrant(self);
        let s = restricted_sin(&x);
        let c = restricted_cos(&x);
        match quadrant {
            0 => (s, c),
            1 => (c, -s),
            2 => (-s, -c),
            _ => (-c, s)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quadrant_test() {
        assert_eq!(0, quadrant(&TwoFloat::from(0.5)).1);
        assert_eq!(0, quadrant(&TwoFloat::from(-0.5)).1);
        assert_eq!(1, quadrant(&TwoFloat::from(2.0)).1);
        assert_eq!(1, quadrant(&TwoFloat::from(-2.0)).1);
        assert_eq!(2, quadrant(&TwoFloat::from(3.14)).1);
        assert_eq!(2, quadrant(&TwoFloat::from(-3.14)).1);
        assert_eq!(3, quadrant(&TwoFloat::from(4.0)).1);
        assert_eq!(3, quadrant(&TwoFloat::from(-4.0)).1);
        assert_eq!(0, quadrant(&TwoFloat::from(6.0)).1);
        assert_eq!(0, quadrant(&TwoFloat::from(-6.0)).1);
    }

    #[test]
    fn sin_test() {
        assert_eq!(0.0, TwoFloat::from(0.0).sin());
        assert!((0.5f64.sin() - TwoFloat::from(0.5).sin()).abs() < 1e-10);
        assert!((1.4f64.sin() - TwoFloat::from(1.4).sin()).abs() < 1e-10);
        assert!((3.0f64.sin() - TwoFloat::from(3.0).sin()).abs() < 1e-10);
        assert!((4.0f64.sin() - TwoFloat::from(4.0).sin()).abs() < 1e-10);
        assert!((6.0f64.sin() - TwoFloat::from(6.0).sin()).abs() < 1e-10);
    }

    #[test]
    fn cos_test() {
        assert_eq!(1.0, TwoFloat::from(0.0).cos());
        assert!((0.5f64.cos() - TwoFloat::from(0.5).cos()).abs() < 1e-10);
        assert!((1.4f64.cos() - TwoFloat::from(1.4).cos()).abs() < 1e-10);
        assert!((3.0f64.cos() - TwoFloat::from(3.0).cos()).abs() < 1e-10);
        assert!((4.0f64.cos() - TwoFloat::from(4.0).cos()).abs() < 1e-10);
        assert!((6.0f64.cos() - TwoFloat::from(6.0).cos()).abs() < 1e-10);
    }
}
