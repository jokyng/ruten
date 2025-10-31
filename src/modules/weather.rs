use crate::error::RutenError;
use crate::interpreter::Value;
use std::collections::HashMap;
use reqwest::blocking;

pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();

    // get current weather for a location
    module.insert(
        "current".to_string(),
        Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err(RutenError::RuntimeError("current() takes 1 argument: location".to_string()));
            }
            
            match &args[0] {
                Value::String(location) => {
                    let url = format!("https://wttr.in/{}?format=j1", location);
                    let response = blocking::get(&url)
                        .map_err(|e| RutenError::RuntimeError(format!("weather api error: {}", e)))?;
                    
                    let text = response.text()
                        .map_err(|e| RutenError::RuntimeError(format!("response error: {}", e)))?;
                    
                    Ok(Value::String(text))
                }
                _ => Err(RutenError::TypeError("current() requires a string location".to_string())),
            }
        }),
    );

    // get simple weather description
    module.insert(
        "simple".to_string(),
        Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err(RutenError::RuntimeError("simple() takes 1 argument: location".to_string()));
            }
            
            match &args[0] {
                Value::String(location) => {
                    let url = format!("https://wttr.in/{}?format=%C+%t", location);
                    let response = blocking::get(&url)
                        .map_err(|e| RutenError::RuntimeError(format!("weather api error: {}", e)))?;
                    
                    let text = response.text()
                        .map_err(|e| RutenError::RuntimeError(format!("response error: {}", e)))?;
                    
                    Ok(Value::String(text.trim().to_string()))
                }
                _ => Err(RutenError::TypeError("simple() requires a string location".to_string())),
            }
        }),
    );

    // get forecast for location
    module.insert(
        "forecast".to_string(),
        Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err(RutenError::RuntimeError("forecast() takes 1 argument: location".to_string()));
            }
            
            match &args[0] {
                Value::String(location) => {
                    let url = format!("https://wttr.in/{}?format=j1", location);
                    let response = blocking::get(&url)
                        .map_err(|e| RutenError::RuntimeError(format!("weather api error: {}", e)))?;
                    
                    let text = response.text()
                        .map_err(|e| RutenError::RuntimeError(format!("response error: {}", e)))?;
                    
                    Ok(Value::String(text))
                }
                _ => Err(RutenError::TypeError("forecast() requires a string location".to_string())),
            }
        }),
    );

    module
}