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
