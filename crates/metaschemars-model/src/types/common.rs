use serde::{Deserialize, Serialize};

/// Scope of a definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Scope {
    Local,
    Global,
}

impl Default for Scope {
    fn default() -> Self {
        Scope::Global
    }
}

/// Yes/No boolean alternative used in Metaschema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum YesNo {
    Yes,
    No,
}

impl From<YesNo> for bool {
    fn from(yn: YesNo) -> bool {
        matches!(yn, YesNo::Yes)
    }
}

impl From<bool> for YesNo {
    fn from(b: bool) -> YesNo {
        if b { YesNo::Yes } else { YesNo::No }
    }
}

/// Markup text content (simplified as String for now).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Markup(pub String);

impl From<String> for Markup {
    fn from(s: String) -> Self {
        Markup(s)
    }
}

impl From<&str> for Markup {
    fn from(s: &str) -> Self {
        Markup(s.to_string())
    }
}

/// A property with name, optional namespace, and value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    pub value: String,
}

/// Occurrence bounds for instances.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Occurs {
    Bounded(u32),
    #[serde(rename = "unbounded")]
    Unbounded,
}

/// Level for constraint severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Error,
    Warning,
    Informational,
}

impl Default for Level {
    fn default() -> Self {
        Level::Error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scope_deserialize() {
        let scope: Scope = serde_yaml::from_str("local").unwrap();
        assert_eq!(scope, Scope::Local);
    }

    #[test]
    fn test_yesno_to_bool() {
        assert!(bool::from(YesNo::Yes));
        assert!(!bool::from(YesNo::No));
    }

    #[test]
    fn test_markup_from_str() {
        let m = Markup::from("test");
        assert_eq!(m.0, "test");
    }

    #[test]
    fn test_property_deserialize() {
        let yaml = r#"
name: test-prop
value: test-value
"#;
        let prop: Property = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(prop.name, "test-prop");
        assert_eq!(prop.value, "test-value");
        assert_eq!(prop.namespace, None);
    }

    #[test]
    fn test_occurs_bounded() {
        let occurs: Occurs = serde_yaml::from_str("5").unwrap();
        assert_eq!(occurs, Occurs::Bounded(5));
    }

    #[test]
    fn test_level_default() {
        assert_eq!(Level::default(), Level::Error);
    }
}
