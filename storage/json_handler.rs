// storage/json_handler.rs
// IWS v1.0 - JSON Handler
// Menangani serialisasi dan deserialisasi data ke/dari JSON

use serde::{Serialize, Deserialize};
use serde_json::{Value, from_str, from_slice, to_string, to_string_pretty, to_vec};

#[derive(Debug, Clone)]
pub struct JsonHandler;

impl JsonHandler {
    pub fn to_json<T: Serialize>(data: &T) -> Result<String, String> {
        to_string(data).map_err(|e| format!("Serialize: {}", e))
    }

    pub fn to_json_pretty<T: Serialize>(data: &T) -> Result<String, String> {
        to_string_pretty(data).map_err(|e| format!("Serialize: {}", e))
    }

    pub fn to_bytes<T: Serialize>(data: &T) -> Result<Vec<u8>, String> {
        to_vec(data).map_err(|e| format!("Serialize: {}", e))
    }

    pub fn from_json<T: for<'de> Deserialize<'de>>(json_str: &str) -> Result<T, String> {
        from_str(json_str).map_err(|e| format!("Deserialize: {}", e))
    }

    pub fn from_slice<T: for<'de> Deserialize<'de>>(data: &[u8]) -> Result<T, String> {
        from_slice(data).map_err(|e| format!("Deserialize: {}", e))
    }

    pub fn validate(data: &str) -> Result<Value, String> {
        from_str::<Value>(data).map_err(|e| format!("Invalid JSON: {}", e))
    }

    pub fn pretty_print(data: &str) -> Result<String, String> {
        let parsed: Value = from_str(data).map_err(|e| format!("Parse: {}", e))?;
        to_string_pretty(&parsed).map_err(|e| format!("Format: {}", e))
    }

    pub fn merge(a: &Value, b: &Value) -> Value {
        match (a, b) {
            (Value::Object(map_a), Value::Object(map_b)) => {
                let mut merged = map_a.clone();
                for (k, v) in map_b {
                    merged.insert(k.clone(), v.clone());
                }
                Value::Object(merged)
            }
            _ => b.clone(),
        }
    }

    pub fn get_nested<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = value;
        for part in parts {
            match current {
                Value::Object(map) => current = map.get(part)?,
                _ => return None,
            }
        }
        Some(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_from_json() {
        let data = vec!["a", "b", "c"];
        let json = JsonHandler::to_json(&data).unwrap();
        let parsed: Vec<String> = JsonHandler::from_json(&json).unwrap();
        assert_eq!(parsed, data);
    }

    #[test]
    fn test_pretty_print() {
        let ugly = r#"{"a":1,"b":2}"#;
        let pretty = JsonHandler::pretty_print(ugly).unwrap();
        assert!(pretty.contains('\n'));
    }

    #[test]
    fn test_validate_invalid() {
        assert!(JsonHandler::validate("not json").is_err());
        assert!(JsonHandler::validate(r#"{"valid": true}"#).is_ok());
    }

    #[test]
    fn test_merge() {
        let a: Value = serde_json::json!({"x": 1, "y": 2});
        let b: Value = serde_json::json!({"y": 99, "z": 3});
        let merged = JsonHandler::merge(&a, &b);
        assert_eq!(merged["x"], 1);
        assert_eq!(merged["y"], 99);
        assert_eq!(merged["z"], 3);
    }

    #[test]
    fn test_get_nested() {
        let value: Value = serde_json::json!({"a": {"b": {"c": "found"}}});
        assert_eq!(JsonHandler::get_nested(&value, "a.b.c").unwrap(), "found");
        assert!(JsonHandler::get_nested(&value, "a.x.y").is_none());
    }
}
