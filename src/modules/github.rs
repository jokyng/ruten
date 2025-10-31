use crate::error::RutenError;
use crate::interpreter::Value;
use std::collections::HashMap;

pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();

    // get user information
    module.insert(
        "user".to_string(),
        Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err(RutenError::RuntimeError(
                    "github.user() requires username".to_string(),
                ));
            }

            let username = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err(RutenError::TypeError("username must be a string".to_string())),
            };

            // make api request
            let url = format!("https://api.github.com/users/{}", username);
            let client = reqwest::blocking::Client::new();
            
            match client
                .get(&url)
                .header("user-agent", "ruten/2.0")
                .send()
            {
                Ok(response) => {
                    let body = response.text().unwrap_or_default();
                    Ok(Value::String(body))
                }
                Err(e) => Err(RutenError::RuntimeError(format!("github api error: {}", e))),
            }
        }),
    );

    // get repository information
    module.insert(
        "repo".to_string(),
        Value::NativeFunction(|args| {
            if args.len() < 2 {
                return Err(RutenError::RuntimeError(
                    "github.repo() requires owner and repo name".to_string(),
                ));
            }

            let owner = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err(RutenError::TypeError("owner must be a string".to_string())),
            };

            let repo = match &args[1] {
                Value::String(s) => s.clone(),
                _ => return Err(RutenError::TypeError("repo must be a string".to_string())),
            };

            let url = format!("https://api.github.com/repos/{}/{}", owner, repo);
            let client = reqwest::blocking::Client::new();
            
            match client
                .get(&url)
                .header("user-agent", "ruten/2.0")
                .send()
            {
                Ok(response) => {
                    let body = response.text().unwrap_or_default();
                    Ok(Value::String(body))
                }
                Err(e) => Err(RutenError::RuntimeError(format!("github api error: {}", e))),
            }
        }),
    );

    // search repositories
    module.insert(
        "search".to_string(),
        Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err(RutenError::RuntimeError(
                    "github.search() requires query".to_string(),
                ));
            }

            let query = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err(RutenError::TypeError("query must be a string".to_string())),
            };

            let url = format!("https://api.github.com/search/repositories?q={}", 
                urlencoding::encode(&query));
            let client = reqwest::blocking::Client::new();
            
            match client
                .get(&url)
                .header("user-agent", "ruten/2.0")
                .send()
            {
                Ok(response) => {
                    let body = response.text().unwrap_or_default();
                    Ok(Value::String(body))
                }
                Err(e) => Err(RutenError::RuntimeError(format!("github api error: {}", e))),
            }
        }),
    );

    module
}