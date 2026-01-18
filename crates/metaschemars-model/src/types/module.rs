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
