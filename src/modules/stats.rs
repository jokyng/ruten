use crate::error::RutenError;
use crate::interpreter::Value;
use std::collections::HashMap;

pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();

    // calculate mean (average)
    module.insert(
        "mean".to_string(),
        Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err(RutenError::RuntimeError("mean() takes 1 argument: list of numbers".to_string()));
            }
            
            match &args[0] {
                Value::List(items) => {
                    if items.is_empty() {
                        return Err(RutenError::RuntimeError("mean() requires non-empty list".to_string()));
                    }
                    
                    let mut sum = 0.0;
                    for item in items {
                        match item {
                            Value::Number(n) => sum += n,
                            _ => return Err(RutenError::TypeError("mean() requires list of numbers".to_string())),
                        }
                    }
                    
                    Ok(Value::Number(sum / items.len() as f64))
                }
                _ => Err(RutenError::TypeError("mean() requires a list".to_string())),
            }
        }),
    );

    // calculate median
    module.insert(
        "median".to_string(),
        Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err(RutenError::RuntimeError("median() takes 1 argument: list of numbers".to_string()));
            }
            
            match &args[0] {
                Value::List(items) => {
                    if items.is_empty() {
                        return Err(RutenError::RuntimeError("median() requires non-empty list".to_string()));
                    }
                    
                    let mut numbers: Vec<f64> = Vec::new();
                    for item in items {
                        match item {
                            Value::Number(n) => numbers.push(*n),
                            _ => return Err(RutenError::TypeError("median() requires list of numbers".to_string())),
                        }
                    }
                    
                    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    let len = numbers.len();
                    let median = if len % 2 == 0 {
                        (numbers[len / 2 - 1] + numbers[len / 2]) / 2.0
                    } else {
                        numbers[len / 2]
                    };
                    
                    Ok(Value::Number(median))
                }
                _ => Err(RutenError::TypeError("median() requires a list".to_string())),
            }
        }),
    );

    // calculate standard deviation
    module.insert(
        "stdev".to_string(),
        Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err(RutenError::RuntimeError("stdev() takes 1 argument: list of numbers".to_string()));
            }
            
            match &args[0] {
                Value::List(items) => {
                    if items.is_empty() {
                        return Err(RutenError::RuntimeError("stdev() requires non-empty list".to_string()));
                    }
                    
                    let mut numbers: Vec<f64> = Vec::new();
                    for item in items {
                        match item {
                            Value::Number(n) => numbers.push(*n),
                            _ => return Err(RutenError::TypeError("stdev() requires list of numbers".to_string())),
                        }
                    }
                    
                    let mean = numbers.iter().sum::<f64>() / numbers.len() as f64;
                    let variance = numbers.iter()
                        .map(|n| (n - mean).powi(2))
                        .sum::<f64>() / numbers.len() as f64;
                    
                    Ok(Value::Number(variance.sqrt()))
                }
                _ => Err(RutenError::TypeError("stdev() requires a list".to_string())),
            }
        }),
    );

    // find minimum value
    module.insert(
        "min".to_string(),
        Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err(RutenError::RuntimeError("min() takes 1 argument: list of numbers".to_string()));
            }
            
            match &args[0] {
                Value::List(items) => {
                    if items.is_empty() {
                        return Err(RutenError::RuntimeError("min() requires non-empty list".to_string()));
                    }
                    
                    let mut min = f64::INFINITY;
                    for item in items {
                        match item {
                            Value::Number(n) => {
                                if *n < min {
                                    min = *n;
                                }
                            }
                            _ => return Err(RutenError::TypeError("min() requires list of numbers".to_string())),
                        }
                    }
                    
                    Ok(Value::Number(min))
                }
                _ => Err(RutenError::TypeError("min() requires a list".to_string())),
            }
        }),
    );

    // find maximum value
    module.insert(
        "max".to_string(),
        Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err(RutenError::RuntimeError("max() takes 1 argument: list of numbers".to_string()));
            }
            
            match &args[0] {
                Value::List(items) => {
                    if items.is_empty() {
                        return Err(RutenError::RuntimeError("max() requires non-empty list".to_string()));
                    }
                    
                    let mut max = f64::NEG_INFINITY;
                    for item in items {
                        match item {
                            Value::Number(n) => {
                                if *n > max {
                                    max = *n;
                                }
                            }
                            _ => return Err(RutenError::TypeError("max() requires list of numbers".to_string())),
                        }
                    }
                    
                    Ok(Value::Number(max))
                }
                _ => Err(RutenError::TypeError("max() requires a list".to_string())),
            }
        }),
    );

    // calculate variance
    module.insert(
        "variance".to_string(),
        Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err(RutenError::RuntimeError("variance() takes 1 argument: list of numbers".to_string()));
            }
            
            match &args[0] {
                Value::List(items) => {
                    if items.is_empty() {
                        return Err(RutenError::RuntimeError("variance() requires non-empty list".to_string()));
                    }
                    
                    let mut numbers: Vec<f64> = Vec::new();
                    for item in items {
                        match item {
                            Value::Number(n) => numbers.push(*n),
                            _ => return Err(RutenError::TypeError("variance() requires list of numbers".to_string())),
                        }
                    }
                    
                    let mean = numbers.iter().sum::<f64>() / numbers.len() as f64;
                    let variance = numbers.iter()
                        .map(|n| (n - mean).powi(2))
                        .sum::<f64>() / numbers.len() as f64;
                    
                    Ok(Value::Number(variance))
                }
                _ => Err(RutenError::TypeError("variance() requires a list".to_string())),
            }
        }),
    );

    // calculate correlation between two datasets
    module.insert(
        "correlation".to_string(),
        Value::NativeFunction(|args| {
            if args.len() < 2 {
                return Err(RutenError::RuntimeError("correlation() takes 2 arguments: two lists of numbers".to_string()));
            }
            
            match (&args[0], &args[1]) {
                (Value::List(x_items), Value::List(y_items)) => {
                    if x_items.len() != y_items.len() {
                        return Err(RutenError::RuntimeError("correlation() requires lists of equal length".to_string()));
                    }
                    
                    if x_items.is_empty() {
                        return Err(RutenError::RuntimeError("correlation() requires non-empty lists".to_string()));
                    }
                    
                    let mut x_vals: Vec<f64> = Vec::new();
                    let mut y_vals: Vec<f64> = Vec::new();
                    
                    for item in x_items {
                        match item {
                            Value::Number(n) => x_vals.push(*n),
                            _ => return Err(RutenError::TypeError("correlation() requires lists of numbers".to_string())),
                        }
                    }
                    
                    for item in y_items {
                        match item {
                            Value::Number(n) => y_vals.push(*n),
                            _ => return Err(RutenError::TypeError("correlation() requires lists of numbers".to_string())),
                        }
                    }
                    
                    let n = x_vals.len() as f64;
                    let x_mean = x_vals.iter().sum::<f64>() / n;
                    let y_mean = y_vals.iter().sum::<f64>() / n;
                    
                    let mut numerator = 0.0;
                    let mut x_sq_sum = 0.0;
                    let mut y_sq_sum = 0.0;
                    
                    for i in 0..x_vals.len() {
                        let x_diff = x_vals[i] - x_mean;
                        let y_diff = y_vals[i] - y_mean;
                        numerator += x_diff * y_diff;
                        x_sq_sum += x_diff * x_diff;
                        y_sq_sum += y_diff * y_diff;
                    }
                    
                    let correlation = numerator / (x_sq_sum * y_sq_sum).sqrt();
                    Ok(Value::Number(correlation))
                }
                _ => Err(RutenError::TypeError("correlation() requires two lists".to_string())),
            }
        }),
    );

    module
}