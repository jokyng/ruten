use crate::error::RutenError;
use crate::interpreter::Value;
use std::collections::HashMap;

pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();

    // get current ip location
    module.insert(
        "myip".to_string(),
        Value::NativeFunction(|_args| {
            let url = "https://ipapi.co/json/";
            let client = reqwest::blocking::Client::new();
            
            match client.get(url).send() {
                Ok(response) => {
                    let body = response.text().unwrap_or_default();
                    Ok(Value::String(body))
                }
                Err(e) => Err(RutenError::RuntimeError(format!("geo api error: {}", e))),
            }
        }),
    );

    // lookup ip address
    module.insert(
        "lookup".to_string(),
        Value::NativeFunction(|args| {
            if args.is_empty() {
                return Err(RutenError::RuntimeError(
                    "geo.lookup() requires ip address".to_string(),
                ));
            }

            let ip = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err(RutenError::TypeError("ip must be a string".to_string())),
            };

            let url = format!("https://ipapi.co/{}/json/", ip);
            let client = reqwest::blocking::Client::new();
            
            match client.get(&url).send() {
                Ok(response) => {
                    let body = response.text().unwrap_or_default();
                    Ok(Value::String(body))
                }
                Err(e) => Err(RutenError::RuntimeError(format!("geo api error: {}", e))),
            }
        }),
    );

    // calculate distance between coordinates
    module.insert(
        "distance".to_string(),
        Value::NativeFunction(|args| {
            if args.len() < 4 {
                return Err(RutenError::RuntimeError(
                    "geo.distance() requires lat1, lon1, lat2, lon2".to_string(),
                ));
            }

            let lat1 = match &args[0] {
                Value::Number(n) => *n,
                _ => return Err(RutenError::TypeError("latitude must be a number".to_string())),
            };

            let lon1 = match &args[1] {
                Value::Number(n) => *n,
                _ => return Err(RutenError::TypeError("longitude must be a number".to_string())),
            };

            let lat2 = match &args[2] {
                Value::Number(n) => *n,
                _ => return Err(RutenError::TypeError("latitude must be a number".to_string())),
            };

            let lon2 = match &args[3] {
                Value::Number(n) => *n,
                _ => return Err(RutenError::TypeError("longitude must be a number".to_string())),
            };

            // haversine formula
            let r = 6371.0; // earth radius in km
            let dlat = (lat2 - lat1).to_radians();
            let dlon = (lon2 - lon1).to_radians();
            let a = (dlat / 2.0).sin().powi(2)
                + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
            let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
            let distance = r * c;

            Ok(Value::Number(distance))
        }),
    );

    module
}