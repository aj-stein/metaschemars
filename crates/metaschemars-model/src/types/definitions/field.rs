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
