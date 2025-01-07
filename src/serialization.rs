use core::convert::TryFrom;

use serde::{
    de::{self, Unexpected, Visitor},
    ser::SerializeStruct,
    Deserialize, Serialize,
};

use crate::TwoFloat;

impl Serialize for TwoFloat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("TwoFloat", 2)?;
        state.serialize_field("hi", &self.hi)?;
        state.serialize_field("lo", &self.lo)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for TwoFloat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["secs", "nanos"];
        enum Field {
            Hi,
            Lo,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct FieldVisitor;

                impl Visitor<'_> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                        formatter.write_str("`hi` or `lo`")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        match v {
                            "hi" => Ok(Field::Hi),
                            "lo" => Ok(Field::Lo),
                            _ => Err(de::Error::unknown_field(v, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct TwoFloatVisitor;

        impl<'de> Visitor<'de> for TwoFloatVisitor {
            type Value = TwoFloat;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("struct TwoFloat")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let hi = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let lo = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                TwoFloat::try_from((hi, lo)).map_err(|_| {
                    de::Error::invalid_value(Unexpected::Float(lo), &"non-overlapping low word")
                })
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut hi = None;
                let mut lo = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Hi => {
                            if hi.is_some() {
                                return Err(de::Error::duplicate_field("hi"));
                            }

                            hi = Some(map.next_value()?);
                        }
                        Field::Lo => {
                            if lo.is_some() {
                                return Err(de::Error::duplicate_field("lo"));
                            }

                            lo = Some(map.next_value()?);
                        }
                    }
                }

                let hi = hi.ok_or_else(|| de::Error::missing_field("hi"))?;
                let lo = lo.ok_or_else(|| de::Error::missing_field("lo"))?;
                TwoFloat::try_from((hi, lo)).map_err(|_| {
                    de::Error::invalid_value(Unexpected::Float(lo), &"non-overlapping low word")
                })
            }
        }

        deserializer.deserialize_struct("TwoFloat", FIELDS, TwoFloatVisitor)
    }
}
