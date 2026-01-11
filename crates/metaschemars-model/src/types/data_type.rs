use serde::{Deserialize, Serialize};

/// All data types supported by Metaschema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DataType {
    // Character-based
    String,
    Token,
    #[serde(rename = "email-address")]
    EmailAddress,
    Hostname,
    #[serde(rename = "ip-v4-address")]
    IpV4Address,
    #[serde(rename = "ip-v6-address")]
    IpV6Address,
    Uri,
    #[serde(rename = "uri-reference")]
    UriReference,
    Uuid,
    #[serde(rename = "qname")]
    QName,

    // Numeric
    Decimal,
    Integer,
    #[serde(rename = "non-negative-integer")]
    NonNegativeInteger,
    #[serde(rename = "positive-integer")]
    PositiveInteger,

    // Boolean/Binary
    Boolean,
    Base64,

    // Temporal
    Date,
    #[serde(rename = "date-with-timezone")]
    DateWithTimezone,
    #[serde(rename = "date-time")]
    DateTime,
    #[serde(rename = "date-time-with-timezone")]
    DateTimeWithTimezone,
    #[serde(rename = "day-time-duration")]
    DayTimeDuration,
    #[serde(rename = "year-month-duration")]
    YearMonthDuration,

    // Markup
    #[serde(rename = "markup-line")]
    MarkupLine,
    #[serde(rename = "markup-multiline")]
    MarkupMultiline,
}

impl Default for DataType {
    fn default() -> Self {
        DataType::String
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_string() {
        let dt: DataType = serde_yaml::from_str("string").unwrap();
        assert_eq!(dt, DataType::String);
    }

    #[test]
    fn test_deserialize_kebab_case() {
        let dt: DataType = serde_yaml::from_str("non-negative-integer").unwrap();
        assert_eq!(dt, DataType::NonNegativeInteger);
    }

    #[test]
    fn test_deserialize_markup() {
        let dt: DataType = serde_yaml::from_str("markup-multiline").unwrap();
        assert_eq!(dt, DataType::MarkupMultiline);
    }

    #[test]
    fn test_serialize_roundtrip() {
        let dt = DataType::DateTimeWithTimezone;
        let yaml = serde_yaml::to_string(&dt).unwrap();
        let parsed: DataType = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(parsed, dt);
    }
}
