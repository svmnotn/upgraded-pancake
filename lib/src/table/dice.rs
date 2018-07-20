use rand::distributions::{Distribution, Standard};
use rand::{thread_rng, Rng};
use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Unexpected, Visitor};
use std::fmt;
use {RNG_DICE_SIZES, RNG_MAX_DICE_AMOUNT};

#[derive(Debug, Serialize)]
pub struct Dice {
    amount: u16,
    size: u16,
}

impl Dice {
    pub fn roll(&self) -> u64 {
        (0..self.amount).fold(0, |acc, _| {
            thread_rng().gen_range(1 as u64, (self.size + 1) as u64) + acc
        })
    }

    pub fn min(&self) -> u64 {
        self.amount as u64
    }

    pub fn max(&self) -> u64 {
        (self.amount as u64) * (self.size as u64)
    }

    pub fn amount(&self) -> u16 {
        self.amount
    }

    pub fn size(&self) -> u16 {
        self.size
    }
}

impl<'de> Deserialize<'de> for Dice {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Amount,
            Size,
        };

        struct DiceVisitor;

        impl<'de> Visitor<'de> for DiceVisitor {
            type Value = Dice;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Dice")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Dice, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let amount = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let size = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(Dice { amount, size })
            }

            fn visit_map<V>(self, mut map: V) -> Result<Dice, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut amount = None;
                let mut size = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Amount => {
                            if amount.is_some() {
                                return Err(de::Error::duplicate_field("secs"));
                            }
                            amount = Some(map.next_value()?);
                        }
                        Field::Size => {
                            if size.is_some() {
                                return Err(de::Error::duplicate_field("nanos"));
                            }
                            size = Some(map.next_value()?);
                        }
                    }
                }
                let amount = amount.ok_or_else(|| de::Error::missing_field("secs"))?;
                let size = size.ok_or_else(|| de::Error::missing_field("nanos"))?;
                Ok(Dice { amount, size })
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if s.contains("d") {
                    let v: Vec<u16> = s
                        .split('d')
                        .map(|x| x.parse::<u16>().expect("not a number!"))
                        .collect(); // TODO make into error
                    Ok(Dice {
                        amount: v[0],
                        size: v[1],
                    })
                } else {
                    Err(de::Error::invalid_value(Unexpected::Str(s), &self))
                }
            }
        }

        deserializer.deserialize_any(DiceVisitor)
    }
}

impl fmt::Display for Dice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}d{}", self.amount, self.size)
    }
}

impl Distribution<Dice> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Dice {
        Dice {
            amount: rng.gen_range(1, RNG_MAX_DICE_AMOUNT),
            size: *rng.choose(&RNG_DICE_SIZES).expect("sizes where empty?"),
        }
    }
}
