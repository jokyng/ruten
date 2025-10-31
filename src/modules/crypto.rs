use crate::error::RutenError;
use crate::interpreter::Value;
use std::collections::HashMap;
use sha2::{Sha256, Sha512, Digest};
use md5;
use rand::Rng;

pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();

    // crypto.sha512(data) - sha512 hash
    module.insert(
        "sha512".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("sha512() takes 1 argument".to_string()));
            }
            match &args[0] {
                Value::String(s) => {
                    let mut hasher = Sha512::new();
                    hasher.update(s.as_bytes());
                    let result = hasher.finalize();
                    Ok(Value::String(format!("{:x}", result)))
                }
                _ => Err(RutenError::TypeError("sha512() requires a string".to_string())),
            }
        }),
    );

    // crypto.sha256(data) - sha256 hash
    module.insert(
        "sha256".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("sha256() takes 1 argument".to_string()));
            }
            match &args[0] {
                Value::String(s) => {
                    let mut hasher = Sha256::new();
                    hasher.update(s.as_bytes());
                    let result = hasher.finalize();
                    Ok(Value::String(format!("{:x}", result)))
                }
                _ => Err(RutenError::TypeError("sha256() requires a string".to_string())),
            }
        }),
    );

    // crypto.md5(data) - md5 hash
    module.insert(
        "md5".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("md5() takes 1 argument".to_string()));
            }
            match &args[0] {
                Value::String(s) => {
                    let digest = md5::compute(s.as_bytes());
                    Ok(Value::String(format!("{:x}", digest)))
                }
                _ => Err(RutenError::TypeError("md5() requires a string".to_string())),
            }
        }),
    );

    // crypto.random_bytes(length) - secure random byte generation
    module.insert(
        "random_bytes".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("random_bytes() takes 1 argument".to_string()));
            }
            match &args[0] {
                Value::Number(n) => {
                    let length = *n as usize;
                    let mut rng = rand::thread_rng();
                    let bytes: Vec<u8> = (0..length).map(|_| rng.gen::<u8>()).collect();
                    Ok(Value::String(hex::encode(bytes)))
                }
                _ => Err(RutenError::TypeError("random_bytes() requires a number".to_string())),
            }
        }),
    );

    // crypto.random() - random float between 0 and 1
    module.insert(
        "random".to_string(),
        Value::NativeFunction(|args| {
            if !args.is_empty() {
                return Err(RutenError::RuntimeError("random() takes no arguments".to_string()));
            }
            let mut rng = rand::thread_rng();
            Ok(Value::Number(rng.gen::<f64>()))
        }),
    );

    // crypto.randint(min, max) - random integer in range
    module.insert(
        "randint".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 2 {
                return Err(RutenError::RuntimeError("randint() takes 2 arguments".to_string()));
            }
            match (&args[0], &args[1]) {
                (Value::Number(min), Value::Number(max)) => {
                    let mut rng = rand::thread_rng();
                    let result = rng.gen_range((*min as i64)..=(*max as i64));
                    Ok(Value::Number(result as f64))
                }
                _ => Err(RutenError::TypeError("randint() requires two numbers".to_string())),
            }
        }),
    );

    // crypto.random_hex(length) - random hex string
    module.insert(
        "random_hex".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("random_hex() takes 1 argument".to_string()));
            }
            match &args[0] {
                Value::Number(n) => {
                    let length = *n as usize;
                    let mut rng = rand::thread_rng();
                    let bytes: Vec<u8> = (0..length).map(|_| rng.gen::<u8>()).collect();
                    Ok(Value::String(hex::encode(bytes)))
                }
                _ => Err(RutenError::TypeError("random_hex() requires a number".to_string())),
            }
        }),
    );

    // crypto.choice(list) - random element from list
    module.insert(
        "choice".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("choice() takes 1 argument".to_string()));
            }
            match &args[0] {
                Value::List(items) => {
                    if items.is_empty() {
                        return Err(RutenError::RuntimeError("choice() requires non-empty list".to_string()));
                    }
                    let mut rng = rand::thread_rng();
                    let index = rng.gen_range(0..items.len());
                    Ok(items[index].clone())
                }
                _ => Err(RutenError::TypeError("choice() requires a list".to_string())),
            }
        }),
    );

    // crypto.uuid() - generate uuid v4
    module.insert(
        "uuid".to_string(),
        Value::NativeFunction(|args| {
            if !args.is_empty() {
                return Err(RutenError::RuntimeError("uuid() takes no arguments".to_string()));
            }
            let mut rng = rand::thread_rng();
            let uuid = format!(
                "{:08x}-{:04x}-4{:03x}-{:04x}-{:012x}",
                rng.gen::<u32>(),
                rng.gen::<u16>(),
                rng.gen::<u16>() & 0x0fff,
                (rng.gen::<u16>() & 0x3fff) | 0x8000,
                rng.gen::<u64>() & 0xffffffffffff
            );
            Ok(Value::String(uuid))
        }),
    );

    module
}