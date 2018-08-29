use crate::error::Error;
use crate::error::Error::{InvalidRange, InvalidRangeSection};
use serde::de::{self, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Deref, RangeInclusive};
use std::str::FromStr;

/// An inclusive range of values that can be rolled
/// on a `Table`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Range(#[doc(hidden)] RangeInclusive<u32>);

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

impl PartialEq<u32> for Range {
    fn eq(&self, other: &u32) -> bool {
        self.contains(other)
    }
}

impl PartialOrd<u32> for Range {
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        Some(if *self.end() < *other {
            Ordering::Less
        } else if *self.start() > *other {
            Ordering::Greater
        } else {
            Ordering::Equal
        })
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Range) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Range {
    // TODO double check this is correct
    fn cmp(&self, other: &Range) -> Ordering {
        if self == other {
            Ordering::Equal
        } else if *self.start() < *other.start() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
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

impl FromStr for Range {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('-') {
            let v: Vec<&str> = s.split('-').collect();
            if v.len() == 2 {
                let start = v[0]
                    .parse::<u32>()
                    .map_err(|_| InvalidRangeSection(String::from(v[0]), 0))?;
                let end = v[1]
                    .parse::<u32>()
                    .map_err(|_| InvalidRangeSection(String::from(v[1]), 1))?;

                Ok(Range(start..=end))
            } else {
                Err(InvalidRange(String::from(s)))
            }
        } else {
            Err(InvalidRange(String::from(s)))
        }
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
                // Simply call FromStr
                if s.contains('-') {
                    let v: Vec<&str> = s.split('-').collect();
                    if v.len() == 2 {
                        let start = v[0].parse::<u32>().map_err(|_| {
                            de::Error::invalid_value(Unexpected::Str(v[0]), &"an integer")
                        })?;
                        let end = v[1].parse::<u32>().map_err(|_| {
                            de::Error::invalid_value(Unexpected::Str(v[1]), &"an integer")
                        })?;

                        Ok(Range(start..=end))
                    } else {
                        Err(de::Error::invalid_value(
                            Unexpected::Str(s),
                            &"two integers separated by a '-'",
                        ))
                    }
                } else {
                    Err(de::Error::invalid_value(Unexpected::Str(s), &self))
                }
            }
        }
        deserializer.deserialize_str(RangeVisitor)
    }
}
