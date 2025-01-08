use crate::TwoFloat;
use core::iter::Sum;
use core::ops::Add;
use num_traits::Zero;

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

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_sum_1() {
        //let v: Vec<TwoFloat> = (1..=10).map(|x| Into::<TwoFloat>::into(x as f64)).collect();
        let v: Vec<f64> = (1..=100).map(|x| x.into()).collect();
        let res: TwoFloat = 5050.0.into();
        let v_sum: TwoFloat = v.iter().sum();

        assert_eq!(v_sum, res);
    }

    #[test]
    fn iter_sum_2() {
        //let v: Vec<TwoFloat> = (1..=10).map(|x| Into::<TwoFloat>::into(x as f64)).collect();
        let v: Vec<f64> = (1..=108).map(|x| 2f64.powi(-x)).collect();
        let one: TwoFloat = 1.0.into();
        let v_sum: TwoFloat = v.iter().sum();
        assert!(v_sum - one < 1e-32);
    }
}
