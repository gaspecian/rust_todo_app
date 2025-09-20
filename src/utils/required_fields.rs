use serde::{Deserialize, Serialize};
use serde_json::Value;

pub fn validate_required_fields<T, R>(data: &T, required_fields: Vec<&str>) -> Result<R, String>
where
    T: Serialize,
    R: for<'de> Deserialize<'de>,
{
    let json_value =
        serde_json::to_value(data).map_err(|e| format!("Failed to serialize data: {e}"))?;

    for field in required_fields {
        match json_value.get(field) {
            Some(Value::String(s)) if s.trim().is_empty() => return Err(field.to_string()),
            Some(Value::Null) | None => return Err(field.to_string()),
            Some(Value::Number(n)) if n.as_i64() == Some(0) => return Err(field.to_string()),
            _ => {}
        }
    }

    serde_json::from_value(json_value).map_err(|e| format!("Failed to deserialize data: {e}"))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::manual_string_new)]
mod tests {
    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    struct TestData {
        name: Option<String>,
        email: Option<String>,
        age: Option<i32>,
        active: Option<bool>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct ValidatedTestData {
        name: String,
        email: String,
        age: i32,
        active: bool,
    }

    #[test]
    fn test_validate_required_fields_success() {
        let data = TestData {
            name: Some("John".to_string()),
            email: Some("john@example.com".to_string()),
            age: Some(25),
            active: Some(true),
        };

        let result: Result<ValidatedTestData, String> =
            validate_required_fields(&data, vec!["name", "email"]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_required_fields_missing_field() {
        let data = TestData {
            name: Some("John".to_string()),
            email: None,
            age: Some(25),
            active: Some(true),
        };

        let result: Result<ValidatedTestData, String> =
            validate_required_fields(&data, vec!["name", "email"]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "email");
    }

    #[test]
    fn test_validate_required_fields_empty_string() {
        let data = TestData {
            name: Some("".to_string()),
            email: Some("john@example.com".to_string()),
            age: Some(25),
            active: Some(true),
        };

        let result: Result<ValidatedTestData, String> =
            validate_required_fields(&data, vec!["name", "email"]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "name");
    }

    #[test]
    fn test_validate_required_fields_whitespace_string() {
        let data = TestData {
            name: Some("   ".to_string()),
            email: Some("john@example.com".to_string()),
            age: Some(25),
            active: Some(true),
        };

        let result: Result<ValidatedTestData, String> =
            validate_required_fields(&data, vec!["name", "email"]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "name");
    }

    #[test]
    fn test_validate_required_fields_zero_number() {
        let data = TestData {
            name: Some("John".to_string()),
            email: Some("john@example.com".to_string()),
            age: Some(0),
            active: Some(true),
        };

        let result: Result<ValidatedTestData, String> =
            validate_required_fields(&data, vec!["name", "email", "age"]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "age");
    }

    #[test]
    fn test_validate_required_fields_no_required_fields() {
        let data = TestData {
            name: Some("John".to_string()),
            email: Some("john@example.com".to_string()),
            age: Some(25),
            active: Some(true),
        };

        let result: Result<ValidatedTestData, String> = validate_required_fields(&data, vec![]);
        assert!(result.is_ok());
    }
}
