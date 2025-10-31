use crate::error::RutenError;
use crate::interpreter::Value;
use std::collections::HashMap;

pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();

    module.insert(
        "indent".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 2 {
                return Err(RutenError::RuntimeError("indent() takes 2 arguments: text, spaces".to_string()));
            }
            
            match (&args[0], &args[1]) {
                (Value::String(text), Value::Number(spaces)) => {
                    let indent = " ".repeat(*spaces as usize);
                    let indented: Vec<String> = text
                        .lines()
                        .map(|line| format!("{}{}", indent, line))
                        .collect();
                    Ok(Value::String(indented.join("\n")))
                }
                _ => Err(RutenError::TypeError("indent() requires string and number".to_string())),
            }
        }),
    );

    module.insert(
        "dedent".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("dedent() takes 1 argument".to_string()));
            }
            
            match &args[0] {
                Value::String(text) => {
                    let lines: Vec<&str> = text.lines().collect();
                    if lines.is_empty() {
                        return Ok(Value::String(String::new()));
                    }
                    
                    // find minimum indentation
                    let min_indent = lines
                        .iter()
                        .filter(|line| !line.trim().is_empty())
                        .map(|line| line.len() - line.trim_start().len())
                        .min()
                        .unwrap_or(0);
                    
                    let dedented: Vec<String> = lines
                        .iter()
                        .map(|line| {
                            if line.len() >= min_indent {
                                line[min_indent..].to_string()
                            } else {
                                line.to_string()
                            }
                        })
                        .collect();
                    
                    Ok(Value::String(dedented.join("\n")))
                }
                _ => Err(RutenError::TypeError("dedent() requires a string".to_string())),
            }
        }),
    );

    module.insert(
        "strip_comments".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("strip_comments() takes 1 argument".to_string()));
            }
            
            match &args[0] {
                Value::String(text) => {
                    let stripped: Vec<String> = text
                        .lines()
                        .filter(|line| {
                            let trimmed = line.trim();
                            !trimmed.starts_with('#') && !trimmed.starts_with("//")
                        })
                        .map(|line| line.to_string())
                        .collect();
                    Ok(Value::String(stripped.join("\n")))
                }
                _ => Err(RutenError::TypeError("strip_comments() requires a string".to_string())),
            }
        }),
    );

    module.insert(
        "minify".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("minify() takes 1 argument".to_string()));
            }
            
            match &args[0] {
                Value::String(text) => {
                    // simple minification: remove extra whitespace
                    let minified = text
                        .lines()
                        .map(|line| line.trim())
                        .filter(|line| !line.is_empty())
                        .collect::<Vec<_>>()
                        .join(" ");
                    Ok(Value::String(minified))
                }
                _ => Err(RutenError::TypeError("minify() requires a string".to_string())),
            }
        }),
    );

    module
}