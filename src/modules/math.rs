use crate::error::RutenError;
use crate::interpreter::Value;
use std::collections::HashMap;

pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();

    // constants
    module.insert("pi".to_string(), Value::Number(std::f64::consts::PI));
    module.insert("e".to_string(), Value::Number(std::f64::consts::E));

    // basic math functions
    module.insert(
        "sqrt".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("sqrt() takes 1 argument".to_string()));
            }
            match &args[0] {
                Value::Number(n) => Ok(Value::Number(n.sqrt())),
                _ => Err(RutenError::TypeError("sqrt() requires a number".to_string())),
            }
        }),
    );

    module.insert(
        "pow".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 2 {
                return Err(RutenError::RuntimeError("pow() takes 2 arguments".to_string()));
            }
            match (&args[0], &args[1]) {
                (Value::Number(base), Value::Number(exp)) => Ok(Value::Number(base.powf(*exp))),
                _ => Err(RutenError::TypeError("pow() requires numbers".to_string())),
            }
        }),
    );

    module.insert(
        "abs".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("abs() takes 1 argument".to_string()));
            }
            match &args[0] {
                Value::Number(n) => Ok(Value::Number(n.abs())),
                _ => Err(RutenError::TypeError("abs() requires a number".to_string())),
            }
        }),
    );

    // trigonometric functions
    module.insert(
        "sin".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("sin() takes 1 argument".to_string()));
            }
            match &args[0] {
                Value::Number(n) => Ok(Value::Number(n.sin())),
                _ => Err(RutenError::TypeError("sin() requires a number".to_string())),
            }
        }),
    );

    module.insert(
        "cos".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("cos() takes 1 argument".to_string()));
            }
            match &args[0] {
                Value::Number(n) => Ok(Value::Number(n.cos())),
                _ => Err(RutenError::TypeError("cos() requires a number".to_string())),
            }
        }),
    );

    module.insert(
        "tan".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("tan() takes 1 argument".to_string()));
            }
            match &args[0] {
                Value::Number(n) => Ok(Value::Number(n.tan())),
                _ => Err(RutenError::TypeError("tan() requires a number".to_string())),
            }
        }),
    );

    // statistical functions
    module.insert(
        "sum".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("sum() takes 1 argument".to_string()));
            }
            match &args[0] {
                Value::List(items) => {
                    let mut total = 0.0;
                    for item in items {
                        if let Value::Number(n) = item {
                            total += n;
                        } else {
                            return Err(RutenError::TypeError("sum() requires a list of numbers".to_string()));
                        }
                    }
                    Ok(Value::Number(total))
                }
                _ => Err(RutenError::TypeError("sum() requires a list".to_string())),
            }
        }),
    );

    module.insert(
        "mean".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("mean() takes 1 argument".to_string()));
            }
            match &args[0] {
                Value::List(items) => {
                    if items.is_empty() {
                        return Err(RutenError::RuntimeError("mean() requires non-empty list".to_string()));
                    }
                    let mut total = 0.0;
                    for item in items {
                        if let Value::Number(n) = item {
                            total += n;
                        } else {
                            return Err(RutenError::TypeError("mean() requires a list of numbers".to_string()));
                        }
                    }
                    Ok(Value::Number(total / items.len() as f64))
                }
                _ => Err(RutenError::TypeError("mean() requires a list".to_string())),
            }
        }),
    );

    // fibonacci function
    module.insert(
        "fibonacci".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("fibonacci() takes 1 argument".to_string()));
            }
            match &args[0] {
                Value::Number(n) => {
                    let n = *n as i32;
                    if n < 0 {
                        return Err(RutenError::RuntimeError("fibonacci() requires non-negative number".to_string()));
                    }
                    let result = fibonacci(n);
                    Ok(Value::Number(result as f64))
                }
                _ => Err(RutenError::TypeError("fibonacci() requires a number".to_string())),
            }
        }),
    );

    module.insert(
        "floor".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("floor() takes 1 argument".to_string()));
            }
            match &args[0] {
                Value::Number(n) => Ok(Value::Number(n.floor())),
                _ => Err(RutenError::TypeError("floor() requires a number".to_string())),
            }
        }),
    );

    module.insert(
        "ceil".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("ceil() takes 1 argument".to_string()));
            }
            match &args[0] {
                Value::Number(n) => Ok(Value::Number(n.ceil())),
                _ => Err(RutenError::TypeError("ceil() requires a number".to_string())),
            }
        }),
    );

    module
}

fn fibonacci(n: i32) -> i64 {
    if n <= 1 {
        return n as i64;
    }
    let mut a = 0i64;
    let mut b = 1i64;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}