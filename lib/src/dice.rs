use core::{
    fmt,
    num::{NonZeroU16, NonZeroU8},
    ops::RangeInclusive,
    str::FromStr,
};
use crate::error::Error;
use rand::{thread_rng, Rng};
use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde_derive::{Deserialize, Serialize};

#[cfg(test)]
mod test;

/// A `Dice` for determining what to roll on a `Table`
#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dice {
    #[doc(hidden)]
    amount: NonZeroU8,
    #[doc(hidden)]
    size: NonZeroU16,
    // TODO add optional modifier +`mod` where
    // mod: i8 / i16?
}

impl Dice {
    /// Create a new `Dice`
    pub fn new(amount: NonZeroU8, size: NonZeroU16) -> Self {
        Dice { amount, size }
    }

    /// Create a new `Dice` if and only if both inputs are > 0
    pub fn maybe_new(amount: u8, size: u16) -> Option<Self> {
        let amount = NonZeroU8::new(amount);
        let size = NonZeroU16::new(size);
        if let None = amount {
            None
        } else if let None = size {
            None
        } else {
            let amount = amount.expect("Expected a NonZeroU16, but found None");
            let size = size.expect("Expected a NonZeroU16, but found None");
            Some(Dice { amount, size })
        }
    }

    /// Roll the `Dice`
    pub fn roll(&self) -> u32 {
        (0..self.amount.get())
            .map(|_| thread_rng().gen_range(1, u32::from(self.size.get() + 1)))
            .sum()
    }

    /// The minimum value that this `Dice` can give
    pub fn min_val(&self) -> u32 {
        u32::from(self.amount.get())
    }

    /// The maximum value that this `Dice` can give
    pub fn max_val(&self) -> u32 {
        u32::from(self.amount.get()) * u32::from(self.size.get())
    }

    /// The amount of `Dice` rolled
    pub fn amount(&self) -> u8 {
        self.amount.get()
    }

    /// The size of the `Dice` rolled
    pub fn size(&self) -> u16 {
        self.size.get()
    }

    /// An iterator over all the values of this `Dice`
    pub fn values(&self) -> RangeInclusive<u32> {
        self.min_val()..=self.max_val()
    }

    /// The probability of getting the value `n` from this `Dice`
    ///
    /// If the value `n` is outside the posibility space of the
    /// `Dice` then a `0` is returned, indicating no probability
    pub fn probability(&self, n: u32) -> f64 {
        if self.min_val() > n || self.max_val() < n {
            // If outside the range, there is no probability
            0.0
        } else if self.amount() == 1 {
            // If single die, then there is only
            // one way to get any number
            1.0 / f64::from(self.size())
        } else {
            // If there are multiple dice, then
            // we need the number of possible combinations
            // that will give us that number,
            // divided by our dice size to the power of our amount.
            // So rolling a 2 on a 2d6, has only 1 combination (1,1)
            // and 6^2 = 36, so the end result is 1/36
            self.possible_combinations(n)
                / f64::from(u32::from(self.size()).pow(u32::from(self.amount())))
        }
    }

    /// The amount of ways in which the dice could be rolled to end
    /// with the given amount
    ///
    /// Example: 2d6, to get a 12, you only have 1 way, (6,6)
    pub fn possible_combinations(&self, n: u32) -> f64 {
        let mut combinations = vec![vec![0.0; (n as usize) + 1]; usize::from(self.amount()) + 1];
        combinations[0][0] = 1.0;

        for i in 1..=usize::from(self.amount()) {
            for j in 1..=(n as usize) {
                if j < i || j > usize::from(self.size()) * i {
                    combinations[i][j] = 0.0;
                } else {
                    let mut k = 1;
                    while k <= usize::from(self.size()) && j >= k {
                        combinations[i][j] += combinations[i - 1][j - k];
                        k += 1;
                    }
                }
            }
        }

        combinations[usize::from(self.amount())][n as usize]
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
                let amount: u8 = v[0]
                    .parse()
                    .map_err(|_| Error::invalid_dice_section(v[0], stringify!(amount)))?;
                let amount = NonZeroU8::new(amount)
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
                let amount: NonZeroU8 = seq
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
                let amount: NonZeroU8 =
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
