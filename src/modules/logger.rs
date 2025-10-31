use crate::interpreter::Value;
use std::collections::HashMap;

pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();

    // info log
    module.insert(
        "info".to_string(),
        Value::NativeFunction(|args| {
            let message = args
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            println!("[INFO] {}", message);
            Ok(Value::None)
        }),
    );

    // warning log
    module.insert(
        "warn".to_string(),
        Value::NativeFunction(|args| {
            let message = args
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            println!("[WARN] {}", message);
            Ok(Value::None)
        }),
    );

    // error log
    module.insert(
        "error".to_string(),
        Value::NativeFunction(|args| {
            let message = args
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            eprintln!("[ERROR] {}", message);
            Ok(Value::None)
        }),
    );

    // debug log
    module.insert(
        "debug".to_string(),
        Value::NativeFunction(|args| {
            let message = args
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            println!("[DEBUG] {}", message);
            Ok(Value::None)
        }),
    );

    // success log
    module.insert(
        "success".to_string(),
        Value::NativeFunction(|args| {
            let message = args
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            println!("[SUCCESS] {}", message);
            Ok(Value::None)
        }),
    );

    module
}