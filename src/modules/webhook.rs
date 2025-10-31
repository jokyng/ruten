use crate::error::RutenError;
use crate::interpreter::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};

pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();

    // webhook.listen(port, handler_function)
    module.insert(
        "listen".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 2 {
                return Err(RutenError::RuntimeError(
                    "listen() takes 2 arguments: port and handler function".to_string()
                ));
            }

            let port = match &args[0] {
                Value::Number(n) => *n as u16,
                _ => return Err(RutenError::TypeError(
                    "listen() first argument must be a port number".to_string()
                )),
            };

            // note: in a real implementation, we would need to handle the function callback
            // this is a simplified version that shows the structure only
            println!("webhook server would start on port {}", port);
            println!("note: full async webhook server requires runtime integration");
            
            Ok(Value::None)
        }),
    );

    // webhook.respond(status, body)
    module.insert(
        "respond".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 2 {
                return Err(RutenError::RuntimeError(
                    "respond() takes 2 arguments: status code and body".to_string()
                ));
            }

            let status = match &args[0] {
                Value::Number(n) => *n as u16,
                _ => return Err(RutenError::TypeError(
                    "respond() first argument must be a status code".to_string()
                )),
            };

            let body = match &args[1] {
                Value::String(s) => s.clone(),
                other => other.to_string(),
            };

            // create response dict
            let mut response = HashMap::new();
            response.insert("status".to_string(), Value::Number(status as f64));
            response.insert("body".to_string(), Value::String(body));
            
            Ok(Value::Dict(response))
        }),
    );

    module
}