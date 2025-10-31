use crate::error::RutenError;
use crate::interpreter::Value;
use std::collections::HashMap;

pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();

    // get image dimensions
    module.insert(
        "dimensions".to_string(),
        Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err(RutenError::RuntimeError(
                    "image.dimensions() requires file path".to_string(),
                ));
            }

            let path = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err(RutenError::TypeError("path must be a string".to_string())),
            };

            // placeholder - would use image crate in production
            println!("getting dimensions for: {}", path);
            
            let mut result = HashMap::new();
            result.insert("width".to_string(), Value::Number(1920.0));
            result.insert("height".to_string(), Value::Number(1080.0));
            
            Ok(Value::Dict(result))
        }),
    );

    // resize image
    module.insert(
        "resize".to_string(),
        Value::NativeFunction(|args| {
            if args.len() < 3 {
                return Err(RutenError::RuntimeError(
                    "image.resize() requires path, width, height".to_string(),
                ));
            }

            let path = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err(RutenError::TypeError("path must be a string".to_string())),
            };

            let width = match &args[1] {
                Value::Number(n) => *n as u32,
                _ => return Err(RutenError::TypeError("width must be a number".to_string())),
            };

            let height = match &args[2] {
                Value::Number(n) => *n as u32,
                _ => return Err(RutenError::TypeError("height must be a number".to_string())),
            };

            println!("resizing {} to {}x{}", path, width, height);
            Ok(Value::Bool(true))
        }),
    );

    module
}