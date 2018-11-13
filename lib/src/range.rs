use core::{
    cmp::Ordering,
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Deref, RangeInclusive},
    str::FromStr,
};
use crate::error::Error;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

/// An inclusive range of values that can be rolled on a `Table`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Range(#[doc(hidden)] RangeInclusive<u32>);

impl Range {
    pub fn iter(&self) -> RangeInclusive<u32> {
        *self.start()..=*self.end()
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
                    .map_err(|_| Error::invalid_range_section(v[0], stringify!(start)))?;
                let end = v[1]
                    .parse::<u32>()
                    .map_err(|_| Error::invalid_range_section(v[1], stringify!(end)))?;

                Ok(Range(start..=end))
            } else {
                Err(Error::invalid_range(s))
            }
        } else {
            Err(Error::invalid_range(s))
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
                Range::from_str(s).map_err(de::Error::custom)
            }
        }
        deserializer.deserialize_str(RangeVisitor)
    }
}
