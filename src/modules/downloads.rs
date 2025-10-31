use crate::error::RutenError;
use crate::interpreter::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use reqwest::blocking;

pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();

    module.insert(
        "download".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 2 {
                return Err(RutenError::RuntimeError("download() takes 2 arguments".to_string()));
            }
            match (&args[0], &args[1]) {
                (Value::String(url), Value::String(filepath)) => {
                    let response = blocking::get(url)
                        .map_err(|e| RutenError::RuntimeError(format!("download error: {}", e)))?;
                    
                    let bytes = response
                        .bytes()
                        .map_err(|e| RutenError::RuntimeError(format!("read error: {}", e)))?;
                    
                    fs::write(filepath, bytes)
                        .map_err(|e| RutenError::RuntimeError(format!("write error: {}", e)))?;
                    
                    Ok(Value::Bool(true))
                }
                _ => Err(RutenError::TypeError("download() requires two strings".to_string())),
            }
        }),
    );

    module.insert(
        "exists".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("exists() takes 1 argument".to_string()));
            }
            match &args[0] {
                Value::String(filepath) => {
                    Ok(Value::Bool(Path::new(filepath).exists()))
                }
                _ => Err(RutenError::TypeError("exists() requires a string".to_string())),
            }
        }),
    );

    module.insert(
        "read_file".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("read_file() takes 1 argument".to_string()));
            }
            match &args[0] {
                Value::String(filepath) => {
                    let content = fs::read_to_string(filepath)
                        .map_err(|e| RutenError::RuntimeError(format!("read error: {}", e)))?;
                    Ok(Value::String(content))
                }
                _ => Err(RutenError::TypeError("read_file() requires a string".to_string())),
            }
        }),
    );

    module.insert(
        "write_file".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 2 {
                return Err(RutenError::RuntimeError("write_file() takes 2 arguments".to_string()));
            }
            match (&args[0], &args[1]) {
                (Value::String(filepath), Value::String(content)) => {
                    fs::write(filepath, content)
                        .map_err(|e| RutenError::RuntimeError(format!("write error: {}", e)))?;
                    Ok(Value::Bool(true))
                }
                _ => Err(RutenError::TypeError("write_file() requires two strings".to_string())),
            }
        }),
    );

    module
}