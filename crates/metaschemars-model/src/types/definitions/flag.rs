use serde::{Deserialize, Serialize};
use crate::types::{Constraint, DataType, Markup, Property, Scope};

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

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub constraints: Vec<Constraint>,
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

    #[test]
    fn test_define_flag_with_constraint() {
        let yaml = r#"
name: status
as-type: token
constraints:
  - !allowed-values
    allow-other: no
    enum:
      - value: active
      - value: inactive
"#;
        let flag: DefineFlag = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(flag.constraints.len(), 1);
    }
}
