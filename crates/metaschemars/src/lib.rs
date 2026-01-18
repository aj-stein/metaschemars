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
