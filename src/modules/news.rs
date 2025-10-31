use crate::error::RutenError;
use crate::interpreter::Value;
use std::collections::HashMap;

pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();

    // get top headlines
    module.insert(
        "headlines".to_string(),
        Value::NativeFunction(|args| {
            let country = if args.is_empty() {
                "us".to_string()
            } else {
                match &args[0] {
                    Value::String(s) => s.clone(),
                    _ => return Err(RutenError::TypeError("country must be a string".to_string())),
                }
            };

            // using newsapi.org free tier
            let url = format!("https://newsapi.org/v2/top-headlines?country={}&apiKey=demo", country);
            let client = reqwest::blocking::Client::new();
            
            match client.get(&url).send() {
                Ok(response) => {
                    let body = response.text().unwrap_or_default();
                    Ok(Value::String(body))
                }
                Err(e) => Err(RutenError::RuntimeError(format!("news api error: {}", e))),
            }
        }),
    );

    // search news articles
    module.insert(
        "search".to_string(),
        Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err(RutenError::RuntimeError(
                    "news.search() requires query".to_string(),
                ));
            }

            let query = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err(RutenError::TypeError("query must be a string".to_string())),
            };

            let url = format!("https://newsapi.org/v2/everything?q={}&apiKey=demo", 
                urlencoding::encode(&query));
            let client = reqwest::blocking::Client::new();
            
            match client.get(&url).send() {
                Ok(response) => {
                    let body = response.text().unwrap_or_default();
                    Ok(Value::String(body))
                }
                Err(e) => Err(RutenError::RuntimeError(format!("news api error: {}", e))),
            }
        }),
    );

    module
}