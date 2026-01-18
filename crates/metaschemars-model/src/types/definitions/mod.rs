mod assembly;
mod field;
mod flag;

pub use assembly::*;
pub use field::*;
pub use flag::*;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{self, MapAccess, Visitor};
use std::fmt;

/// A top-level definition in a Metaschema module.
#[derive(Debug, Clone, PartialEq)]
pub enum Definition {
    DefineFlag(DefineFlag),
    DefineField(DefineField),
    DefineAssembly(DefineAssembly),
}

impl Serialize for Definition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(1))?;
        match self {
            Definition::DefineFlag(f) => map.serialize_entry("define-flag", f)?,
            Definition::DefineField(f) => map.serialize_entry("define-field", f)?,
            Definition::DefineAssembly(a) => map.serialize_entry("define-assembly", a)?,
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for Definition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DefinitionVisitor;

        impl<'de> Visitor<'de> for DefinitionVisitor {
            type Value = Definition;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map with a single key: define-flag, define-field, or define-assembly")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Definition, M::Error>
            where
                M: MapAccess<'de>,
            {
                let key: String = map.next_key()?
                    .ok_or_else(|| de::Error::custom("expected a definition key"))?;

                let result = match key.as_str() {
                    "define-flag" => {
                        let value: DefineFlag = map.next_value()?;
                        Definition::DefineFlag(value)
                    }
                    "define-field" => {
                        let value: DefineField = map.next_value()?;
                        Definition::DefineField(value)
                    }
                    "define-assembly" => {
                        let value: DefineAssembly = map.next_value()?;
                        Definition::DefineAssembly(value)
                    }
                    _ => return Err(de::Error::unknown_variant(
                        &key,
                        &["define-flag", "define-field", "define-assembly"],
                    )),
                };

                Ok(result)
            }
        }

        deserializer.deserialize_map(DefinitionVisitor)
    }
}
