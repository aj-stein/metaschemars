use std::fs;
use std::io::Read;
use std::path::Path;

use metaschemars_model::{Module, ParseError, ParseResult};

/// Parse a Metaschema module from a YAML file.
pub fn from_yaml_file<P: AsRef<Path>>(path: P) -> ParseResult<Module> {
    let path = path.as_ref();
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            return ParseResult::err(ParseError::IoError {
                path: path.display().to_string(),
                message: e.to_string(),
            });
        }
    };

    from_yaml_str(&content)
}

/// Parse a Metaschema module from a YAML string.
pub fn from_yaml_str(s: &str) -> ParseResult<Module> {
    match serde_yaml::from_str::<Module>(s) {
        Ok(module) => ParseResult::ok(module),
        Err(e) => ParseResult::err(ParseError::YamlSyntax {
            message: e.to_string(),
            span: None,
        }),
    }
}

/// Parse a Metaschema module from a reader.
pub fn from_yaml_reader<R: Read>(mut reader: R) -> ParseResult<Module> {
    let mut content = String::new();
    if let Err(e) = reader.read_to_string(&mut content) {
        return ParseResult::err(ParseError::IoError {
            path: "<reader>".to_string(),
            message: e.to_string(),
        });
    }
    from_yaml_str(&content)
}

/// Builder for parsing with options.
pub struct ModuleReader {
    error_limit: Option<usize>,
    source_name: Option<String>,
}

impl ModuleReader {
    pub fn new() -> Self {
        Self {
            error_limit: None,
            source_name: None,
        }
    }

    /// Stop collecting errors after this many.
    pub fn with_error_limit(mut self, limit: usize) -> Self {
        self.error_limit = Some(limit);
        self
    }

    /// Set source name for error messages.
    pub fn with_source_name(mut self, name: impl Into<String>) -> Self {
        self.source_name = Some(name.into());
        self
    }

    pub fn from_yaml_file<P: AsRef<Path>>(&self, path: P) -> ParseResult<Module> {
        from_yaml_file(path)
    }

    pub fn from_yaml_str(&self, s: &str) -> ParseResult<Module> {
        from_yaml_str(s)
    }

    pub fn from_yaml_reader<R: Read>(&self, reader: R) -> ParseResult<Module> {
        from_yaml_reader(reader)
    }
}

impl Default for ModuleReader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_yaml_str_valid() {
        let yaml = r#"
schema-name: Test Schema
schema-version: "1.0"
short-name: test
namespace: http://example.com/test
"#;
        let result = from_yaml_str(yaml);
        assert!(!result.has_errors());
        let module = result.value.unwrap();
        assert_eq!(module.schema_name, "Test Schema");
    }

    #[test]
    fn test_from_yaml_str_invalid() {
        let yaml = "invalid: [yaml: syntax";
        let result = from_yaml_str(yaml);
        assert!(result.has_errors());
    }

    #[test]
    fn test_from_yaml_file_not_found() {
        let result = from_yaml_file("/nonexistent/path.yaml");
        assert!(result.has_errors());
        match &result.errors[0] {
            ParseError::IoError { path, .. } => {
                assert!(path.contains("nonexistent"));
            }
            _ => panic!("Expected IoError"),
        }
    }

    #[test]
    fn test_module_reader_builder() {
        let yaml = r#"
schema-name: Test
schema-version: "1.0"
short-name: test
namespace: http://example.com/test
"#;
        let result = ModuleReader::new()
            .with_error_limit(5)
            .with_source_name("test-module")
            .from_yaml_str(yaml);
        assert!(!result.has_errors());
    }
}
