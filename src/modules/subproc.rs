use crate::error::RutenError;
use crate::interpreter::Value;
use std::collections::HashMap;
use subprocess::{Exec, Redirection};

pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();

    module.insert(
        "sh".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("sh() takes 1 argument".to_string()));
            }
            match &args[0] {
                Value::String(command) => {
                    let result = if cfg!(target_os = "windows") {
                        Exec::cmd("cmd")
                            .arg("/C")
                            .arg(command)
                            .stdout(Redirection::Pipe)
                            .stderr(Redirection::Merge)
                            .capture()
                    } else {
                        Exec::shell(command)
                            .stdout(Redirection::Pipe)
                            .stderr(Redirection::Merge)
                            .capture()
                    };
                    
                    match result {
                        Ok(capture) => {
                            let output = String::from_utf8_lossy(&capture.stdout).to_string();
                            Ok(Value::String(output.trim().to_string()))
                        }
                        Err(e) => Err(RutenError::RuntimeError(format!("command error: {}", e))),
                    }
                }
                _ => Err(RutenError::TypeError("sh() requires a string".to_string())),
            }
        }),
    );

    module.insert(
        "exec".to_string(),
        Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err(RutenError::RuntimeError("exec() takes at least 1 argument".to_string()));
            }
            
            let command = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err(RutenError::TypeError("exec() requires string arguments".to_string())),
            };
            
            let mut exec = Exec::cmd(&command);
            
            for arg in &args[1..] {
                if let Value::String(s) = arg {
                    exec = exec.arg(s);
                } else {
                    return Err(RutenError::TypeError("exec() requires string arguments".to_string()));
                }
            }
            
            let result = exec
                .stdout(Redirection::Pipe)
                .stderr(Redirection::Merge)
                .capture();
            
            match result {
                Ok(capture) => {
                    let output = String::from_utf8_lossy(&capture.stdout).to_string();
                    Ok(Value::String(output.trim().to_string()))
                }
                Err(e) => Err(RutenError::RuntimeError(format!("exec error: {}", e))),
            }
        }),
    );

    module
}