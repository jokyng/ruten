use crate::error::RutenError;
use crate::interpreter::Value;
use std::collections::HashMap;

pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();

    // get random fact
    module.insert(
        "random".to_string(),
        Value::NativeFunction(|_args| {
            let url = "https://uselessfacts.jsph.pl/random.json?language=en";
            let client = reqwest::blocking::Client::new();
            
            match client.get(url).send() {
                Ok(response) => {
                    let body = response.text().unwrap_or_default();
                    Ok(Value::String(body))
                }
                Err(e) => Err(RutenError::RuntimeError(format!("facts api error: {}", e))),
            }
        }),
    );

    // get fact of the day
    module.insert(
        "today".to_string(),
        Value::NativeFunction(|_args| {
            let url = "https://uselessfacts.jsph.pl/today.json?language=en";
            let client = reqwest::blocking::Client::new();
            
            match client.get(url).send() {
                Ok(response) => {
                    let body = response.text().unwrap_or_default();
                    Ok(Value::String(body))
                }
                Err(e) => Err(RutenError::RuntimeError(format!("facts api error: {}", e))),
            }
        }),
    );

    module
}