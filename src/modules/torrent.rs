use crate::error::RutenError;
use crate::interpreter::Value;
use std::collections::HashMap;

pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();

    module.insert(
        "download".to_string(),
        Value::NativeFunction(|args| {
            if args.len() < 2 {
                return Err(RutenError::RuntimeError("download() takes 2 arguments: magnet_url, output_path".to_string()));
            }
            
            match (&args[0], &args[1]) {
                (Value::String(url), Value::String(output)) => {
                    // note: full torrent implementation requires external libraries
                    // this is a placeholder that validates inputs
                    if !url.starts_with("magnet:?") && !url.ends_with(".torrent") {
                        return Err(RutenError::RuntimeError(
                            "download() requires valid magnet link or .torrent file".to_string()
                        ));
                    }
                    
                    Err(RutenError::RuntimeError(
                        format!("torrent.download() not yet fully implemented. would download {} to {}", url, output)
                    ))
                }
                _ => Err(RutenError::TypeError("download() requires url and output path strings".to_string())),
            }
        }),
    );

    module.insert(
        "info".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("info() takes 1 argument: torrent_file".to_string()));
            }
            
            match &args[0] {
                Value::String(_path) => {
                    Err(RutenError::RuntimeError("torrent.info() not yet implemented".to_string()))
                }
                _ => Err(RutenError::TypeError("info() requires a string path".to_string())),
            }
        }),
    );

    module.insert(
        "status".to_string(),
        Value::NativeFunction(|args| {
            if args.len() != 1 {
                return Err(RutenError::RuntimeError("status() takes 1 argument: download_id".to_string()));
            }
            
            Err(RutenError::RuntimeError("torrent.status() not yet implemented".to_string()))
        }),
    );

    module
}