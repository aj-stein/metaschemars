# metaschemars Design Document

## Overview

**metaschemars** is a Rust library for parsing Metaschema module definitions into strongly-typed Rust structures. It supports YAML, XML, and JSON formats through a modular crate architecture.

**Goal:** Parse Metaschema modules (per framework.metaschema.dev spec) into Rust structs for downstream tooling.

**Scope:** Read-only parsing with error accumulation. No semantic validation or code generation.

---

## Architecture

### Crate Structure

```
metaschemars/
├── Cargo.toml              # Workspace root
├── crates/
│   ├── metaschemars-model/ # Core data structures
│   ├── metaschemars-yaml/  # YAML format support
│   ├── metaschemars-xml/   # XML format (future)
│   ├── metaschemars-json/  # JSON format (future)
│   └── metaschemars/       # Facade crate
```

### Dependency Graph

```
metaschemars (facade)
    ├── metaschemars-model
    ├── metaschemars-yaml
    ├── metaschemars-xml (optional feature)
    └── metaschemars-json (optional feature)

metaschemars-yaml
    └── metaschemars-model

metaschemars-xml
    └── metaschemars-model

metaschemars-json
    └── metaschemars-model
```

### Key Dependencies

- `serde` + `serde_derive` - Serialization framework
- `serde_yaml` - YAML parsing (metaschemars-yaml)
- `thiserror` - Error type definitions
- `miette` - Rich error diagnostics with source spans

---

## Data Model (metaschemars-model)

### Module Root

```rust
pub struct Module {
    pub schema_name: String,
    pub schema_version: String,
    pub short_name: String,
    pub namespace: String,
    pub json_base_uri: Option<String>,
    pub remarks: Option<Markup>,
    pub imports: Vec<Import>,
    pub definitions: Vec<Definition>,
}

pub struct Import {
    pub href: String,
}
```

### Definitions

```rust
pub enum Definition {
    Flag(DefineFlag),
    Field(DefineField),
    Assembly(DefineAssembly),
}

pub struct DefineFlag {
    pub name: String,
    pub as_type: Option<DataType>,
    pub default: Option<String>,
    pub scope: Option<Scope>,
    pub deprecated: Option<String>,
    pub formal_name: Option<String>,
    pub description: Option<Markup>,
    pub remarks: Option<Markup>,
    pub constraints: Vec<Constraint>,
    pub props: Vec<Property>,
    pub use_name: Option<String>,
}

pub struct DefineField {
    pub name: String,
    pub as_type: Option<DataType>,
    pub default: Option<String>,
    pub collapsible: Option<YesNo>,
    pub scope: Option<Scope>,
    pub deprecated: Option<String>,
    pub formal_name: Option<String>,
    pub description: Option<Markup>,
    pub remarks: Option<Markup>,
    pub flags: Vec<FlagInstance>,
    pub constraints: Vec<Constraint>,
    pub props: Vec<Property>,
    pub json_key: Option<JsonKey>,
    pub json_value_key: Option<String>,
    pub use_name: Option<String>,
}

pub struct DefineAssembly {
    pub name: String,
    pub scope: Option<Scope>,
    pub deprecated: Option<String>,
    pub formal_name: Option<String>,
    pub description: Option<Markup>,
    pub remarks: Option<Markup>,
    pub root_name: Option<String>,
    pub flags: Vec<FlagInstance>,
    pub model: Option<Model>,
    pub constraints: Vec<Constraint>,
    pub props: Vec<Property>,
    pub json_key: Option<JsonKey>,
    pub use_name: Option<String>,
}
```

### Supporting Types

```rust
pub enum Scope { Local, Global }
pub enum YesNo { Yes, No }

pub struct Markup(pub String);  // Simplified for now

pub struct Property {
    pub name: String,
    pub namespace: Option<String>,
    pub value: String,
}
```

---

## Instances and Model Structure

### Flag Instances

```rust
pub enum FlagInstance {
    /// Reference to a top-level define-flag
    Reference(FlagReference),
    /// Inline flag definition
    Inline(DefineFlag),
}

pub struct FlagReference {
    pub ref_name: String,
    pub required: Option<YesNo>,
    pub use_name: Option<String>,
    pub remarks: Option<Markup>,
}
```

### Assembly Model

```rust
pub struct Model {
    pub instances: Vec<ModelInstance>,
}

pub enum ModelInstance {
    Field(FieldInstance),
    Assembly(AssemblyInstance),
    Choice(Choice),
    Any(Any),
}

pub struct FieldInstance {
    pub ref_name: Option<String>,      // Reference to define-field
    pub inline: Option<DefineField>,   // Or inline definition
    pub min_occurs: Option<u32>,
    pub max_occurs: Option<Occurs>,
    pub group_as: Option<GroupAs>,
    pub use_name: Option<String>,
    pub remarks: Option<Markup>,
}

pub struct AssemblyInstance {
    pub ref_name: Option<String>,
    pub inline: Option<DefineAssembly>,
    pub min_occurs: Option<u32>,
    pub max_occurs: Option<Occurs>,
    pub group_as: Option<GroupAs>,
    pub use_name: Option<String>,
    pub remarks: Option<Markup>,
}

pub enum Occurs {
    Bounded(u32),
    Unbounded,
}

pub struct GroupAs {
    pub name: String,
    pub in_json: Option<GroupInJson>,
    pub in_xml: Option<GroupInXml>,
}

pub enum GroupInJson { Array, SingletonOrArray, ByKey }
pub enum GroupInXml { Ungrouped, Grouped }

pub struct Choice {
    pub instances: Vec<ModelInstance>,
}

pub struct Any {
    pub process_contents: Option<ProcessContents>,
}

pub enum ProcessContents { Strict, Lax, None }

pub struct JsonKey {
    pub flag_ref: String,
}
```

---

## Data Types

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataType {
    // Character-based
    String,
    Token,
    EmailAddress,
    Hostname,
    IpV4Address,
    IpV6Address,
    Uri,
    UriReference,
    Uuid,
    QName,

    // Numeric
    Decimal,
    Integer,
    NonNegativeInteger,
    PositiveInteger,

    // Boolean/Binary
    Boolean,
    Base64,

    // Temporal
    Date,
    DateWithTimezone,
    DateTime,
    DateTimeWithTimezone,
    DayTimeDuration,
    YearMonthDuration,

    // Markup
    MarkupLine,
    MarkupMultiline,
}
```

---

## Constraints

```rust
pub enum Constraint {
    AllowedValues(AllowedValues),
    Expect(Expect),
    Report(Report),
    Matches(Matches),
    HasCardinality(HasCardinality),
    Index(Index),
    IndexHasKey(IndexHasKey),
    IsUnique(IsUnique),
    Let(Let),
}

pub struct AllowedValues {
    pub id: Option<String>,
    pub level: Option<Level>,
    pub target: Option<String>,
    pub allow_other: Option<YesNo>,
    pub extensible: Option<YesNo>,
    pub enums: Vec<EnumValue>,
    pub remarks: Option<Markup>,
}

pub struct EnumValue {
    pub value: String,
    pub deprecated: Option<String>,
    pub description: Option<Markup>,
}

pub struct Expect {
    pub id: Option<String>,
    pub level: Option<Level>,
    pub target: Option<String>,
    pub test: String,  // Metapath expression
    pub message: Option<String>,
    pub remarks: Option<Markup>,
}

pub struct Matches {
    pub id: Option<String>,
    pub level: Option<Level>,
    pub target: Option<String>,
    pub datatype: Option<DataType>,
    pub regex: Option<String>,
    pub remarks: Option<Markup>,
}

pub struct HasCardinality {
    pub id: Option<String>,
    pub level: Option<Level>,
    pub target: String,
    pub min_occurs: Option<u32>,
    pub max_occurs: Option<Occurs>,
    pub remarks: Option<Markup>,
}

pub enum Level { Error, Warning, Informational }
```

The remaining constraint types (`Report`, `Index`, `IndexHasKey`, `IsUnique`, `Let`) follow similar patterns.

---

## Error Handling

### Error Types

```rust
use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum ParseError {
    #[error("Failed to read file: {path}")]
    IoError {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Invalid YAML syntax")]
    #[diagnostic(code(metaschemars::yaml::syntax))]
    YamlSyntax {
        #[label("here")]
        span: Option<SourceSpan>,
        #[source_code]
        src: String,
        #[source]
        source: serde_yaml::Error,
    },

    #[error("Unknown data type: {value}")]
    #[diagnostic(code(metaschemars::unknown_datatype))]
    UnknownDataType {
        value: String,
        #[label("this data type")]
        span: Option<SourceSpan>,
    },

    #[error("Missing required field: {field}")]
    #[diagnostic(code(metaschemars::missing_field))]
    MissingField {
        field: String,
        #[label("in this definition")]
        span: Option<SourceSpan>,
    },
}

pub use miette::SourceSpan;
```

### Error Collection

```rust
#[derive(Debug)]
pub struct ParseResult<T> {
    pub value: Option<T>,
    pub errors: Vec<ParseError>,
}

impl<T> ParseResult<T> {
    pub fn ok(value: T) -> Self {
        Self { value: Some(value), errors: vec![] }
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn into_result(self) -> Result<T, Vec<ParseError>> {
        if self.errors.is_empty() {
            Ok(self.value.unwrap())
        } else {
            Err(self.errors)
        }
    }
}
```

---

## Parsing API

### Simple Functions

```rust
// From file
pub fn from_yaml_file<P: AsRef<Path>>(path: P) -> ParseResult<Module> { ... }
pub fn from_xml_file<P: AsRef<Path>>(path: P) -> ParseResult<Module> { ... }
pub fn from_json_file<P: AsRef<Path>>(path: P) -> ParseResult<Module> { ... }

// From string
pub fn from_yaml_str(s: &str) -> ParseResult<Module> { ... }
pub fn from_xml_str(s: &str) -> ParseResult<Module> { ... }
pub fn from_json_str(s: &str) -> ParseResult<Module> { ... }

// From reader
pub fn from_yaml_reader<R: Read>(reader: R) -> ParseResult<Module> { ... }
```

### Builder Pattern

```rust
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

    pub fn with_error_limit(mut self, limit: usize) -> Self {
        self.error_limit = Some(limit);
        self
    }

    pub fn with_source_name(mut self, name: impl Into<String>) -> Self {
        self.source_name = Some(name.into());
        self
    }

    pub fn from_yaml_file<P: AsRef<Path>>(&self, path: P) -> ParseResult<Module> { ... }
    pub fn from_yaml_str(&self, s: &str) -> ParseResult<Module> { ... }
    pub fn from_yaml_reader<R: Read>(&self, reader: R) -> ParseResult<Module> { ... }
}
```

### Usage Examples

```rust
use metaschemars::{from_yaml_file, ModuleReader};

// Simple usage
let result = from_yaml_file("oscal-catalog.yaml");
if result.has_errors() {
    for err in &result.errors {
        eprintln!("{:?}", miette::Report::new(err.clone()));
    }
}
let module = result.value.unwrap();

// With builder
let result = ModuleReader::new()
    .with_error_limit(10)
    .with_source_name("inline-module")
    .from_yaml_str(&yaml_content);
```

---

## Initial Scope

### What We're Building Now

1. **metaschemars-model** crate:
   - All Rust structs for Module, Definitions, Instances, Constraints
   - DataType enum
   - Error types with miette integration
   - ParseResult for error accumulation

2. **metaschemars-yaml** crate:
   - YAML deserialization using serde_yaml
   - Source span tracking for error locations
   - Simple functions + builder API

3. **metaschemars** facade crate:
   - Re-exports from model and yaml crates
   - Feature flags for future format crates

### Deferred to Later

- **metaschemars-xml** - XML parsing (quick-xml + serde)
- **metaschemars-json** - JSON parsing (serde_json)
- **Module resolution** - Following imports to load referenced modules
- **Semantic validation** - Checking references resolve, types are valid
- **Serialization** - Writing modules back to disk

---

## Testing Strategy

- Unit tests for each data structure's deserialization
- Integration tests with real Metaschema module examples from framework.metaschema.dev
- Error case tests ensuring we collect multiple errors properly

---

## Dependencies Summary

```toml
# metaschemars-model
serde = { version = "1", features = ["derive"] }
thiserror = "2"
miette = { version = "7", features = ["fancy"] }

# metaschemars-yaml
metaschemars-model = { path = "../metaschemars-model" }
serde_yaml = "0.9"
```
