use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug, Clone, PartialEq)]
pub enum ParseError {
    #[error("Failed to read file: {path}")]
    IoError {
        path: String,
        message: String,
    },

    #[error("Invalid YAML syntax: {message}")]
    #[diagnostic(code(metaschemars::yaml::syntax))]
    YamlSyntax {
        message: String,
        #[label("here")]
        span: Option<SourceSpan>,
    },

    #[error("Unknown data type: {value}")]
    #[diagnostic(code(metaschemars::unknown_datatype))]
    UnknownDataType {
        value: String,
    },

    #[error("Missing required field: {field}")]
    #[diagnostic(code(metaschemars::missing_field))]
    MissingField {
        field: String,
    },
}

/// Result type that can hold a value and accumulated errors.
#[derive(Debug)]
pub struct ParseResult<T> {
    pub value: Option<T>,
    pub errors: Vec<ParseError>,
}

impl<T> ParseResult<T> {
    pub fn ok(value: T) -> Self {
        Self {
            value: Some(value),
            errors: vec![],
        }
    }

    pub fn err(error: ParseError) -> Self {
        Self {
            value: None,
            errors: vec![error],
        }
    }

    pub fn with_errors(value: T, errors: Vec<ParseError>) -> Self {
        Self {
            value: Some(value),
            errors,
        }
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn into_result(self) -> Result<T, Vec<ParseError>> {
        if self.errors.is_empty() {
            self.value.ok_or_else(|| vec![])
        } else {
            Err(self.errors)
        }
    }
}

impl<T> Default for ParseResult<T> {
    fn default() -> Self {
        Self {
            value: None,
            errors: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_result_ok() {
        let result = ParseResult::ok(42);
        assert!(!result.has_errors());
        assert_eq!(result.value, Some(42));
    }

    #[test]
    fn test_parse_result_err() {
        let result: ParseResult<i32> = ParseResult::err(ParseError::MissingField {
            field: "name".to_string(),
        });
        assert!(result.has_errors());
        assert_eq!(result.value, None);
    }

    #[test]
    fn test_parse_result_into_result_ok() {
        let result = ParseResult::ok(42);
        assert_eq!(result.into_result(), Ok(42));
    }

    #[test]
    fn test_parse_result_into_result_err() {
        let result: ParseResult<i32> = ParseResult::err(ParseError::MissingField {
            field: "name".to_string(),
        });
        let err = result.into_result().unwrap_err();
        assert_eq!(err.len(), 1);
    }
}
