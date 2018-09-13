mod table_result;
pub use self::table_result::TableResult;

#[cfg(test)]
mod tests;

use crate::{Column, Dice, Result, Rows};
use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use std::fmt;

/// A table that can be rolled on
#[derive(Debug, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Table {
    #[doc(hidden)]
    dice: Dice,
    #[doc(hidden)]
    heading: Column,
    #[doc(hidden)]
    results: Rows,
}

impl Table {
    /// Crate a new table
    pub fn new<C, R>(dice: Dice, heading: C, results: R) -> Result<Self>
    where
        C: Into<Column>,
        R: Into<Rows>,
    {
        let heading: Column = heading.into();
        let results: Rows = results.into();

        results.validate(&dice)?;

        Ok(Table {
            dice,
            heading,
            results,
        })
    }

    /// Perform a roll on this table
    pub fn roll(&self) -> TableResult {
        let roll = self.dice.roll();

        match self
            .results
            .iter()
            .enumerate()
            .find(|(_, row)| **row == roll)
            .map(|(i, _)| TableResult::new(roll, i))
        {
            Some(t) => t,
            None => unreachable!("Table was created without all posible rolls!"),
        }
    }
}

impl<'de> Deserialize<'de> for Table {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Dice,
            Heading,
            Results,
        };

        struct TableVisitor;

        impl<'de> Visitor<'de> for TableVisitor {
            type Value = Table;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Table")
            }

            fn visit_seq<V>(self, mut seq: V) -> std::result::Result<Table, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let dice: Dice = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let heading: Column = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let results: Rows = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Table::new(dice, heading, results).map_err(de::Error::custom)
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Table, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut dice = None;
                let mut heading = None;
                let mut results = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Dice => {
                            if dice.is_some() {
                                return Err(de::Error::duplicate_field("dice"));
                            }
                            dice = Some(map.next_value()?);
                        }
                        Field::Heading => {
                            if heading.is_some() {
                                return Err(de::Error::duplicate_field("heading"));
                            }
                            heading = Some(map.next_value()?);
                        }
                        Field::Results => {
                            if results.is_some() {
                                return Err(de::Error::duplicate_field("results"));
                            }
                            results = Some(map.next_value()?);
                        }
                    }
                }
                let dice: Dice = dice.ok_or_else(|| de::Error::missing_field("dice"))?;
                let heading: Column = heading.ok_or_else(|| de::Error::missing_field("heading"))?;
                let results: Rows = results.ok_or_else(|| de::Error::missing_field("results"))?;
                Table::new(dice, heading, results).map_err(de::Error::custom)
            }
        }

        const FIELDS: &'static [&'static str] = &["dice", "heading", "results"];
        deserializer.deserialize_struct("Table", FIELDS, TableVisitor)
    }
}
