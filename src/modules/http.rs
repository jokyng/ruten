use crate::error::RutenError;
use crate::interpreter::Value;
use std::collections::HashMap;
use reqwest::blocking;

pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();

    // http.get(url, headers?) - get request
    module.insert(
        "get".to_string(),
        Value::NativeFunction(|args| {
            if args.is_empty() || args.len() > 2 {
                return Err(RutenError::RuntimeError("get() takes 1 or 2 arguments".to_string()));
            }
            match &args[0] {
                Value::String(url) => {
                    let client = blocking::Client::new();
                    let mut request = client.get(url);
                    
                    // add headers if provided
                    if args.len() == 2 {
                        if let Value::Dict(headers) = &args[1] {
                            for (key, val) in headers {
                                if let Value::String(v) = val {
                                    request = request.header(key, v);
                                }
                            }
                        }
                    }
                    
                    let response = request
                        .send()
                        .map_err(|e| RutenError::RuntimeError(format!("http error: {}", e)))?;
                    
                    let status = response.status().as_u16() as f64;
                    let text = response
                        .text()
                        .map_err(|e| RutenError::RuntimeError(format!("response error: {}", e)))?;
                    
                    let mut result = HashMap::new();
                    result.insert("status".to_string(), Value::Number(status));
                    result.insert("body".to_string(), Value::String(text));
                    Ok(Value::Dict(result))
                }
                _ => Err(RutenError::TypeError("get() requires a url string".to_string())),
            }
        }),
    );

    // http.post(url, body?, headers?) - post request
    module.insert(
        "post".to_string(),
        Value::NativeFunction(|args| {
            if args.is_empty() || args.len() > 3 {
                return Err(RutenError::RuntimeError("post() takes 1 to 3 arguments".to_string()));
            }
            match &args[0] {
                Value::String(url) => {
                    let client = blocking::Client::new();
                    let mut request = client.post(url);
                    
                    // add body if provided
                    if args.len() >= 2 {
                        if let Value::String(body) = &args[1] {
                            request = request.body(body.clone());
                        }
                    }
                    
                    // add headers if provided
                    if args.len() == 3 {
                        if let Value::Dict(headers) = &args[2] {
                            for (key, val) in headers {
                                if let Value::String(v) = val {
                                    request = request.header(key, v);
                                }
                            }
                        }
                    }
                    
                    let response = request
                        .send()
                        .map_err(|e| RutenError::RuntimeError(format!("http error: {}", e)))?;
                    
                    let status = response.status().as_u16() as f64;
                    let text = response
                        .text()
                        .map_err(|e| RutenError::RuntimeError(format!("response error: {}", e)))?;
                    
                    let mut result = HashMap::new();
                    result.insert("status".to_string(), Value::Number(status));
                    result.insert("body".to_string(), Value::String(text));
                    Ok(Value::Dict(result))
                }
                _ => Err(RutenError::TypeError("post() requires a url string".to_string())),
            }
        }),
    );

    module
}