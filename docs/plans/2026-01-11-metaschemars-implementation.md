# metaschemars Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a Rust library that parses Metaschema modules from YAML into strongly-typed Rust structs.

**Architecture:** Workspace with 4 crates - model (data structures), yaml (YAML parsing), and facade (re-exports). Error accumulation via miette.

**Tech Stack:** Rust, serde, serde_yaml, thiserror, miette

---

## Implementation Status

| Task | Description | Status |
|------|-------------|--------|
| 1 | Initialize Workspace Structure | ✅ Complete |
| 2 | Define Basic Error Types | ✅ Complete |
| 3 | Define DataType Enum | ✅ Complete |
| 4 | Define Common Types | ✅ Complete |
| 5 | Define DefineFlag | ✅ Complete |
| 6 | Define Constraints | ✅ Complete |
| 7 | Add Constraints to DefineFlag | ✅ Complete |
| 8 | Define DefineField | ✅ Complete |
| 9 | Define DefineAssembly and Model | ✅ Complete |
| 10 | Define Module (Root Structure) | ✅ Complete |
| 11 | Implement YAML Parser | ✅ Complete |
| 12 | Wire Up Facade Crate | ✅ Complete |
| 13 | Add Integration Tests | ✅ Complete |
| 14 | Final Verification | ⏳ Not started |

**Last updated:** 2026-01-18

**Notes:**
- Tasks 10-13 implemented custom Serialize/Deserialize for `Definition` and `Constraint` enums to handle Metaschema's object-key YAML format instead of YAML tags
- 43 tests passing (32 model + 4 yaml + 6 integration + 1 doctest)

---

## Task 1: Initialize Workspace Structure

**Files:**
- Create: `Cargo.toml` (workspace root)
- Create: `crates/metaschemars-model/Cargo.toml`
- Create: `crates/metaschemars-model/src/lib.rs`
- Create: `crates/metaschemars-yaml/Cargo.toml`
- Create: `crates/metaschemars-yaml/src/lib.rs`
- Create: `crates/metaschemars/Cargo.toml`
- Create: `crates/metaschemars/src/lib.rs`

**Step 1: Create workspace root Cargo.toml**

```toml
[workspace]
resolver = "2"
members = [
    "crates/metaschemars-model",
    "crates/metaschemars-yaml",
    "crates/metaschemars",
]

[workspace.package]
version = "0.1.0"
edition = "2024"
license = "GPL-3.0"
repository = "https://github.com/aj-stein/metaschemars"

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
serde_yaml = "0.9"
thiserror = "2"
miette = { version = "7", features = ["fancy"] }
```

**Step 2: Create metaschemars-model crate**

`crates/metaschemars-model/Cargo.toml`:
```toml
[package]
name = "metaschemars-model"
version.workspace = true
edition.workspace = true
license.workspace = true
description = "Core data model for Metaschema modules"

[dependencies]
serde = { workspace = true }
thiserror = { workspace = true }
miette = { workspace = true }
```

`crates/metaschemars-model/src/lib.rs`:
```rust
//! Core data model for Metaschema modules.

pub mod types;
pub mod error;

pub use types::*;
pub use error::*;
```

**Step 3: Create metaschemars-yaml crate**

`crates/metaschemars-yaml/Cargo.toml`:
```toml
[package]
name = "metaschemars-yaml"
version.workspace = true
edition.workspace = true
license.workspace = true
description = "YAML parsing for Metaschema modules"

[dependencies]
metaschemars-model = { path = "../metaschemars-model" }
serde = { workspace = true }
serde_yaml = { workspace = true }
```

`crates/metaschemars-yaml/src/lib.rs`:
```rust
//! YAML parsing for Metaschema modules.

pub mod parser;

pub use parser::*;
```

**Step 4: Create metaschemars facade crate**

`crates/metaschemars/Cargo.toml`:
```toml
[package]
name = "metaschemars"
version.workspace = true
edition.workspace = true
license.workspace = true
description = "Rust library for parsing Metaschema module definitions"

[dependencies]
metaschemars-model = { path = "../metaschemars-model" }
metaschemars-yaml = { path = "../metaschemars-yaml" }

[features]
default = ["yaml"]
yaml = []
```

`crates/metaschemars/src/lib.rs`:
```rust
//! Rust library for parsing Metaschema module definitions.

pub use metaschemars_model::*;

#[cfg(feature = "yaml")]
pub use metaschemars_yaml::*;
```

**Step 5: Verify workspace builds**

Run: `cargo build`
Expected: Successful compilation with no errors

**Step 6: Commit**

```bash
git add -A
git commit -m "chore: Initialize workspace with model, yaml, and facade crates"
```

---

## Task 2: Define Basic Error Types

**Files:**
- Create: `crates/metaschemars-model/src/error.rs`
- Modify: `crates/metaschemars-model/src/lib.rs`

**Step 1: Write test for error types**

Create `crates/metaschemars-model/src/error.rs`:
```rust
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug, Clone)]
pub enum ParseError {
    #[error("Failed to read file: {path}")]
    IoError {
        path: String,
        message: String,
    },

    #[error("Invalid YAML syntax: {message}")]
    #[diagnostic(code(metaschemars::yaml::syntax))]
    YamlSyntax {
        message: String,
        #[label("here")]
        span: Option<SourceSpan>,
    },

    #[error("Unknown data type: {value}")]
    #[diagnostic(code(metaschemars::unknown_datatype))]
    UnknownDataType {
        value: String,
    },

    #[error("Missing required field: {field}")]
    #[diagnostic(code(metaschemars::missing_field))]
    MissingField {
        field: String,
    },
}

/// Result type that can hold a value and accumulated errors.
#[derive(Debug)]
pub struct ParseResult<T> {
    pub value: Option<T>,
    pub errors: Vec<ParseError>,
}

impl<T> ParseResult<T> {
    pub fn ok(value: T) -> Self {
        Self {
            value: Some(value),
            errors: vec![],
        }
    }

    pub fn err(error: ParseError) -> Self {
        Self {
            value: None,
            errors: vec![error],
        }
    }

    pub fn with_errors(value: T, errors: Vec<ParseError>) -> Self {
        Self {
            value: Some(value),
            errors,
        }
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn into_result(self) -> Result<T, Vec<ParseError>> {
        if self.errors.is_empty() {
            self.value.ok_or_else(|| vec![])
        } else {
            Err(self.errors)
        }
    }
}

impl<T> Default for ParseResult<T> {
    fn default() -> Self {
        Self {
            value: None,
            errors: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_result_ok() {
        let result = ParseResult::ok(42);
        assert!(!result.has_errors());
        assert_eq!(result.value, Some(42));
    }

    #[test]
    fn test_parse_result_err() {
        let result: ParseResult<i32> = ParseResult::err(ParseError::MissingField {
            field: "name".to_string(),
        });
        assert!(result.has_errors());
        assert_eq!(result.value, None);
    }

    #[test]
    fn test_parse_result_into_result_ok() {
        let result = ParseResult::ok(42);
        assert_eq!(result.into_result(), Ok(42));
    }

    #[test]
    fn test_parse_result_into_result_err() {
        let result: ParseResult<i32> = ParseResult::err(ParseError::MissingField {
            field: "name".to_string(),
        });
        let err = result.into_result().unwrap_err();
        assert_eq!(err.len(), 1);
    }
}
```

**Step 2: Update lib.rs**

`crates/metaschemars-model/src/lib.rs`:
```rust
//! Core data model for Metaschema modules.

pub mod error;

pub use error::*;
```

**Step 3: Run tests**

Run: `cargo test -p metaschemars-model`
Expected: 4 tests pass

**Step 4: Commit**

```bash
git add -A
git commit -m "feat(model): Add ParseError and ParseResult types with miette integration"
```

---

## Task 3: Define DataType Enum

**Files:**
- Create: `crates/metaschemars-model/src/types/data_type.rs`
- Create: `crates/metaschemars-model/src/types/mod.rs`
- Modify: `crates/metaschemars-model/src/lib.rs`

**Step 1: Create types module**

`crates/metaschemars-model/src/types/mod.rs`:
```rust
mod data_type;

pub use data_type::*;
```

**Step 2: Write DataType with tests**

`crates/metaschemars-model/src/types/data_type.rs`:
```rust
use serde::{Deserialize, Serialize};

/// All data types supported by Metaschema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DataType {
    // Character-based
    String,
    Token,
    #[serde(rename = "email-address")]
    EmailAddress,
    Hostname,
    #[serde(rename = "ip-v4-address")]
    IpV4Address,
    #[serde(rename = "ip-v6-address")]
    IpV6Address,
    Uri,
    #[serde(rename = "uri-reference")]
    UriReference,
    Uuid,
    #[serde(rename = "qname")]
    QName,

    // Numeric
    Decimal,
    Integer,
    #[serde(rename = "non-negative-integer")]
    NonNegativeInteger,
    #[serde(rename = "positive-integer")]
    PositiveInteger,

    // Boolean/Binary
    Boolean,
    Base64,

    // Temporal
    Date,
    #[serde(rename = "date-with-timezone")]
    DateWithTimezone,
    #[serde(rename = "date-time")]
    DateTime,
    #[serde(rename = "date-time-with-timezone")]
    DateTimeWithTimezone,
    #[serde(rename = "day-time-duration")]
    DayTimeDuration,
    #[serde(rename = "year-month-duration")]
    YearMonthDuration,

    // Markup
    #[serde(rename = "markup-line")]
    MarkupLine,
    #[serde(rename = "markup-multiline")]
    MarkupMultiline,
}

impl Default for DataType {
    fn default() -> Self {
        DataType::String
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_string() {
        let dt: DataType = serde_yaml::from_str("string").unwrap();
        assert_eq!(dt, DataType::String);
    }

    #[test]
    fn test_deserialize_kebab_case() {
        let dt: DataType = serde_yaml::from_str("non-negative-integer").unwrap();
        assert_eq!(dt, DataType::NonNegativeInteger);
    }

    #[test]
    fn test_deserialize_markup() {
        let dt: DataType = serde_yaml::from_str("markup-multiline").unwrap();
        assert_eq!(dt, DataType::MarkupMultiline);
    }

    #[test]
    fn test_serialize_roundtrip() {
        let dt = DataType::DateTimeWithTimezone;
        let yaml = serde_yaml::to_string(&dt).unwrap();
        let parsed: DataType = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(parsed, dt);
    }
}
```

**Step 3: Update lib.rs**

`crates/metaschemars-model/src/lib.rs`:
```rust
//! Core data model for Metaschema modules.

pub mod error;
pub mod types;

pub use error::*;
pub use types::*;
```

**Step 4: Run tests**

Run: `cargo test -p metaschemars-model`
Expected: All tests pass (4 error tests + 4 data_type tests)

**Step 5: Commit**

```bash
git add -A
git commit -m "feat(model): Add DataType enum with serde support"
```

---

## Task 4: Define Common Types (Scope, YesNo, Markup, Property)

**Files:**
- Create: `crates/metaschemars-model/src/types/common.rs`
- Modify: `crates/metaschemars-model/src/types/mod.rs`

**Step 1: Write common types with tests**

`crates/metaschemars-model/src/types/common.rs`:
```rust
use serde::{Deserialize, Serialize};

/// Scope of a definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Scope {
    Local,
    Global,
}

impl Default for Scope {
    fn default() -> Self {
        Scope::Global
    }
}

/// Yes/No boolean alternative used in Metaschema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum YesNo {
    Yes,
    No,
}

impl From<YesNo> for bool {
    fn from(yn: YesNo) -> bool {
        matches!(yn, YesNo::Yes)
    }
}

impl From<bool> for YesNo {
    fn from(b: bool) -> YesNo {
        if b { YesNo::Yes } else { YesNo::No }
    }
}

/// Markup text content (simplified as String for now).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Markup(pub String);

impl From<String> for Markup {
    fn from(s: String) -> Self {
        Markup(s)
    }
}

impl From<&str> for Markup {
    fn from(s: &str) -> Self {
        Markup(s.to_string())
    }
}

/// A property with name, optional namespace, and value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    pub value: String,
}

/// Occurrence bounds for instances.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Occurs {
    Bounded(u32),
    #[serde(rename = "unbounded")]
    Unbounded,
}

/// Level for constraint severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Error,
    Warning,
    Informational,
}

impl Default for Level {
    fn default() -> Self {
        Level::Error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scope_deserialize() {
        let scope: Scope = serde_yaml::from_str("local").unwrap();
        assert_eq!(scope, Scope::Local);
    }

    #[test]
    fn test_yesno_to_bool() {
        assert!(bool::from(YesNo::Yes));
        assert!(!bool::from(YesNo::No));
    }

    #[test]
    fn test_markup_from_str() {
        let m = Markup::from("test");
        assert_eq!(m.0, "test");
    }

    #[test]
    fn test_property_deserialize() {
        let yaml = r#"
name: test-prop
value: test-value
"#;
        let prop: Property = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(prop.name, "test-prop");
        assert_eq!(prop.value, "test-value");
        assert_eq!(prop.namespace, None);
    }

    #[test]
    fn test_occurs_bounded() {
        let occurs: Occurs = serde_yaml::from_str("5").unwrap();
        assert_eq!(occurs, Occurs::Bounded(5));
    }

    #[test]
    fn test_level_default() {
        assert_eq!(Level::default(), Level::Error);
    }
}
```

**Step 2: Update types/mod.rs**

`crates/metaschemars-model/src/types/mod.rs`:
```rust
mod common;
mod data_type;

pub use common::*;
pub use data_type::*;
```

**Step 3: Run tests**

Run: `cargo test -p metaschemars-model`
Expected: All tests pass

**Step 4: Commit**

```bash
git add -A
git commit -m "feat(model): Add common types (Scope, YesNo, Markup, Property, Occurs, Level)"
```

---

## Task 5: Define DefineFlag

**Files:**
- Create: `crates/metaschemars-model/src/types/definitions/mod.rs`
- Create: `crates/metaschemars-model/src/types/definitions/flag.rs`
- Modify: `crates/metaschemars-model/src/types/mod.rs`

**Step 1: Create definitions module structure**

`crates/metaschemars-model/src/types/definitions/mod.rs`:
```rust
mod flag;

pub use flag::*;
```

**Step 2: Write DefineFlag with tests**

`crates/metaschemars-model/src/types/definitions/flag.rs`:
```rust
use serde::{Deserialize, Serialize};
use crate::types::{DataType, Markup, Property, Scope};

/// A flag definition (leaf node with typed value).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DefineFlag {
    pub name: String,

    #[serde(rename = "as-type", skip_serializing_if = "Option::is_none")]
    pub as_type: Option<DataType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub formal_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Markup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<Markup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_name: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define_flag_minimal() {
        let yaml = r#"
name: test-flag
"#;
        let flag: DefineFlag = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(flag.name, "test-flag");
        assert_eq!(flag.as_type, None);
    }

    #[test]
    fn test_define_flag_with_type() {
        let yaml = r#"
name: count
as-type: integer
default: "0"
formal-name: Count
description: A count of items
"#;
        let flag: DefineFlag = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(flag.name, "count");
        assert_eq!(flag.as_type, Some(DataType::Integer));
        assert_eq!(flag.default, Some("0".to_string()));
        assert_eq!(flag.formal_name, Some("Count".to_string()));
    }

    #[test]
    fn test_define_flag_with_scope() {
        let yaml = r#"
name: internal-flag
scope: local
"#;
        let flag: DefineFlag = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(flag.scope, Some(Scope::Local));
    }
}
```

**Step 3: Update types/mod.rs**

`crates/metaschemars-model/src/types/mod.rs`:
```rust
mod common;
mod data_type;
mod definitions;

pub use common::*;
pub use data_type::*;
pub use definitions::*;
```

**Step 4: Run tests**

Run: `cargo test -p metaschemars-model`
Expected: All tests pass

**Step 5: Commit**

```bash
git add -A
git commit -m "feat(model): Add DefineFlag definition type"
```

---

## Task 6: Define Constraints

**Files:**
- Create: `crates/metaschemars-model/src/types/constraints/mod.rs`
- Create: `crates/metaschemars-model/src/types/constraints/allowed_values.rs`
- Create: `crates/metaschemars-model/src/types/constraints/expect.rs`
- Create: `crates/metaschemars-model/src/types/constraints/matches.rs`
- Create: `crates/metaschemars-model/src/types/constraints/cardinality.rs`
- Modify: `crates/metaschemars-model/src/types/mod.rs`

**Step 1: Create constraints module**

`crates/metaschemars-model/src/types/constraints/mod.rs`:
```rust
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
```

**Step 2: Write AllowedValues**

`crates/metaschemars-model/src/types/constraints/allowed_values.rs`:
```rust
use serde::{Deserialize, Serialize};
use crate::types::{Level, Markup, YesNo};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AllowedValues {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<Level>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_other: Option<YesNo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensible: Option<YesNo>,

    #[serde(default, rename = "enum")]
    pub enums: Vec<EnumValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<Markup>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct EnumValue {
    pub value: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Markup>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allowed_values() {
        let yaml = r#"
allow-other: no
enum:
  - value: laptop
    description: Portable computer
  - value: desktop
    description: Desktop computer
"#;
        let av: AllowedValues = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(av.allow_other, Some(YesNo::No));
        assert_eq!(av.enums.len(), 2);
        assert_eq!(av.enums[0].value, "laptop");
    }
}
```

**Step 3: Write Expect**

`crates/metaschemars-model/src/types/constraints/expect.rs`:
```rust
use serde::{Deserialize, Serialize};
use crate::types::{Level, Markup};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Expect {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<Level>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    pub test: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<Markup>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expect() {
        let yaml = r#"
target: "."
test: "@min le @max"
message: Minimum must be less than maximum
"#;
        let exp: Expect = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(exp.test, "@min le @max");
        assert!(exp.message.is_some());
    }
}
```

**Step 4: Write Matches**

`crates/metaschemars-model/src/types/constraints/matches.rs`:
```rust
use serde::{Deserialize, Serialize};
use crate::types::{DataType, Level, Markup};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Matches {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<Level>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub datatype: Option<DataType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<Markup>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matches_regex() {
        let yaml = r#"
target: "."
regex: "^\\d{3}-\\d{2}-\\d{4}$"
"#;
        let m: Matches = serde_yaml::from_str(yaml).unwrap();
        assert!(m.regex.is_some());
    }
}
```

**Step 5: Write HasCardinality**

`crates/metaschemars-model/src/types/constraints/cardinality.rs`:
```rust
use serde::{Deserialize, Serialize};
use crate::types::{Level, Markup, Occurs};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct HasCardinality {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<Level>,

    pub target: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_occurs: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_occurs: Option<Occurs>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<Markup>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_cardinality() {
        let yaml = r#"
target: item
min-occurs: 1
"#;
        let hc: HasCardinality = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(hc.target, "item");
        assert_eq!(hc.min_occurs, Some(1));
    }
}
```

**Step 6: Update types/mod.rs**

`crates/metaschemars-model/src/types/mod.rs`:
```rust
mod common;
mod constraints;
mod data_type;
mod definitions;

pub use common::*;
pub use constraints::*;
pub use data_type::*;
pub use definitions::*;
```

**Step 7: Run tests**

Run: `cargo test -p metaschemars-model`
Expected: All tests pass

**Step 8: Commit**

```bash
git add -A
git commit -m "feat(model): Add constraint types (AllowedValues, Expect, Matches, HasCardinality)"
```

---

## Task 7: Add Constraints to DefineFlag

**Files:**
- Modify: `crates/metaschemars-model/src/types/definitions/flag.rs`

**Step 1: Update DefineFlag to include constraints**

Add to `crates/metaschemars-model/src/types/definitions/flag.rs`:

Update imports:
```rust
use crate::types::{Constraint, DataType, Markup, Property, Scope};
```

Add field to DefineFlag struct (after props):
```rust
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub constraints: Vec<Constraint>,
```

Add test:
```rust
    #[test]
    fn test_define_flag_with_constraint() {
        let yaml = r#"
name: status
as-type: token
constraints:
  - allowed-values:
      allow-other: no
      enum:
        - value: active
        - value: inactive
"#;
        let flag: DefineFlag = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(flag.constraints.len(), 1);
    }
```

**Step 2: Run tests**

Run: `cargo test -p metaschemars-model`
Expected: All tests pass

**Step 3: Commit**

```bash
git add -A
git commit -m "feat(model): Add constraints field to DefineFlag"
```

---

## Task 8: Define DefineField

**Files:**
- Create: `crates/metaschemars-model/src/types/definitions/field.rs`
- Create: `crates/metaschemars-model/src/types/instances/mod.rs`
- Create: `crates/metaschemars-model/src/types/instances/flag.rs`
- Modify: `crates/metaschemars-model/src/types/definitions/mod.rs`
- Modify: `crates/metaschemars-model/src/types/mod.rs`

**Step 1: Create flag instance types**

`crates/metaschemars-model/src/types/instances/mod.rs`:
```rust
mod flag;

pub use flag::*;
```

`crates/metaschemars-model/src/types/instances/flag.rs`:
```rust
use serde::{Deserialize, Serialize};
use crate::types::{DefineFlag, Markup, YesNo};

/// A flag instance - either a reference or inline definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FlagInstance {
    Reference(FlagReference),
    Inline(DefineFlag),
}

/// Reference to a top-level define-flag.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct FlagReference {
    #[serde(rename = "ref")]
    pub ref_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<YesNo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<Markup>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flag_reference() {
        let yaml = r#"
ref: my-flag
required: yes
"#;
        let fi: FlagInstance = serde_yaml::from_str(yaml).unwrap();
        match fi {
            FlagInstance::Reference(r) => {
                assert_eq!(r.ref_name, "my-flag");
                assert_eq!(r.required, Some(YesNo::Yes));
            }
            _ => panic!("Expected reference"),
        }
    }
}
```

**Step 2: Create DefineField**

`crates/metaschemars-model/src/types/definitions/field.rs`:
```rust
use serde::{Deserialize, Serialize};
use crate::types::{Constraint, DataType, FlagInstance, Markup, Property, Scope, YesNo};

/// JSON key configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct JsonKey {
    pub flag_ref: String,
}

/// A field definition (value with optional flags).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DefineField {
    pub name: String,

    #[serde(rename = "as-type", skip_serializing_if = "Option::is_none")]
    pub as_type: Option<DataType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapsible: Option<YesNo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub formal_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Markup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<Markup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_key: Option<JsonKey>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_value_key: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub flags: Vec<FlagInstance>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub constraints: Vec<Constraint>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define_field_minimal() {
        let yaml = r#"
name: title
"#;
        let field: DefineField = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(field.name, "title");
    }

    #[test]
    fn test_define_field_with_type_and_flags() {
        let yaml = r#"
name: description
as-type: markup-multiline
formal-name: Description
flags:
  - ref: lang
"#;
        let field: DefineField = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(field.as_type, Some(DataType::MarkupMultiline));
        assert_eq!(field.flags.len(), 1);
    }
}
```

**Step 3: Update definitions/mod.rs**

`crates/metaschemars-model/src/types/definitions/mod.rs`:
```rust
mod field;
mod flag;

pub use field::*;
pub use flag::*;
```

**Step 4: Update types/mod.rs**

`crates/metaschemars-model/src/types/mod.rs`:
```rust
mod common;
mod constraints;
mod data_type;
mod definitions;
mod instances;

pub use common::*;
pub use constraints::*;
pub use data_type::*;
pub use definitions::*;
pub use instances::*;
```

**Step 5: Run tests**

Run: `cargo test -p metaschemars-model`
Expected: All tests pass

**Step 6: Commit**

```bash
git add -A
git commit -m "feat(model): Add DefineField and FlagInstance types"
```

---

## Task 9: Define DefineAssembly and Model

**Files:**
- Create: `crates/metaschemars-model/src/types/definitions/assembly.rs`
- Create: `crates/metaschemars-model/src/types/instances/model.rs`
- Modify: `crates/metaschemars-model/src/types/definitions/mod.rs`
- Modify: `crates/metaschemars-model/src/types/instances/mod.rs`

**Step 1: Create model instance types**

`crates/metaschemars-model/src/types/instances/model.rs`:
```rust
use serde::{Deserialize, Serialize};
use crate::types::{DefineAssembly, DefineField, Markup, Occurs};

/// Grouping configuration for JSON/XML output.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GroupAs {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_json: Option<GroupInJson>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_xml: Option<GroupInXml>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GroupInJson {
    Array,
    SingletonOrArray,
    ByKey,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
pub enum GroupInXml {
    Ungrouped,
    Grouped,
}

/// Field instance in a model.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct FieldInstance {
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    pub ref_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_occurs: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_occurs: Option<Occurs>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_as: Option<GroupAs>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<Markup>,
}

/// Assembly instance in a model.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssemblyInstance {
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    pub ref_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_occurs: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_occurs: Option<Occurs>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_as: Option<GroupAs>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<Markup>,
}

/// Choice group in a model.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Choice {
    #[serde(default)]
    pub instances: Vec<ModelInstance>,
}

/// Any content placeholder.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Any {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_contents: Option<ProcessContents>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProcessContents {
    Strict,
    Lax,
    None,
}

/// Instance within a model.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ModelInstance {
    Field(FieldInstance),
    Assembly(AssemblyInstance),
    Choice(Choice),
    Any(Any),
}

/// The model section of an assembly.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Model {
    #[serde(default, flatten)]
    pub instances: Vec<ModelInstance>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_instance() {
        let yaml = r#"
ref: title
min-occurs: 1
max-occurs: 1
"#;
        let fi: FieldInstance = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(fi.ref_name, Some("title".to_string()));
        assert_eq!(fi.min_occurs, Some(1));
    }

    #[test]
    fn test_group_as() {
        let yaml = r#"
name: items
in-json: array
"#;
        let ga: GroupAs = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(ga.name, "items");
        assert_eq!(ga.in_json, Some(GroupInJson::Array));
    }
}
```

**Step 2: Create DefineAssembly**

`crates/metaschemars-model/src/types/definitions/assembly.rs`:
```rust
use serde::{Deserialize, Serialize};
use crate::types::{Constraint, FlagInstance, JsonKey, Markup, Model, Property, Scope};

/// An assembly definition (composite container).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DefineAssembly {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub formal_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Markup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<Markup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_key: Option<JsonKey>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub flags: Vec<FlagInstance>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<Model>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Property>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub constraints: Vec<Constraint>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define_assembly_minimal() {
        let yaml = r#"
name: catalog
"#;
        let asm: DefineAssembly = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(asm.name, "catalog");
    }

    #[test]
    fn test_define_assembly_with_root() {
        let yaml = r#"
name: catalog
root-name: catalog
formal-name: Catalog
description: A collection of controls
"#;
        let asm: DefineAssembly = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(asm.root_name, Some("catalog".to_string()));
    }
}
```

**Step 3: Update definitions/mod.rs**

`crates/metaschemars-model/src/types/definitions/mod.rs`:
```rust
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
```

**Step 4: Update instances/mod.rs**

`crates/metaschemars-model/src/types/instances/mod.rs`:
```rust
mod flag;
mod model;

pub use flag::*;
pub use model::*;
```

**Step 5: Run tests**

Run: `cargo test -p metaschemars-model`
Expected: All tests pass

**Step 6: Commit**

```bash
git add -A
git commit -m "feat(model): Add DefineAssembly, Model, and instance types"
```

---

## Task 10: Define Module (Root Structure)

**Files:**
- Create: `crates/metaschemars-model/src/types/module.rs`
- Modify: `crates/metaschemars-model/src/types/mod.rs`

**Step 1: Create Module**

`crates/metaschemars-model/src/types/module.rs`:
```rust
use serde::{Deserialize, Serialize};
use crate::types::{Definition, Markup};

/// Import reference to another Metaschema module.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Import {
    pub href: String,
}

/// A Metaschema module - the root structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Module {
    pub schema_name: String,

    pub schema_version: String,

    pub short_name: String,

    pub namespace: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_base_uri: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<Markup>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub imports: Vec<Import>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub definitions: Vec<Definition>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_minimal() {
        let yaml = r#"
schema-name: Test Schema
schema-version: "1.0"
short-name: test
namespace: http://example.com/test
"#;
        let module: Module = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(module.schema_name, "Test Schema");
        assert_eq!(module.short_name, "test");
    }

    #[test]
    fn test_module_with_import() {
        let yaml = r#"
schema-name: Test Schema
schema-version: "1.0"
short-name: test
namespace: http://example.com/test
imports:
  - href: ./common.yaml
"#;
        let module: Module = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(module.imports.len(), 1);
        assert_eq!(module.imports[0].href, "./common.yaml");
    }

    #[test]
    fn test_module_with_definitions() {
        let yaml = r#"
schema-name: Test Schema
schema-version: "1.0"
short-name: test
namespace: http://example.com/test
definitions:
  - define-flag:
      name: id
      as-type: token
"#;
        let module: Module = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(module.definitions.len(), 1);
    }
}
```

**Step 2: Update types/mod.rs**

`crates/metaschemars-model/src/types/mod.rs`:
```rust
mod common;
mod constraints;
mod data_type;
mod definitions;
mod instances;
mod module;

pub use common::*;
pub use constraints::*;
pub use data_type::*;
pub use definitions::*;
pub use instances::*;
pub use module::*;
```

**Step 3: Run tests**

Run: `cargo test -p metaschemars-model`
Expected: All tests pass

**Step 4: Commit**

```bash
git add -A
git commit -m "feat(model): Add Module root structure with imports"
```

---

## Task 11: Implement YAML Parser

**Files:**
- Create: `crates/metaschemars-yaml/src/parser.rs`
- Modify: `crates/metaschemars-yaml/src/lib.rs`

**Step 1: Write parser with tests**

`crates/metaschemars-yaml/src/parser.rs`:
```rust
use std::fs;
use std::io::Read;
use std::path::Path;

use metaschemars_model::{Module, ParseError, ParseResult};

/// Parse a Metaschema module from a YAML file.
pub fn from_yaml_file<P: AsRef<Path>>(path: P) -> ParseResult<Module> {
    let path = path.as_ref();
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            return ParseResult::err(ParseError::IoError {
                path: path.display().to_string(),
                message: e.to_string(),
            });
        }
    };

    from_yaml_str(&content)
}

/// Parse a Metaschema module from a YAML string.
pub fn from_yaml_str(s: &str) -> ParseResult<Module> {
    match serde_yaml::from_str::<Module>(s) {
        Ok(module) => ParseResult::ok(module),
        Err(e) => ParseResult::err(ParseError::YamlSyntax {
            message: e.to_string(),
            span: None,
        }),
    }
}

/// Parse a Metaschema module from a reader.
pub fn from_yaml_reader<R: Read>(mut reader: R) -> ParseResult<Module> {
    let mut content = String::new();
    if let Err(e) = reader.read_to_string(&mut content) {
        return ParseResult::err(ParseError::IoError {
            path: "<reader>".to_string(),
            message: e.to_string(),
        });
    }
    from_yaml_str(&content)
}

/// Builder for parsing with options.
pub struct ModuleReader {
    error_limit: Option<usize>,
    source_name: Option<String>,
}

impl ModuleReader {
    pub fn new() -> Self {
        Self {
            error_limit: None,
            source_name: None,
        }
    }

    /// Stop collecting errors after this many.
    pub fn with_error_limit(mut self, limit: usize) -> Self {
        self.error_limit = Some(limit);
        self
    }

    /// Set source name for error messages.
    pub fn with_source_name(mut self, name: impl Into<String>) -> Self {
        self.source_name = Some(name.into());
        self
    }

    pub fn from_yaml_file<P: AsRef<Path>>(&self, path: P) -> ParseResult<Module> {
        from_yaml_file(path)
    }

    pub fn from_yaml_str(&self, s: &str) -> ParseResult<Module> {
        from_yaml_str(s)
    }

    pub fn from_yaml_reader<R: Read>(&self, reader: R) -> ParseResult<Module> {
        from_yaml_reader(reader)
    }
}

impl Default for ModuleReader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_yaml_str_valid() {
        let yaml = r#"
schema-name: Test Schema
schema-version: "1.0"
short-name: test
namespace: http://example.com/test
"#;
        let result = from_yaml_str(yaml);
        assert!(!result.has_errors());
        let module = result.value.unwrap();
        assert_eq!(module.schema_name, "Test Schema");
    }

    #[test]
    fn test_from_yaml_str_invalid() {
        let yaml = "invalid: [yaml: syntax";
        let result = from_yaml_str(yaml);
        assert!(result.has_errors());
    }

    #[test]
    fn test_from_yaml_file_not_found() {
        let result = from_yaml_file("/nonexistent/path.yaml");
        assert!(result.has_errors());
        match &result.errors[0] {
            ParseError::IoError { path, .. } => {
                assert!(path.contains("nonexistent"));
            }
            _ => panic!("Expected IoError"),
        }
    }

    #[test]
    fn test_module_reader_builder() {
        let yaml = r#"
schema-name: Test
schema-version: "1.0"
short-name: test
namespace: http://example.com/test
"#;
        let result = ModuleReader::new()
            .with_error_limit(5)
            .with_source_name("test-module")
            .from_yaml_str(yaml);
        assert!(!result.has_errors());
    }
}
```

**Step 2: Update lib.rs**

`crates/metaschemars-yaml/src/lib.rs`:
```rust
//! YAML parsing for Metaschema modules.

mod parser;

pub use parser::*;
```

**Step 3: Run tests**

Run: `cargo test -p metaschemars-yaml`
Expected: All tests pass

**Step 4: Commit**

```bash
git add -A
git commit -m "feat(yaml): Implement YAML parser with builder pattern"
```

---

## Task 12: Wire Up Facade Crate

**Files:**
- Modify: `crates/metaschemars/src/lib.rs`

**Step 1: Update facade to re-export everything**

`crates/metaschemars/src/lib.rs`:
```rust
//! Rust library for parsing Metaschema module definitions.
//!
//! # Example
//!
//! ```no_run
//! use metaschemars::from_yaml_file;
//!
//! let result = from_yaml_file("path/to/module.yaml");
//! if result.has_errors() {
//!     for err in &result.errors {
//!         eprintln!("{}", err);
//!     }
//! } else if let Some(module) = result.value {
//!     println!("Loaded module: {}", module.schema_name);
//! }
//! ```

// Re-export all model types
pub use metaschemars_model::*;

// Re-export YAML parsing
#[cfg(feature = "yaml")]
pub use metaschemars_yaml::{from_yaml_file, from_yaml_reader, from_yaml_str, ModuleReader};
```

**Step 2: Run all tests**

Run: `cargo test --workspace`
Expected: All tests pass

**Step 3: Commit**

```bash
git add -A
git commit -m "feat: Wire up facade crate with re-exports"
```

---

## Task 13: Add Integration Tests

**Files:**
- Create: `crates/metaschemars/tests/integration.rs`
- Create: `crates/metaschemars/tests/fixtures/simple-module.yaml`

**Step 1: Create test fixture**

`crates/metaschemars/tests/fixtures/simple-module.yaml`:
```yaml
schema-name: Simple Test Module
schema-version: "1.0.0"
short-name: simple
namespace: http://example.com/simple

definitions:
  - define-flag:
      name: id
      as-type: token
      formal-name: Identifier
      description: A unique identifier

  - define-field:
      name: title
      as-type: string
      formal-name: Title
      description: The title of an item

  - define-assembly:
      name: item
      root-name: item
      formal-name: Item
      description: A simple item
      flags:
        - ref: id
          required: yes
      model:
        - field:
            ref: title
            min-occurs: 1
            max-occurs: 1
```

**Step 2: Write integration tests**

`crates/metaschemars/tests/integration.rs`:
```rust
use metaschemars::{from_yaml_file, from_yaml_str, DataType, Definition, Module};

#[test]
fn test_parse_simple_module_from_file() {
    let result = from_yaml_file("tests/fixtures/simple-module.yaml");

    assert!(!result.has_errors(), "Unexpected errors: {:?}", result.errors);

    let module = result.value.expect("Expected module to be parsed");

    assert_eq!(module.schema_name, "Simple Test Module");
    assert_eq!(module.schema_version, "1.0.0");
    assert_eq!(module.short_name, "simple");
    assert_eq!(module.definitions.len(), 3);
}

#[test]
fn test_parse_module_with_all_definition_types() {
    let yaml = r#"
schema-name: Complete Test
schema-version: "1.0"
short-name: complete
namespace: http://example.com/complete

definitions:
  - define-flag:
      name: test-flag
      as-type: boolean

  - define-field:
      name: test-field
      as-type: markup-line

  - define-assembly:
      name: test-assembly
      root-name: root
"#;

    let result = from_yaml_str(yaml);
    assert!(!result.has_errors());

    let module = result.value.unwrap();
    assert_eq!(module.definitions.len(), 3);

    // Verify each definition type
    match &module.definitions[0] {
        Definition::DefineFlag(f) => assert_eq!(f.name, "test-flag"),
        _ => panic!("Expected DefineFlag"),
    }
    match &module.definitions[1] {
        Definition::DefineField(f) => assert_eq!(f.name, "test-field"),
        _ => panic!("Expected DefineField"),
    }
    match &module.definitions[2] {
        Definition::DefineAssembly(a) => assert_eq!(a.name, "test-assembly"),
        _ => panic!("Expected DefineAssembly"),
    }
}

#[test]
fn test_error_on_invalid_yaml() {
    let yaml = "this is not valid: [yaml";
    let result = from_yaml_str(yaml);

    assert!(result.has_errors());
    assert!(result.value.is_none());
}

#[test]
fn test_error_on_missing_required_field() {
    let yaml = r#"
schema-name: Missing Fields
short-name: missing
"#;
    // Missing schema-version and namespace
    let result = from_yaml_str(yaml);

    assert!(result.has_errors());
}
```

**Step 3: Run integration tests**

Run: `cargo test -p metaschemars --test integration`
Expected: All tests pass

**Step 4: Commit**

```bash
git add -A
git commit -m "test: Add integration tests with fixture"
```

---

## Task 14: Final Verification and Documentation

**Files:**
- Modify: Workspace `Cargo.toml` (add metadata)

**Step 1: Update workspace Cargo.toml with full metadata**

Add to workspace `Cargo.toml`:
```toml
[workspace.package]
version = "0.1.0"
edition = "2024"
license = "GPL-3.0"
repository = "https://github.com/aj-stein/metaschemars"
authors = ["AJ Stein"]
description = "Rust library for parsing Metaschema module definitions"
keywords = ["metaschema", "oscal", "schema", "parser"]
categories = ["parsing", "data-structures"]
```

**Step 2: Run full test suite**

Run: `cargo test --workspace`
Expected: All tests pass

**Step 3: Build release**

Run: `cargo build --release`
Expected: Successful build

**Step 4: Run clippy**

Run: `cargo clippy --workspace -- -D warnings`
Expected: No warnings

**Step 5: Final commit**

```bash
git add -A
git commit -m "chore: Add workspace metadata and verify build"
```

---

## Summary

This implementation plan creates a working Metaschema YAML parser in 14 tasks:

1. **Tasks 1-2**: Workspace setup and error types
2. **Tasks 3-7**: Data model types (DataType, common types, DefineFlag, constraints)
3. **Tasks 8-10**: Complex types (DefineField, DefineAssembly, Module)
4. **Tasks 11-12**: YAML parser and facade wiring
5. **Tasks 13-14**: Integration tests and final verification

Total: ~45 individual steps following TDD practices with commits after each task.
