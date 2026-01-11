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
