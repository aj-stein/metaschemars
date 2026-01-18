mod allowed_values;
mod cardinality;
mod expect;
mod matches;

pub use allowed_values::*;
pub use cardinality::*;
pub use expect::*;
pub use matches::*;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{self, MapAccess, Visitor};
use std::fmt;

/// All constraint types supported by Metaschema.
#[derive(Debug, Clone, PartialEq)]
pub enum Constraint {
    AllowedValues(AllowedValues),
    Expect(Expect),
    Matches(Matches),
    HasCardinality(HasCardinality),
    // Future: Index, IndexHasKey, IsUnique, Let, Report
}

impl Serialize for Constraint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(1))?;
        match self {
            Constraint::AllowedValues(v) => map.serialize_entry("allowed-values", v)?,
            Constraint::Expect(v) => map.serialize_entry("expect", v)?,
            Constraint::Matches(v) => map.serialize_entry("matches", v)?,
            Constraint::HasCardinality(v) => map.serialize_entry("has-cardinality", v)?,
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for Constraint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ConstraintVisitor;

        impl<'de> Visitor<'de> for ConstraintVisitor {
            type Value = Constraint;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map with a single key: allowed-values, expect, matches, or has-cardinality")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Constraint, M::Error>
            where
                M: MapAccess<'de>,
            {
                let key: String = map.next_key()?
                    .ok_or_else(|| de::Error::custom("expected a constraint key"))?;

                let result = match key.as_str() {
                    "allowed-values" => {
                        let value: AllowedValues = map.next_value()?;
                        Constraint::AllowedValues(value)
                    }
                    "expect" => {
                        let value: Expect = map.next_value()?;
                        Constraint::Expect(value)
                    }
                    "matches" => {
                        let value: Matches = map.next_value()?;
                        Constraint::Matches(value)
                    }
                    "has-cardinality" => {
                        let value: HasCardinality = map.next_value()?;
                        Constraint::HasCardinality(value)
                    }
                    _ => return Err(de::Error::unknown_variant(
                        &key,
                        &["allowed-values", "expect", "matches", "has-cardinality"],
                    )),
                };

                Ok(result)
            }
        }

        deserializer.deserialize_map(ConstraintVisitor)
    }
}
