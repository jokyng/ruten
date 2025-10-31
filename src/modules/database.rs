use crate::error::RutenError;
use crate::interpreter::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

lazy_static! {
    static ref DB_STORE: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();

    // set key-value pair
    module.insert(
        "set".to_string(),
        Value::NativeFunction(|args| {
            if args.len() < 2 {
                return Err(RutenError::RuntimeError(
                    "database.set() requires key and value".to_string(),
                ));
            }

            let key = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err(RutenError::TypeError("key must be a string".to_string())),
            };

            let value = args[1].to_string();

            let mut store = DB_STORE.lock().unwrap();
            store.insert(key, value);

            Ok(Value::Bool(true))
        }),
    );

    // get value by key
    module.insert(
        "get".to_string(),
        Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err(RutenError::RuntimeError(
                    "database.get() requires key".to_string(),
                ));
            }

            let key = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err(RutenError::TypeError("key must be a string".to_string())),
            };

            let store = DB_STORE.lock().unwrap();
            match store.get(&key) {
                Some(value) => Ok(Value::String(value.clone())),
                None => Ok(Value::None),
            }
        }),
    );

    // delete key
    module.insert(
        "delete".to_string(),
        Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err(RutenError::RuntimeError(
                    "database.delete() requires key".to_string(),
                ));
            }

            let key = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err(RutenError::TypeError("key must be a string".to_string())),
            };

            let mut store = DB_STORE.lock().unwrap();
            store.remove(&key);

            Ok(Value::Bool(true))
        }),
    );

    // check if key exists
    module.insert(
        "exists".to_string(),
        Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err(RutenError::RuntimeError(
                    "database.exists() requires key".to_string(),
                ));
            }

            let key = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err(RutenError::TypeError("key must be a string".to_string())),
            };

            let store = DB_STORE.lock().unwrap();
            Ok(Value::Bool(store.contains_key(&key)))
        }),
    );

    // get all keys
    module.insert(
        "keys".to_string(),
        Value::NativeFunction(|_args| {
            let store = DB_STORE.lock().unwrap();
            let keys: Vec<Value> = store
                .keys()
                .map(|k| Value::String(k.clone()))
                .collect();
            Ok(Value::List(keys))
        }),
    );

    // clear all data
    module.insert(
        "clear".to_string(),
        Value::NativeFunction(|_args| {
            let mut store = DB_STORE.lock().unwrap();
            store.clear();
            Ok(Value::Bool(true))
        }),
    );

    module
}