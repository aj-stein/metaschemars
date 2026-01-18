use serde::{Deserialize, Serialize};
use crate::types::{DefineFlag, Markup, YesNo};

/// A flag instance - either a reference or inline definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FlagInstance {
    Reference(FlagReference),
    Inline(DefineFlag),
}

/// Reference to a top-level define-flag.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct FlagReference {
    #[serde(rename = "ref")]
    pub ref_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<YesNo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<Markup>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flag_reference() {
        let yaml = r#"
ref: my-flag
required: yes
"#;
        let fi: FlagInstance = serde_yaml::from_str(yaml).unwrap();
        match fi {
            FlagInstance::Reference(r) => {
                assert_eq!(r.ref_name, "my-flag");
                assert_eq!(r.required, Some(YesNo::Yes));
            }
            _ => panic!("Expected reference"),
        }
    }
}
