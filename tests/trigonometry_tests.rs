pub mod common;

#[cfg(feature = "math_funcs")]
mod tests {
    use super::common::*;
    use core::convert::TryFrom;
    use rand::Rng;
    use twofloat::TwoFloat;

    #[test]
    fn sin_cos_test() {
        let mut rng = rand::thread_rng();
        let dist = rand::distributions::Uniform::new_inclusive(-20.0, 20.0);

        repeated_test(|| {
            let source = loop {
                let a = rng.sample(dist);
                let b = random_float();
                if let Ok(result) = TwoFloat::try_from((a, b)) {
                    break result;
                }
            };

            let (sin, cos) = source.sin_cos();
            assert!(
                sin.is_valid(),
                "sin_cos({:?}).0 returned invalid result",
                source
            );
            assert_eq!(
                sin,
                source.sin(),
                "sin_cos({0:?}).0 not equal to sin({0:?})",
                source
            );

            assert!(
                cos.is_valid(),
                "sin_cos({:?}).1 returned invalid result",
                source
            );
            assert_eq!(
                cos,
                source.cos(),
                "sin_cos({0:?}).1 not equal to cos({0:?})",
                source
            );

            let result = cos * cos + sin * sin;
            let difference = (1.0 - result).abs();
            assert!(
                difference < 1e-10,
                "cos^2 + sin^2 for {:?} returned value different from 1",
                source
            );
        });
    }

    #[test]
    fn sin_asin_test() {
        let mut rng = rand::thread_rng();
        let dist = rand::distributions::Uniform::new_inclusive(
            -core::f64::consts::FRAC_PI_2,
            core::f64::consts::FRAC_PI_2,
        );
        repeated_test(|| {
            let source = TwoFloat::from(rng.sample(dist));
            let result = source.sin().asin();
            assert!(
                result.is_valid(),
                "Angle {:?} does not produce valid value for sin/asin round trip",
                source
            );
            assert!(
                (source - result).abs() < 1e-10,
                "Angle {:?} does not return same value after sin/asin round trip ({:?})",
                source,
                result
            );
        });
    }

    #[test]
    fn cos_acos_test() {
        let mut rng = rand::thread_rng();
        let dist = rand::distributions::Uniform::new_inclusive(0.0, core::f64::consts::PI);
        repeated_test(|| {
            let source = TwoFloat::from(rng.sample(dist));
            let result = source.cos().acos();
            assert!(
                result.is_valid(),
                "Angle {:?} does not produce valid value for cos/acos round trip",
                source
            );
            assert!(
                (source - result).abs() < 1e-10,
                "Angle {:?} does not return same value after cos/acos round trip ({:?})",
                source,
                result
            );
        });
    }

    #[test]
    fn tan_atan_test() {
        let mut rng = rand::thread_rng();
        let dist = rand::distributions::Uniform::new_inclusive(
            -core::f64::consts::FRAC_PI_2,
            core::f64::consts::FRAC_PI_2,
        );
        repeated_test(|| {
            let source = TwoFloat::from(rng.sample(dist));
            let result = source.tan().atan();
            assert!(
                result.is_valid(),
                "Angle {:?} does not produce valid value for tan/atan round trip",
                source
            );
            assert!(
                (source - result).abs() < 1e-10,
                "Angle {:?} does not return same value after tan/atan round trip ({:?})",
                source,
                result
            );
        });
    }

    #[test]
    fn sin_cos_atan2_test() {
        let mut rng = rand::thread_rng();
        let dist = rand::distributions::Uniform::new_inclusive(
            -core::f64::consts::PI,
            core::f64::consts::PI,
        );
        repeated_test(|| {
            let source = TwoFloat::from(rng.sample(dist));
            let (s, c) = source.sin_cos();
            let result = TwoFloat::atan2(s, c);
            assert!(
                result.is_valid(),
                "Angle {:?} does not produce valid value for sin_cos/atan2 round trip",
                source
            );
            assert!(
                (source - result).abs() < 1e-10,
                "Angle {:?} does not return same value after sin_cos/atan2 round trip ({:?})",
                source,
                result
            );
        });
    }
}
