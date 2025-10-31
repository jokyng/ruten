use crate::error::RutenError;
use crate::interpreter::Value;
use std::collections::HashMap;
use reqwest::blocking;

pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();
    // get current price for a cryptocurrency
    module.insert(
        "price".to_string(),
        Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err(RutenError::RuntimeError("price() takes 1-2 arguments: symbol, [currency]".to_string()));
            }
            
            match &args[0] {
                Value::String(symbol) => {
                    let currency = if args.len() > 1 {
                        match &args[1] {
                            Value::String(c) => c.to_uppercase(),
                            _ => "USD".to_string(),
                        }
                    } else {
                        "USD".to_string()
                    };
                    
                    let url = format!("https://api.coinbase.com/v2/prices/{}-{}/spot", 
                                    symbol.to_uppercase(), currency);
                    
                    let response = blocking::get(&url)
                        .map_err(|e| RutenError::RuntimeError(format!("crypto api error: {}", e)))?;
                    
                    let text = response.text()
                        .map_err(|e| RutenError::RuntimeError(format!("response error: {}", e)))?;
                    
                    Ok(Value::String(text))
                }
                _ => Err(RutenError::TypeError("price() requires a string symbol".to_string())),
            }
        }),
    );

    // get multiple crypto prices at once
    module.insert(
        "prices".to_string(),
        Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err(RutenError::RuntimeError("prices() takes 1 argument: list of symbols".to_string()));
            }
            
            match &args[0] {
                Value::List(symbols) => {
                    let mut results = Vec::new();
                    
                    for symbol in symbols {
                        if let Value::String(sym) = symbol {
                            let url = format!("https://api.coinbase.com/v2/prices/{}-USD/spot", 
                                            sym.to_uppercase());
                            
                            match blocking::get(&url) {
                                Ok(response) => {
                                    if let Ok(text) = response.text() {
                                        results.push(Value::String(text));
                                    }
                                }
                                Err(_) => continue,
                            }
                        }
                    }
                    
                    Ok(Value::List(results))
                }
                _ => Err(RutenError::TypeError("prices() requires a list of symbols".to_string())),
            }
        }),
    );

    // get exchange rate between two currencies
    module.insert(
        "exchange".to_string(),
        Value::NativeFunction(|args| {
            if args.len() < 2 {
                return Err(RutenError::RuntimeError("exchange() takes 2 arguments: from, to".to_string()));
            }
            
            match (&args[0], &args[1]) {
                (Value::String(from), Value::String(_to)) => {
                    let url = format!("https://api.coinbase.com/v2/exchange-rates?currency={}", 
                                    from.to_uppercase());
                    
                    let response = blocking::get(&url)
                        .map_err(|e| RutenError::RuntimeError(format!("exchange api error: {}", e)))?;
                    
                    let text = response.text()
                        .map_err(|e| RutenError::RuntimeError(format!("response error: {}", e)))?;
                    
                    Ok(Value::String(text))
                }
                _ => Err(RutenError::TypeError("exchange() requires two string currencies".to_string())),
            }
        }),
    );

    module
}