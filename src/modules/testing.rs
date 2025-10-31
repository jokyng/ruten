use crate::error::RutenError;
use crate::interpreter::Value;
use std::collections::HashMap;

pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();

    // assert equal
    module.insert(
        "assert_equal".to_string(),
        Value::NativeFunction(|args| {
            if args.len() < 2 {
                return Err(RutenError::RuntimeError(
                    "testing.assert_equal() requires two values".to_string(),
                ));
            }

            let a = &args[0];
            let b = &args[1];

            let equal = match (a, b) {
                (Value::Number(x), Value::Number(y)) => x == y,
                (Value::String(x), Value::String(y)) => x == y,
                (Value::Bool(x), Value::Bool(y)) => x == y,
                _ => false,
            };

            if !equal {
                return Err(RutenError::RuntimeError(format!(
                    "assertion failed: {} != {}",
                    a.to_string(),
                    b.to_string()
                )));
            }

            Ok(Value::Bool(true))
        }),
    );

    // assert true
    module.insert(
        "assert_true".to_string(),
        Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err(RutenError::RuntimeError(
                    "testing.assert_true() requires value".to_string(),
                ));
            }

            if !args[0].is_truthy() {
                return Err(RutenError::RuntimeError(
                    "assertion failed: value is not true".to_string(),
                ));
            }

            Ok(Value::Bool(true))
        }),
    );

    // assert false
    module.insert(
        "assert_false".to_string(),
        Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err(RutenError::RuntimeError(
                    "testing.assert_false() requires value".to_string(),
                ));
            }

            if args[0].is_truthy() {
                return Err(RutenError::RuntimeError(
                    "assertion failed: value is not false".to_string(),
                ));
            }

            Ok(Value::Bool(true))
        }),
    );

    module
}