use crate::error::RutenError;
use crate::interpreter::Value;
use std::collections::HashMap;

pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();

    module.insert(
        "load_image".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("load_image() takes 1 argument".to_string()));
            }
            
            match &args[0] {
                Value::String(path) => {
                    // placeholder - would use image processing library
                    Err(RutenError::RuntimeError(
                        format!("vision.load_image() not yet implemented for: {}", path)
                    ))
                }
                _ => Err(RutenError::TypeError("load_image() requires a string path".to_string())),
            }
        }),
    );

    module.insert(
        "detect_faces".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("detect_faces() takes 1 argument".to_string()));
            }
            
            Err(RutenError::RuntimeError("vision.detect_faces() not yet implemented".to_string()))
        }),
    );

    module.insert(
        "resize".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 3 {
                return Err(RutenError::RuntimeError("resize() takes 3 arguments: image, width, height".to_string()));
            }
            
            Err(RutenError::RuntimeError("vision.resize() not yet implemented".to_string()))
        }),
    );

    module
}