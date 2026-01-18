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
