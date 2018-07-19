#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Table {
    Simple {
        die: String,
        results: Vec<SimpleRow>,
    },
    Complex {
        die: Die,
        results: Vec<ComplexRow>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Die {
    pub amount: i64,
    pub size: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleRow {
    pub roll: i64,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComplexRow {
    pub range: Range,
    pub value: String,
}

#[derive(Debug)]
pub struct Range(pub ::std::ops::RangeInclusive<i64>);

impl Range {
    pub fn contains(&self, v: i64) -> bool {
        (v < *self.0.start() || v > *self.0.end()) == false
    }
}

use serde::{Serialize, Serializer};

impl Serialize for Range {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}-{}", self.0.start(), self.0.end()))
    }
}

use serde::de::{Deserialize, Deserializer, Visitor};

impl<'de> Deserialize<'de> for Range {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de;
        use serde::de::Unexpected;
        use std::fmt;

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

impl Table {
    pub fn get(&self) -> String {
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();

        match self {
            Table::Simple { results: v, .. } => {
                if let Some(r) = rng.choose(&v) {
                    format!("Rolled a `{}`, resulting in: {}", r.roll, &r.value)
                } else {
                    String::new()
                }
            }
            Table::Complex { die, results: v } => {
                let res = (0..die.amount).fold(0, |acc, _| rng.gen_range(0, die.size) + acc);
                if let Some(val) = v
                    .iter()
                    .filter(|ComplexRow { range: r, .. }| r.contains(res))
                    .next()
                {
                    format!("Rolled a `{}`, resulting in: {}", res, &val.value)
                } else {
                    String::new()
                }
            }
        }
    }
}
