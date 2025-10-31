use crate::error::RutenError;
use crate::interpreter::Value;
use std::collections::HashMap;
use serde_json;

pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();

    module.insert(
        "parse".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("parse() takes 1 argument".to_string()));
            }
            match &args[0] {
                Value::String(json_str) => {
                    let json_value: serde_json::Value = serde_json::from_str(json_str)
                        .map_err(|e| RutenError::RuntimeError(format!("json parse error: {}", e)))?;
                    json_to_value(&json_value)
                }
                _ => Err(RutenError::TypeError("parse() requires a string".to_string())),
            }
        }),
    );

    module.insert(
        "stringify".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("stringify() takes 1 argument".to_string()));
            }
            let json_value = value_to_json(&args[0])?;
            let json_str = serde_json::to_string(&json_value)
                .map_err(|e| RutenError::RuntimeError(format!("json stringify error: {}", e)))?;
            Ok(Value::String(json_str))
        }),
    );

    module.insert(
        "pretty".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("pretty() takes 1 argument".to_string()));
            }
            let json_value = value_to_json(&args[0])?;
            let json_str = serde_json::to_string_pretty(&json_value)
                .map_err(|e| RutenError::RuntimeError(format!("json pretty error: {}", e)))?;
            Ok(Value::String(json_str))
        }),
    );

    module
}

fn json_to_value(json: &serde_json::Value) -> Result<Value, RutenError> {
    match json {
        serde_json::Value::Null => Ok(Value::None),
        serde_json::Value::Bool(b) => Ok(Value::Bool(*b)),
        serde_json::Value::Number(n) => {
            if let Some(f) = n.as_f64() {
                Ok(Value::Number(f))
            } else {
                Err(RutenError::RuntimeError("invalid json number".to_string()))
            }
        }
        serde_json::Value::String(s) => Ok(Value::String(s.clone())),
        serde_json::Value::Array(arr) => {
            let values: Result<Vec<_>, _> = arr.iter().map(json_to_value).collect();
            Ok(Value::List(values?))
        }
        serde_json::Value::Object(obj) => {
            let mut map = HashMap::new();
            for (key, val) in obj {
                map.insert(key.clone(), json_to_value(val)?);
            }
            Ok(Value::Dict(map))
        }
    }
}

fn value_to_json(value: &Value) -> Result<serde_json::Value, RutenError> {
    match value {
        Value::None => Ok(serde_json::Value::Null),
        Value::Bool(b) => Ok(serde_json::Value::Bool(*b)),
        Value::Number(n) => {
            serde_json::Number::from_f64(*n)
                .map(serde_json::Value::Number)
                .ok_or_else(|| RutenError::RuntimeError("invalid number for json".to_string()))
        }
        Value::String(s) => Ok(serde_json::Value::String(s.clone())),
        Value::List(items) => {
            let json_items: Result<Vec<_>, _> = items.iter().map(value_to_json).collect();
            Ok(serde_json::Value::Array(json_items?))
        }
        Value::Dict(map) => {
            let mut json_obj = serde_json::Map::new();
            for (key, val) in map {
                json_obj.insert(key.clone(), value_to_json(val)?);
            }
            Ok(serde_json::Value::Object(json_obj))
        }
        _ => Err(RutenError::TypeError("cannot convert to json".to_string())),
    }
}
