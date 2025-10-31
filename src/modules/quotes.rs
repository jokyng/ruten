use crate::error::RutenError;
use crate::interpreter::Value;
use std::collections::HashMap;

pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();

    // get random quote
    module.insert(
        "random".to_string(),
        Value::NativeFunction(|_args| {
            let url = "https://api.quotable.io/random";
            let client = reqwest::blocking::Client::new();
            
            match client.get(url).send() {
                Ok(response) => {
                    let body = response.text().unwrap_or_default();
                    Ok(Value::String(body))
                }
                Err(e) => Err(RutenError::RuntimeError(format!("quotes api error: {}", e))),
            }
        }),
    );

    // get quote by author
    module.insert(
        "author".to_string(),
        Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err(RutenError::RuntimeError(
                    "quotes.author() requires author name".to_string(),
                ));
            }

            let author = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err(RutenError::TypeError("author must be a string".to_string())),
            };

            let url = format!("https://api.quotable.io/quotes?author={}", 
                urlencoding::encode(&author));
            let client = reqwest::blocking::Client::new();
            
            match client.get(&url).send() {
                Ok(response) => {
                    let body = response.text().unwrap_or_default();
                    Ok(Value::String(body))
                }
                Err(e) => Err(RutenError::RuntimeError(format!("quotes api error: {}", e))),
            }
        }),
    );

    // get quote of the day
    module.insert(
        "today".to_string(),
        Value::NativeFunction(|_args| {
            let url = "https://api.quotable.io/quotes/random?limit=1";
            let client = reqwest::blocking::Client::new();
            
            match client.get(url).send() {
                Ok(response) => {
                    let body = response.text().unwrap_or_default();
                    Ok(Value::String(body))
                }
                Err(e) => Err(RutenError::RuntimeError(format!("quotes api error: {}", e))),
            }
        }),
    );

    module
}