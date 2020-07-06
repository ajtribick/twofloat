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
                Self {
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
                Self {
                    hi: value as f64,
                    lo: 0.0,
                }
            }
        }

        from_conversion!(|value: TwoFloat| -> Result<$type, ConversionError> {
            const LOWER_BOUND: f64 = $type::MIN as f64;
            const UPPER_BOUND: f64 = $type::MAX as f64;
            let truncated = value.trunc();
            if truncated < LOWER_BOUND || truncated > UPPER_BOUND {
                Err(ConversionError {})
            } else {
                Ok(truncated.hi() as $type)
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

macro_rules! bigint_convert {
    ($type:tt) => {
        impl From<$type> for TwoFloat {
            fn from(value: $type) -> Self {
                let a = value as f64;
                let b = if a == $type::MAX as f64 {
                    -((($type::MAX - value) + 1) as f64)
                } else if value >= a as $type {
                    (value - a as $type) as f64
                } else {
                    -((a as $type - value) as f64)
                };

                Self { hi: a, lo: b }
            }
        }

        from_conversion!(|value: TwoFloat| -> Result<$type, ConversionError> {
            const LOWER_BOUND: TwoFloat = TwoFloat {
                hi: $type::MIN as f64,
                lo: 0.0,
            };

            const UPPER_BOUND: TwoFloat = TwoFloat {
                hi: $type::MAX as f64,
                lo: -1.0,
            };

            let truncated = value.trunc();
            if truncated < LOWER_BOUND || truncated > UPPER_BOUND {
                Err(ConversionError {})
            } else if truncated.hi() == UPPER_BOUND.hi() {
                Ok($type::MAX - (-truncated.lo() as $type) + 1)
            } else if truncated.lo() >= 0.0 {
                Ok(truncated.hi() as $type + truncated.lo() as $type)
            } else {
                Ok(truncated.hi() as $type - (-truncated.lo()) as $type)
            }
        });
    };
}

bigint_convert!(i64);
bigint_convert!(u64);
