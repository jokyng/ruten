use crate::error::RutenError;
use crate::interpreter::Value;
use std::collections::HashMap;

pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();

    // get random joke
    module.insert(
        "random".to_string(),
        Value::NativeFunction(|_args| {
            let url = "https://official-joke-api.appspot.com/random_joke";
            let client = reqwest::blocking::Client::new();
            
            match client.get(url).send() {
                Ok(response) => {
                    let body = response.text().unwrap_or_default();
                    Ok(Value::String(body))
                }
                Err(e) => Err(RutenError::RuntimeError(format!("jokes api error: {}", e))),
            }
        }),
    );

    // get programming joke
    module.insert(
        "programming".to_string(),
        Value::NativeFunction(|_args| {
            let url = "https://official-joke-api.appspot.com/jokes/programming/random";
            let client = reqwest::blocking::Client::new();
            
            match client.get(url).send() {
                Ok(response) => {
                    let body = response.text().unwrap_or_default();
                    Ok(Value::String(body))
                }
                Err(e) => Err(RutenError::RuntimeError(format!("jokes api error: {}", e))),
            }
        }),
    );

    // get multiple jokes
    module.insert(
        "multiple".to_string(),
        Value::NativeFunction(|args| {
            let count = if args.is_empty() {
                5
            } else {
                match &args[0] {
                    Value::Number(n) => *n as i32,
                    _ => return Err(RutenError::TypeError("count must be a number".to_string())),
                }
            };

            let url = format!("https://official-joke-api.appspot.com/random_joke/{}", count);
            let client = reqwest::blocking::Client::new();
            
            match client.get(&url).send() {
                Ok(response) => {
                    let body = response.text().unwrap_or_default();
                    Ok(Value::String(body))
                }
                Err(e) => Err(RutenError::RuntimeError(format!("jokes api error: {}", e))),
            }
        }),
    );

    module
}