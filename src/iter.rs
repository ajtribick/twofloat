use crate::TwoFloat;
use core::iter::{Product, Sum};
use core::ops::{Add, Mul};
use num_traits::{One, Zero};

impl<T> Sum<T> for TwoFloat
where
    Self: Add<T, Output = Self>,
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        iter.fold(Self::zero(), <Self>::add)
    }
}

impl<T> Product<T> for TwoFloat
where
    Self: Mul<T, Output = Self>,
{
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        iter.fold(Self::one(), <Self>::mul)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "std")]
    #[test]
    fn iter_sum_vec_1() {
        let v: Vec<f64> = (1..=100).map(|x| x.into()).collect();
        let v_sum: TwoFloat = v.iter().sum();
        let res: TwoFloat = 5050.0.into();

        assert_eq!(v_sum, res);
    }

    #[cfg(feature = "std")]
    #[test]
    fn iter_sum_vec_2() {
        let v: Vec<f64> = (1..=108).map(|x| 2f64.powi(-x)).collect();
        let v_sum: TwoFloat = v.iter().sum();
        let one: TwoFloat = 1.0.into();
        assert!(v_sum - one < 1e-32);
    }

    #[cfg(feature = "std")]
    #[test]
    fn iter_product_vec_1() {
        let v: Vec<f64> = (1..=10).map(|x| x.into()).collect();
        let v_product: TwoFloat = v.iter().product();
        let res: TwoFloat = 3628800.0.into();

        assert_eq!(v_product, res);
    }

    #[cfg(feature = "std")]
    #[test]
    fn iter_product_vec_2() {
        let v: Vec<f64> = (1..=18).map(|x| 2f64.powi(-x)).collect();
        let v_product: TwoFloat = v.iter().product();
        let one: TwoFloat = 1.0.into();
        assert!(v_product - one < 1e-32);
    }

    #[test]
    fn iter_sum_1() {
        let sum: TwoFloat = (1..=100).map(|x| x as f64).sum();
        let res: TwoFloat = 5050.0.into();

        assert_eq!(sum, res);
    }

    #[test]
    fn iter_sum_2() {
        let sum: TwoFloat = (1..=108).map(|x| 2f64.powi(-x)).sum();
        let one: TwoFloat = 1.0.into();
        assert!(sum - one < 1e-32);
    }

    #[test]
    fn iter_product_1() {
        let product: TwoFloat = (1..=10).map(|x| x as f64).product();
        let res: TwoFloat = 3628800.0.into();

        assert_eq!(product, res);
    }

    #[test]
    fn iter_product_2() {
        let product: TwoFloat = (1..=18).map(|x| 2f64.powi(-x)).product();
        let one: TwoFloat = 1.0.into();
        assert!(product - one < 1e-32);
    }
}
