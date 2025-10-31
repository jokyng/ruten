use crate::error::RutenError;
use crate::interpreter::Value;
use std::collections::HashMap;

pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();

    // placeholder for ai integrations
    // these would require api keys and async runtime
    module.insert(
        "chat".to_string(),
        Value::NativeFunction(|args| {
            if args.len() < 2 {
                return Err(RutenError::RuntimeError("chat() takes at least 2 arguments: model, prompt".to_string()));
            }
            
            match (&args[0], &args[1]) {
                (Value::String(model), Value::String(prompt)) => {
                    // placeholder implementation
                    // in production, this would call openai/claude/ollama apis
                    Err(RutenError::RuntimeError(
                        format!("ai.chat() not yet implemented. would call {} with prompt: {}", model, prompt)
                    ))
                }
                _ => Err(RutenError::TypeError("chat() requires model and prompt strings".to_string())),
            }
        }),
    );

    module.insert(
        "complete".to_string(),
        Value::NativeFunction(|args| {
            if args.len() < 2 {
                return Err(RutenError::RuntimeError("complete() takes at least 2 arguments".to_string()));
            }
            
            Err(RutenError::RuntimeError("ai.complete() not yet implemented".to_string()))
        }),
    );

    module
}