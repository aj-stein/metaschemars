use serde::{Deserialize, Serialize};
use crate::types::{Level, Markup};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Expect {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<Level>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    pub test: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<Markup>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expect() {
        let yaml = r#"
target: "."
test: "@min le @max"
message: Minimum must be less than maximum
"#;
        let exp: Expect = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(exp.test, "@min le @max");
        assert!(exp.message.is_some());
    }
}
