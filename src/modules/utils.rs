use crate::error::RutenError;
use crate::interpreter::Value;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();

    // get current unix timestamp
    module.insert(
        "timestamp".to_string(),
        Value::NativeFunction(|args| {
            if !args.is_empty() {
                return Err(RutenError::RuntimeError("timestamp() takes no arguments".to_string()));
            }
            
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| RutenError::RuntimeError(format!("time error: {}", e)))?;
            
            Ok(Value::Number(now.as_secs() as f64))
        }),
    );

    // get current timestamp in milliseconds
    module.insert(
        "timestamp_ms".to_string(),
        Value::NativeFunction(|args| {
            if !args.is_empty() {
                return Err(RutenError::RuntimeError("timestamp_ms() takes no arguments".to_string()));
            }
            
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| RutenError::RuntimeError(format!("time error: {}", e)))?;
            
            Ok(Value::Number(now.as_millis() as f64))
        }),
    );

    // generate uuid v4
    module.insert(
        "uuid".to_string(),
        Value::NativeFunction(|args| {
            if !args.is_empty() {
                return Err(RutenError::RuntimeError("uuid() takes no arguments".to_string()));
            }
            
            let id = Uuid::new_v4();
            Ok(Value::String(id.to_string()))
        }),
    );

    // sleep for specified seconds
    module.insert(
        "sleep".to_string(),
        Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err(RutenError::RuntimeError("sleep() takes 1 argument: seconds".to_string()));
            }
            
            match &args[0] {
                Value::Number(seconds) => {
                    if *seconds < 0.0 {
                        return Err(RutenError::RuntimeError("sleep() requires positive number".to_string()));
                    }
                    
                    let duration = std::time::Duration::from_secs_f64(*seconds);
                    std::thread::sleep(duration);
                    Ok(Value::None)
                }
                _ => Err(RutenError::TypeError("sleep() requires a number".to_string())),
            }
        }),
    );

    // get environment variable
    module.insert(
        "getenv".to_string(),
        Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err(RutenError::RuntimeError("getenv() takes 1 argument: variable name".to_string()));
            }
            
            match &args[0] {
                Value::String(var_name) => {
                    match std::env::var(var_name) {
                        Ok(value) => Ok(Value::String(value)),
                        Err(_) => Ok(Value::None),
                    }
                }
                _ => Err(RutenError::TypeError("getenv() requires a string".to_string())),
            }
        }),
    );

    // set environment variable
    module.insert(
        "setenv".to_string(),
        Value::NativeFunction(|args| {
            if args.len() < 2 {
                return Err(RutenError::RuntimeError("setenv() takes 2 arguments: name, value".to_string()));
            }
            
            match (&args[0], &args[1]) {
                (Value::String(name), Value::String(value)) => {
                    std::env::set_var(name, value);
                    Ok(Value::None)
                }
                _ => Err(RutenError::TypeError("setenv() requires two strings".to_string())),
            }
        }),
    );

    // generate random number between 0 and 1
    module.insert(
        "random".to_string(),
        Value::NativeFunction(|args| {
            if !args.is_empty() {
                return Err(RutenError::RuntimeError("random() takes no arguments".to_string()));
            }
            
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let num: f64 = rng.gen();
            Ok(Value::Number(num))
        }),
    );

    // generate random integer in range
    module.insert(
        "randint".to_string(),
        Value::NativeFunction(|args| {
            if args.len() < 2 {
                return Err(RutenError::RuntimeError("randint() takes 2 arguments: min, max".to_string()));
            }
            
            match (&args[0], &args[1]) {
                (Value::Number(min), Value::Number(max)) => {
                    use rand::Rng;
                    let mut rng = rand::thread_rng();
                    let num = rng.gen_range((*min as i64)..=(*max as i64));
                    Ok(Value::Number(num as f64))
                }
                _ => Err(RutenError::TypeError("randint() requires two numbers".to_string())),
            }
        }),
    );

    // choose random element from list
    module.insert(
        "choice".to_string(),
        Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err(RutenError::RuntimeError("choice() takes 1 argument: list".to_string()));
            }
            
            match &args[0] {
                Value::List(items) => {
                    if items.is_empty() {
                        return Err(RutenError::RuntimeError("choice() requires non-empty list".to_string()));
                    }
                    
                    use rand::Rng;
                    let mut rng = rand::thread_rng();
                    let index = rng.gen_range(0..items.len());
                    Ok(items[index].clone())
                }
                _ => Err(RutenError::TypeError("choice() requires a list".to_string())),
            }
        }),
    );

    module
}