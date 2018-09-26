use crate::error::Error;
use rand::{thread_rng, Rng};
use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::num::NonZeroU16;
use std::str::FromStr;

/// A dice for determining what to roll on a `Table`
#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dice {
    #[doc(hidden)]
    amount: NonZeroU16,
    #[doc(hidden)]
    size: NonZeroU16,
    // TODO add optional modifier +X where
    // X: i8/16/32?
}

impl Dice {
    /// Create a new `Dice`
    pub fn new(amount: NonZeroU16, size: NonZeroU16) -> Self {
        Dice { amount, size }
    }

    /// Roll the `Dice`
    pub fn roll(&self) -> u32 {
        (0..self.amount.get()).fold(0, |acc, _| {
            thread_rng().gen_range(1, u32::from(self.size.get() + 1)) + acc
        })
    }

    /// The minimum value that this `Dice` can give
    pub fn min(&self) -> u32 {
        u32::from(self.amount.get())
    }

    /// The maximum value that this `Dice` can give
    pub fn max(&self) -> u32 {
        u32::from(self.amount.get()) * u32::from(self.size.get())
    }

    /// The amount of `Dice` rolled
    pub fn amount(&self) -> u16 {
        self.amount.get()
    }

    /// The size of the `Dice` rolled
    pub fn size(&self) -> u16 {
        self.size.get()
    }
}

impl fmt::Display for Dice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}d{}", self.amount, self.size)
    }
}

impl FromStr for Dice {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('d') {
            let v: Vec<&str> = s.split('d').collect();
            if v.len() == 2 {
                let amount: u16 = v[0]
                    .parse()
                    .map_err(|_| Error::invalid_dice_section(v[0], stringify!(amount)))?;
                let amount = NonZeroU16::new(amount)
                    .ok_or_else(|| Error::invalid_dice_section(v[0], stringify!(amount)))?;

                let size: u16 = v[1]
                    .parse()
                    .map_err(|_| Error::invalid_dice_section(v[1], stringify!(size)))?;
                let size = NonZeroU16::new(size)
                    .ok_or_else(|| Error::invalid_dice_section(v[1], stringify!(size)))?;

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
                let amount: NonZeroU16 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let size: NonZeroU16 = seq
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
                let amount: NonZeroU16 =
                    amount.ok_or_else(|| de::Error::missing_field(stringify!(amount)))?;
                let size: NonZeroU16 =
                    size.ok_or_else(|| de::Error::missing_field(stringify!(size)))?;
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
