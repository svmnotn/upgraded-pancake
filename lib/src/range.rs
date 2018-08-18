use serde::de::{self, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Deref, RangeInclusive};

#[derive(Debug, Clone)]
pub struct Range(RangeInclusive<u32>);

impl Range {
    pub fn contains(&self, v: u32) -> bool {
        v >= *self.start() && *self.end() >= v
    }
}

impl From<RangeInclusive<u32>> for Range {
    fn from(r: RangeInclusive<u32>) -> Self {
        Range(r)
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}-{}", self.0.start(), self.0.end())
    }
}

impl Deref for Range {
    type Target = RangeInclusive<u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for Range {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl<'de> Deserialize<'de> for Range {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RangeVisitor;

        impl<'de> Visitor<'de> for RangeVisitor {
            type Value = Range;

            fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
                formatter.write_str("struct Range")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if s.contains('-') {
                    let v: Vec<u32> = s
                        .split('-')
                        .map(|x| x.parse::<u32>().expect("not a number!"))
                        .collect(); // TODO make into error
                    Ok(Range(v[0]..=v[1]))
                } else {
                    Err(de::Error::invalid_value(Unexpected::Str(s), &self))
                }
            }
        }
        deserializer.deserialize_str(RangeVisitor)
    }
}
