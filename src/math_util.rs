/// A wrapper struct for mathematical operations on `f64`s.
///
/// It uses "libm" if it's enabled, which is required for "no_std".
/// Fallbacks to "std" otherwise.
pub(crate) struct Math;

#[cfg(feature = "libm")]
impl Math {
    #[inline(always)]
    pub fn abs(x: f64) -> f64 {
        libm::fabs(x)
    }
    #[inline(always)]
    pub fn ceil(x: f64) -> f64 {
        libm::ceil(x)
    }
    #[inline(always)]
    pub fn exp2(x: f64) -> f64 {
        libm::exp2(x)
    }
    #[inline(always)]
    pub fn floor(x: f64) -> f64 {
        libm::floor(x)
    }
    #[inline(always)]
    pub fn fma(a: f64, b: f64, c: f64) -> f64 {
        libm::fma(a, b, c)
    }
    #[inline(always)]
    pub fn fract(x: f64) -> f64 {
        libm::modf(x).0
    }
    #[inline(always)]
    pub fn round(x: f64) -> f64 {
        libm::round(x)
    }
    #[inline(always)]
    pub fn signum(x: f64) -> f64 {
        libm::copysign(1., x)
    }
    #[inline(always)]
    pub fn trunc(x: f64) -> f64 {
        libm::trunc(x)
    }
}

#[cfg(not(feature = "libm"))]
impl Math {
    #[inline(always)]
    pub fn abs(x: f64) -> f64 {
        x.abs()
    }
    #[inline(always)]
    pub fn ceil(x: f64) -> f64 {
        x.ceil()
    }
    #[inline(always)]
    pub fn exp2(x: f64) -> f64 {
        x.exp2()
    }
    #[inline(always)]
    pub fn floor(x: f64) -> f64 {
        x.floor()
    }
    #[inline(always)]
    pub fn fma(a: f64, b: f64, c: f64) -> f64 {
        a.mul_add(b, c)
    }
    #[inline(always)]
    pub fn fract(x: f64) -> f64 {
        x.fract()
    }
    #[inline(always)]
    pub fn round(x: f64) -> f64 {
        x.round()
    }
    #[inline(always)]
    pub fn signum(x: f64) -> f64 {
        x.signum()
    }
    #[inline(always)]
    pub fn trunc(x: f64) -> f64 {
        x.trunc()
    }
}
