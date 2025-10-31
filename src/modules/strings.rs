use crate::error::RutenError;
use crate::interpreter::Value;
use std::collections::HashMap;
use regex::Regex;

pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();

    module.insert(
        "upper".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("upper() takes 1 argument".to_string()));
            }
            match &args[0] {
                Value::String(s) => Ok(Value::String(s.to_uppercase())),
                _ => Err(RutenError::TypeError("upper() requires a string".to_string())),
            }
        }),
    );

    module.insert(
        "lower".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("lower() takes 1 argument".to_string()));
            }
            match &args[0] {
                Value::String(s) => Ok(Value::String(s.to_lowercase())),
                _ => Err(RutenError::TypeError("lower() requires a string".to_string())),
            }
        }),
    );

    module.insert(
        "trim".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("trim() takes 1 argument".to_string()));
            }
            match &args[0] {
                Value::String(s) => Ok(Value::String(s.trim().to_string())),
                _ => Err(RutenError::TypeError("trim() requires a string".to_string())),
            }
        }),
    );

    module.insert(
        "split".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 2 {
                return Err(RutenError::RuntimeError("split() takes 2 arguments".to_string()));
            }
            match (&args[0], &args[1]) {
                (Value::String(s), Value::String(delimiter)) => {
                    let parts: Vec<Value> = s
                        .split(delimiter.as_str())
                        .map(|p| Value::String(p.to_string()))
                        .collect();
                    Ok(Value::List(parts))
                }
                _ => Err(RutenError::TypeError("split() requires two strings".to_string())),
            }
        }),
    );

    module.insert(
        "join".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 2 {
                return Err(RutenError::RuntimeError("join() takes 2 arguments".to_string()));
            }
            match (&args[0], &args[1]) {
                (Value::String(separator), Value::List(items)) => {
                    let strings: Result<Vec<String>, _> = items
                        .iter()
                        .map(|v| match v {
                            Value::String(s) => Ok(s.clone()),
                            _ => Err(RutenError::TypeError("join() requires list of strings".to_string())),
                        })
                        .collect();
                    Ok(Value::String(strings?.join(separator)))
                }
                _ => Err(RutenError::TypeError("join() requires string and list".to_string())),
            }
        }),
    );

    module.insert(
        "replace".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 3 {
                return Err(RutenError::RuntimeError("replace() takes 3 arguments".to_string()));
            }
            match (&args[0], &args[1], &args[2]) {
                (Value::String(s), Value::String(from), Value::String(to)) => {
                    Ok(Value::String(s.replace(from, to)))
                }
                _ => Err(RutenError::TypeError("replace() requires three strings".to_string())),
            }
        }),
    );

    module.insert(
        "startswith".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 2 {
                return Err(RutenError::RuntimeError("startswith() takes 2 arguments".to_string()));
            }
            match (&args[0], &args[1]) {
                (Value::String(s), Value::String(prefix)) => {
                    Ok(Value::Bool(s.starts_with(prefix)))
                }
                _ => Err(RutenError::TypeError("startswith() requires two strings".to_string())),
            }
        }),
    );

    module.insert(
        "endswith".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 2 {
                return Err(RutenError::RuntimeError("endswith() takes 2 arguments".to_string()));
            }
            match (&args[0], &args[1]) {
                (Value::String(s), Value::String(suffix)) => {
                    Ok(Value::Bool(s.ends_with(suffix)))
                }
                _ => Err(RutenError::TypeError("endswith() requires two strings".to_string())),
            }
        }),
    );

    module.insert(
        "contains".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 2 {
                return Err(RutenError::RuntimeError("contains() takes 2 arguments".to_string()));
            }
            match (&args[0], &args[1]) {
                (Value::String(s), Value::String(substring)) => {
                    Ok(Value::Bool(s.contains(substring.as_str())))
                }
                _ => Err(RutenError::TypeError("contains() requires two strings".to_string())),
            }
        }),
    );

    module.insert(
        "regex_match".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 2 {
                return Err(RutenError::RuntimeError("regex_match() takes 2 arguments".to_string()));
            }
            match (&args[0], &args[1]) {
                (Value::String(pattern), Value::String(text)) => {
                    let re = Regex::new(pattern)
                        .map_err(|e| RutenError::RuntimeError(format!("invalid regex: {}", e)))?;
                    Ok(Value::Bool(re.is_match(text)))
                }
                _ => Err(RutenError::TypeError("regex_match() requires two strings".to_string())),
            }
        }),
    );

    module.insert(
        "regex_find".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 2 {
                return Err(RutenError::RuntimeError("regex_find() takes 2 arguments".to_string()));
            }
            match (&args[0], &args[1]) {
                (Value::String(pattern), Value::String(text)) => {
                    let re = Regex::new(pattern)
                        .map_err(|e| RutenError::RuntimeError(format!("invalid regex: {}", e)))?;
                    let matches: Vec<Value> = re
                        .find_iter(text)
                        .map(|m| Value::String(m.as_str().to_string()))
                        .collect();
                    Ok(Value::List(matches))
                }
                _ => Err(RutenError::TypeError("regex_find() requires two strings".to_string()
            )),
            }
        }),
    );

    module
}