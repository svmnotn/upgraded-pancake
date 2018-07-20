use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::de::{self, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct Range(pub RangeInclusive<i64>);

impl Range {
    pub fn contains(&self, v: i64) -> bool {
        (v < *self.0.start() || v > *self.0.end()) == false
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.0.start(), self.0.end())
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

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Range")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if s.contains("-") {
                    let v: Vec<i64> = s
                        .split('-')
                        .map(|x| x.parse::<i64>().expect("not a number!"))
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

impl Distribution<Range> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Range {
        Range(rng.gen_range(1, 101)..=rng.gen_range(1, 101))
    }
}
