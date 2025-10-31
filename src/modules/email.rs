use crate::error::RutenError;
use crate::interpreter::Value;
use std::collections::HashMap;

pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();

    // send email (simplified - would need proper smtp in production)
    module.insert(
        "send".to_string(),
        Value::NativeFunction(|args| {
            if args.len() < 3 {
                return Err(RutenError::RuntimeError(
                    "email.send() requires to, subject, body".to_string(),
                ));
            }

            let to = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err(RutenError::TypeError("to must be a string".to_string())),
            };

            let subject = match &args[1] {
                Value::String(s) => s.clone(),
                _ => return Err(RutenError::TypeError("subject must be a string".to_string())),
            };

            let body = match &args[2] {
                Value::String(s) => s.clone(),
                _ => return Err(RutenError::TypeError("body must be a string".to_string())),
            };

            // in production, this would use lettre or similar
            println!("email sent to: {}", to);
            println!("subject: {}", subject);
            println!("body: {}", body);

            Ok(Value::Bool(true))
        }),
    );

    // validate email format
    module.insert(
        "validate".to_string(),
        Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err(RutenError::RuntimeError(
                    "email.validate() requires email address".to_string(),
                ));
            }

            let email = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err(RutenError::TypeError("email must be a string".to_string())),
            };

            // basic email validation
            let is_valid = email.contains('@') && email.contains('.') && email.len() > 5;

            Ok(Value::Bool(is_valid))
        }),
    );

    module
}