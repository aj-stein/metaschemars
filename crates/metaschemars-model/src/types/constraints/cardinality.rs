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
