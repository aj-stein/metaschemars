use metaschemars::{from_yaml_file, from_yaml_str, DataType, Definition};

#[test]
fn test_parse_simple_module_from_file() {
    let result = from_yaml_file("tests/fixtures/simple-module.yaml");

    assert!(!result.has_errors(), "Unexpected errors: {:?}", result.errors);

    let module = result.value.expect("Expected module to be parsed");

    assert_eq!(module.schema_name, "Simple Test Module");
    assert_eq!(module.schema_version, "1.0.0");
    assert_eq!(module.short_name, "simple");
    assert_eq!(module.definitions.len(), 3);
}

#[test]
fn test_parse_module_with_all_definition_types() {
    let yaml = r#"
schema-name: Complete Test
schema-version: "1.0"
short-name: complete
namespace: http://example.com/complete

definitions:
  - define-flag:
      name: test-flag
      as-type: boolean

  - define-field:
      name: test-field
      as-type: markup-line

  - define-assembly:
      name: test-assembly
      root-name: root
"#;

    let result = from_yaml_str(yaml);
    assert!(!result.has_errors());

    let module = result.value.unwrap();
    assert_eq!(module.definitions.len(), 3);

    // Verify each definition type
    match &module.definitions[0] {
        Definition::DefineFlag(f) => assert_eq!(f.name, "test-flag"),
        _ => panic!("Expected DefineFlag"),
    }
    match &module.definitions[1] {
        Definition::DefineField(f) => assert_eq!(f.name, "test-field"),
        _ => panic!("Expected DefineField"),
    }
    match &module.definitions[2] {
        Definition::DefineAssembly(a) => assert_eq!(a.name, "test-assembly"),
        _ => panic!("Expected DefineAssembly"),
    }
}

#[test]
fn test_parse_flag_with_constraints() {
    let yaml = r#"
schema-name: Constraints Test
schema-version: "1.0"
short-name: constraints
namespace: http://example.com/constraints

definitions:
  - define-flag:
      name: status
      as-type: token
      constraints:
        - allowed-values:
            allow-other: no
            enum:
              - value: active
                description: Currently active
              - value: inactive
                description: Not active
"#;

    let result = from_yaml_str(yaml);
    assert!(!result.has_errors(), "Errors: {:?}", result.errors);

    let module = result.value.unwrap();
    match &module.definitions[0] {
        Definition::DefineFlag(f) => {
            assert_eq!(f.name, "status");
            assert_eq!(f.constraints.len(), 1);
        }
        _ => panic!("Expected DefineFlag"),
    }
}

#[test]
fn test_error_on_invalid_yaml() {
    let yaml = "this is not valid: [yaml";
    let result = from_yaml_str(yaml);

    assert!(result.has_errors());
    assert!(result.value.is_none());
}

#[test]
fn test_error_on_missing_required_field() {
    let yaml = r#"
schema-name: Missing Fields
short-name: missing
"#;
    // Missing schema-version and namespace
    let result = from_yaml_str(yaml);

    assert!(result.has_errors());
}

#[test]
fn test_data_types_roundtrip() {
    let yaml = r#"
schema-name: DataTypes Test
schema-version: "1.0"
short-name: datatypes
namespace: http://example.com/datatypes

definitions:
  - define-flag:
      name: string-flag
      as-type: string

  - define-flag:
      name: integer-flag
      as-type: integer

  - define-flag:
      name: date-flag
      as-type: date-time-with-timezone

  - define-field:
      name: markup-field
      as-type: markup-multiline
"#;

    let result = from_yaml_str(yaml);
    assert!(!result.has_errors(), "Errors: {:?}", result.errors);

    let module = result.value.unwrap();
    assert_eq!(module.definitions.len(), 4);

    // Verify data types
    match &module.definitions[0] {
        Definition::DefineFlag(f) => assert_eq!(f.as_type, Some(DataType::String)),
        _ => panic!("Expected DefineFlag"),
    }
    match &module.definitions[1] {
        Definition::DefineFlag(f) => assert_eq!(f.as_type, Some(DataType::Integer)),
        _ => panic!("Expected DefineFlag"),
    }
    match &module.definitions[2] {
        Definition::DefineFlag(f) => assert_eq!(f.as_type, Some(DataType::DateTimeWithTimezone)),
        _ => panic!("Expected DefineFlag"),
    }
    match &module.definitions[3] {
        Definition::DefineField(f) => assert_eq!(f.as_type, Some(DataType::MarkupMultiline)),
        _ => panic!("Expected DefineField"),
    }
}
