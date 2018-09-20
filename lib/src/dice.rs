use crate::error::Error;
use crate::{RNG_DICE_SIZES, RNG_MAX_DICE_AMOUNT};
use rand::distributions::{Distribution, Standard};
use rand::{thread_rng, Rng};
use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// A dice for determining what to roll on a `Table`
#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dice {
    #[doc(hidden)]
    amount: u16,
    #[doc(hidden)]
    size: u16,
    // TODO add optional modifier +X where
    // X: i8/16/32?
}

impl Dice {
    /// Create a new `Dice`
    // TODO Add dice sizing errors
    pub fn new(amount: u16, size: u16) -> Self {
        Dice { amount, size }
    }

    /// Roll the `Dice`
    pub fn roll(&self) -> u32 {
        (0..self.amount).fold(0, |acc, _| {
            thread_rng().gen_range(1, u32::from(self.size + 1)) + acc
        })
    }

    /// The minimum value that this `Dice` can give
    pub fn min(&self) -> u32 {
        u32::from(self.amount)
    }

    /// The maximum value that this `Dice` can give
    pub fn max(&self) -> u32 {
        u32::from(self.amount) * u32::from(self.size)
    }

    /// The amount of `Dice` rolled
    pub fn amount(&self) -> u16 {
        self.amount
    }

    /// The size of the `Dice` rolled
    pub fn size(&self) -> u16 {
        self.size
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

impl FromStr for Dice {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('d') {
            let v: Vec<&str> = s.split('d').collect();
            if v.len() == 2 {
                let amount = v[0]
                    .parse::<u16>()
                    .map_err(|_| Error::invalid_dice_section(v[0], stringify!(amount)))?;
                let size = v[1]
                    .parse::<u16>()
                    .map_err(|_| Error::invalid_dice_section(v[1], stringify!(size)))?;

                Ok(Dice::new(amount, size))
            } else {
                Err(Error::invalid_dice(s))
            }
        } else {
            Err(Error::invalid_dice(s))
        }
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
                Ok(Dice::new(amount, size))
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
                                return Err(de::Error::duplicate_field(stringify!(amount)));
                            }
                            amount = Some(map.next_value()?);
                        }
                        Field::Size => {
                            if size.is_some() {
                                return Err(de::Error::duplicate_field(stringify!(size)));
                            }
                            size = Some(map.next_value()?);
                        }
                    }
                }
                let amount = amount.ok_or_else(|| de::Error::missing_field(stringify!(amount)))?;
                let size = size.ok_or_else(|| de::Error::missing_field(stringify!(size)))?;
                Ok(Dice::new(amount, size))
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Dice::from_str(s).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_any(DiceVisitor)
    }
}
