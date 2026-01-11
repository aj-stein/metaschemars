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
