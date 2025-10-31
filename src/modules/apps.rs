use crate::error::RutenError;
use crate::interpreter::Value;
use std::collections::HashMap;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();

    module.insert(
        "color".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 2 {
                return Err(RutenError::RuntimeError("color() takes 2 arguments: text, color".to_string()));
            }
            
            match (&args[0], &args[1]) {
                (Value::String(text), Value::String(color)) => {
                    let colored_text = match color.as_str() {
                        "red" => text.red().to_string(),
                        "green" => text.green().to_string(),
                        "blue" => text.blue().to_string(),
                        "yellow" => text.yellow().to_string(),
                        "magenta" => text.magenta().to_string(),
                        "cyan" => text.cyan().to_string(),
                        "white" => text.white().to_string(),
                        "black" => text.black().to_string(),
                        _ => text.clone(),
                    };
                    Ok(Value::String(colored_text))
                }
                _ => Err(RutenError::TypeError("color() requires two strings".to_string())),
            }
        }),
    );

    module.insert(
        "bold".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("bold() takes 1 argument".to_string()));
            }
            
            match &args[0] {
                Value::String(text) => Ok(Value::String(text.bold().to_string())),
                _ => Err(RutenError::TypeError("bold() requires a string".to_string())),
            }
        }),
    );

    module.insert(
        "progress".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("progress() takes 1 argument".to_string()));
            }
            
            match &args[0] {
                Value::Number(total) => {
                    let pb = ProgressBar::new(*total as u64);
                    pb.set_style(
                        ProgressStyle::default_bar()
                            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
                            .unwrap()
                            .progress_chars("=>-")
                    );
                    
                    // simulate progress
                    for _ in 0..(*total as u64) {
                        pb.inc(1);
                        std::thread::sleep(Duration::from_millis(50));
                    }
                    pb.finish_with_message("done");
                    
                    Ok(Value::None)
                }
                _ => Err(RutenError::TypeError("progress() requires a number".to_string())),
            }
        }),
    );

    module.insert(
        "clear".to_string(),
        Value::NativeFunction(|args| {
            if !args.is_empty() {
                return Err(RutenError::RuntimeError("clear() takes no arguments".to_string()));
            }
            
            // clear terminal
            if cfg!(target_os = "windows") {
                std::process::Command::new("cmd")
                    .args(&["/C", "cls"])
                    .status()
                    .ok();
            } else {
                std::process::Command::new("clear")
                    .status()
                    .ok();
            }
            
            Ok(Value::None)
        }),
    );

    module
}