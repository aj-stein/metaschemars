mod assembly;
mod field;
mod flag;

pub use assembly::*;
pub use field::*;
pub use flag::*;

use serde::{Deserialize, Serialize};

/// A top-level definition in a Metaschema module.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Definition {
    DefineFlag(DefineFlag),
    DefineField(DefineField),
    DefineAssembly(DefineAssembly),
}
