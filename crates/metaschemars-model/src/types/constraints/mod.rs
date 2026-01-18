mod allowed_values;
mod cardinality;
mod expect;
mod matches;

pub use allowed_values::*;
pub use cardinality::*;
pub use expect::*;
pub use matches::*;

use serde::{Deserialize, Serialize};

/// All constraint types supported by Metaschema.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Constraint {
    AllowedValues(AllowedValues),
    Expect(Expect),
    Matches(Matches),
    HasCardinality(HasCardinality),
    // Future: Index, IndexHasKey, IsUnique, Let, Report
}
