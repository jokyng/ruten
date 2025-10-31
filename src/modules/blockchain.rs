use crate::error::RutenError;
use crate::interpreter::Value;
use std::collections::HashMap;

pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();
    // get bitcoin price
    module.insert(
        "btc_price".to_string(),
        Value::NativeFunction(|_args| {
            let url = "https://api.coinbase.com/v2/prices/BTC-USD/spot";
            let client = reqwest::blocking::Client::new();
            
            match client.get(url).send() {
                Ok(response) => {
                    let body = response.text().unwrap_or_default();
                    Ok(Value::String(body))
                }
                Err(e) => Err(RutenError::RuntimeError(format!("blockchain api error: {}", e))),
            }
        }),
    );

    // get ethereum price
    module.insert(
        "eth_price".to_string(),
        Value::NativeFunction(|_args| {
            let url = "https://api.coinbase.com/v2/prices/ETH-USD/spot";
            let client = reqwest::blocking::Client::new();
            
            match client.get(url).send() {
                Ok(response) => {
                    let body = response.text().unwrap_or_default();
                    Ok(Value::String(body))
                }
                Err(e) => Err(RutenError::RuntimeError(format!("blockchain api error: {}", e))),
            }
        }),
    );

    // get any crypto price
    module.insert(
        "price".to_string(),
        Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err(RutenError::RuntimeError(
                    "blockchain.price() requires symbol".to_string(),
                ));
            }

            let symbol = match &args[0] {
                Value::String(s) => s.to_uppercase(),
                _ => return Err(RutenError::TypeError("symbol must be a string".to_string())),
            };

            let url = format!("https://api.coinbase.com/v2/prices/{}-USD/spot", symbol);
            let client = reqwest::blocking::Client::new();
            
            match client.get(&url).send() {
                Ok(response) => {
                    let body = response.text().unwrap_or_default();
                    Ok(Value::String(body))
                }
                Err(e) => Err(RutenError::RuntimeError(format!("blockchain api error: {}", e))),
            }
        }),
    );

    module
}