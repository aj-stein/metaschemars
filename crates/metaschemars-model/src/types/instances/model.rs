use serde::{Deserialize, Serialize};
use crate::types::{Markup, Occurs};

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
    #[serde(default)]
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
