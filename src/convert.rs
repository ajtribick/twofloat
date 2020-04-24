use core::cmp::Eq;
use core::convert::{From, TryFrom};
use core::fmt;
use std::error;

use crate::base::{no_overlap, TwoFloat};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ConversionError;

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid TwoFloat conversion")
    }
}

impl error::Error for ConversionError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

macro_rules! from_conversion {
    (|$source_i:ident : TwoFloat| -> $dest:tt $code:block) => {
        impl From<TwoFloat> for $dest {
            fn from($source_i: TwoFloat) -> Self $code
        }

        impl<'a> From<&'a TwoFloat> for $dest {
            fn from($source_i: &'a TwoFloat) -> Self $code
        }
    };
    (|$source_i:ident: TwoFloat| -> Result<$dest:tt, $err:tt> $code:block) => {
        impl TryFrom<TwoFloat> for $dest {
            type Error = $err;

            fn try_from($source_i: TwoFloat) -> Result<Self, Self::Error> $code
        }

        impl<'a> TryFrom<&'a TwoFloat> for $dest {
            type Error = $err;

            fn try_from($source_i: &'a TwoFloat) -> Result<Self, Self::Error> $code
        }
    };
}

from_conversion!(|value: TwoFloat| -> (f64, f64) { (value.hi, value.lo) });

impl TryFrom<(f64, f64)> for TwoFloat {
    type Error = ConversionError;

    fn try_from(value: (f64, f64)) -> Result<Self, Self::Error> {
        if no_overlap(value.0, value.1) {
            Ok(Self {
                hi: value.0,
                lo: value.1,
            })
        } else {
            Err(Self::Error {})
        }
    }
}

from_conversion!(|value: TwoFloat| -> [f64; 2] { [value.hi, value.lo] });

impl TryFrom<[f64; 2]> for TwoFloat {
    type Error = ConversionError;

    fn try_from(value: [f64; 2]) -> Result<Self, Self::Error> {
        if no_overlap(value[0], value[1]) {
            Ok(Self {
                hi: value[0],
                lo: value[1],
            })
        } else {
            Err(Self::Error {})
        }
    }
}

macro_rules! float_convert {
    ($type:tt) => {
        impl From<$type> for TwoFloat {
            fn from(value: $type) -> Self {
                TwoFloat {
                    hi: value as f64,
                    lo: 0.0,
                }
            }
        }

        from_conversion!(|value: TwoFloat| -> $type { value.hi as $type });
    };
}

float_convert!(f64);
float_convert!(f32);

macro_rules! int_convert {
    ($type:tt) => {
        impl From<$type> for TwoFloat {
            fn from(value: $type) -> Self {
                TwoFloat {
                    hi: value as f64,
                    lo: 0.0,
                }
            }
        }

        from_conversion!(|value: TwoFloat| -> Result<$type, ConversionError> {
            const LOWER_BOUND: f64 = $type::MIN as f64 - 1.0;
            const UPPER_BOUND: f64 = $type::MAX as f64 + 1.0;
            if value.hi < LOWER_BOUND || value.hi > UPPER_BOUND {
                Err(ConversionError {})
            } else if value.hi == LOWER_BOUND {
                if value.lo > 0.0 {
                    Ok($type::MIN)
                } else {
                    Err(ConversionError {})
                }
            } else if value.hi == UPPER_BOUND {
                if value.lo < 0.0 {
                    Ok($type::MAX)
                } else {
                    Err(ConversionError {})
                }
            } else if value.hi.fract() == 0.0 {
                if value.hi < 0.0 && value.lo > 0.0 {
                    Ok(value.hi as $type + 1)
                } else if value.hi >= 0.0 && value.lo < 0.0 {
                    Ok(value.hi as $type - 1)
                } else {
                    Ok(value.hi as $type)
                }
            } else {
                Ok(value.hi as $type)
            }
        });
    };
}

int_convert!(i32);
int_convert!(i16);
int_convert!(i8);
int_convert!(u32);
int_convert!(u16);
int_convert!(u8);

impl From<i64> for TwoFloat {
    fn from(value: i64) -> Self {
        let a = value as f64;
        let b = if a == i64::MAX as f64 {
            ((value - i64::MAX) - 1) as f64
        } else {
            (value - a as i64) as f64
        };

        TwoFloat { hi: a, lo: b }
    }
}

from_conversion!(|value: TwoFloat| -> Result<i64, ConversionError> {
    const LOWER_BOUND: f64 = i64::MIN as f64;
    const UPPER_BOUND: f64 = i64::MAX as f64;

    if value.hi < LOWER_BOUND || value.hi > UPPER_BOUND {
        Err(ConversionError {})
    } else if value.hi == LOWER_BOUND {
        if value.lo >= 0.0 {
            Ok(i64::MIN + value.lo as i64)
        } else {
            Err(ConversionError {})
        }
    } else if value.hi == UPPER_BOUND {
        if value.lo < 0.0 {
            Ok(i64::MAX + value.lo.floor() as i64 + 1)
        } else {
            Err(ConversionError {})
        }
    } else if value.hi.fract() == 0.0 {
        if value.lo.trunc() == 0.0 {
            if value.hi < 0.0 && value.lo > 0.0 {
                Ok(value.hi as i64 + 1)
            } else if value.hi >= 0.0 && value.lo < 0.0 {
                Ok(value.hi as i64 - 1)
            } else {
                Ok(value.hi as i64)
            }
        } else {
            Ok(value.hi as i64 + value.lo as i64)
        }
    } else {
        Ok(value.hi as i64)
    }
});

impl From<u64> for TwoFloat {
    fn from(value: u64) -> Self {
        let a = value as f64;
        let b = if a == u64::MAX as f64 {
            -(((u64::MAX - value) + 1) as f64)
        } else if value >= a as u64 {
            (value - a as u64) as f64
        } else {
            -((a as u64 - value) as f64)
        };

        TwoFloat { hi: a, lo: b }
    }
}

from_conversion!(|value: TwoFloat| -> Result<u64, ConversionError> {
    const LOWER_BOUND: f64 = -1.0;
    const UPPER_BOUND: f64 = u64::MAX as f64;

    if value.hi < LOWER_BOUND || value.hi > UPPER_BOUND {
        Err(ConversionError {})
    } else if value.hi == LOWER_BOUND {
        if value.lo >= 0.0 {
            Ok(0)
        } else {
            Err(ConversionError {})
        }
    } else if value.hi == UPPER_BOUND {
        if value.lo < 0.0 {
            Ok(u64::MAX - (-value.lo.floor() as u64) + 1)
        } else {
            Err(ConversionError {})
        }
    } else if value.hi.fract() == 0.0 {
        if value.lo.trunc() == 0.0 {
            if value.hi < 0.0 && value.lo > 0.0 {
                Ok(value.hi as u64 + 1)
            } else if value.hi >= 0.0 && value.lo < 0.0 {
                Ok(value.hi as u64 - 1)
            } else {
                Ok(value.hi as u64)
            }
        } else if value.lo >= 0.0 {
            Ok(value.hi as u64 + value.lo as u64)
        } else {
            Ok(value.hi as u64 - (-value.lo) as u64)
        }
    } else {
        Ok(value.hi as u64)
    }
});
