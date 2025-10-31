use crate::error::RutenError;
use crate::interpreter::Value;
use std::collections::HashMap;

pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();

    module.insert(
        "tokenize".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("tokenize() takes 1 argument".to_string()));
            }
            
            match &args[0] {
                Value::String(text) => {
                    // simple word tokenization
                    let tokens: Vec<Value> = text
                        .split_whitespace()
                        .map(|word| Value::String(word.to_string()))
                        .collect();
                    Ok(Value::List(tokens))
                }
                _ => Err(RutenError::TypeError("tokenize() requires a string".to_string())),
            }
        }),
    );

    module.insert(
        "word_count".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("word_count() takes 1 argument".to_string()));
            }
            
            match &args[0] {
                Value::String(text) => {
                    let count = text.split_whitespace().count();
                    Ok(Value::Number(count as f64))
                }
                _ => Err(RutenError::TypeError("word_count() requires a string".to_string())),
            }
        }),
    );

    module.insert(
        "sentiment".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("sentiment() takes 1 argument".to_string()));
            }
            
            match &args[0] {
                Value::String(text) => {
                    // simple sentiment analysis based on keywords
                    let positive_words = ["good", "great", "excellent", "amazing", "wonderful", "fantastic", "love", "happy"];
                    let negative_words = ["bad", "terrible", "awful", "horrible", "hate", "sad", "angry", "poor"];
                    
                    let lower_text = text.to_lowercase();
                    let mut score = 0.0;
                    
                    for word in positive_words.iter() {
                        if lower_text.contains(word) {
                            score += 1.0;
                        }
                    }
                    
                    for word in negative_words.iter() {
                        if lower_text.contains(word) {
                            score -= 1.0;
                        }
                    }
                    
                    let sentiment = if score > 0.0 {
                        "positive"
                    } else if score < 0.0 {
                        "negative"
                    } else {
                        "neutral"
                    };
                    
                    let mut result = HashMap::new();
                    result.insert("score".to_string(), Value::Number(score));
                    result.insert("sentiment".to_string(), Value::String(sentiment.to_string()));
                    Ok(Value::Dict(result))
                }
                _ => Err(RutenError::TypeError("sentiment() requires a string".to_string())),
            }
        }),
    );

    module
}