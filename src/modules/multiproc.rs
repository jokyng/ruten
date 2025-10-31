use crate::error::RutenError;
use crate::interpreter::Value;
use std::collections::HashMap;

pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();

    // get number of cpu cores
    module.insert(
        "cpu_count".to_string(),
        Value::NativeFunction(|args| {
            if !args.is_empty() {
                return Err(RutenError::RuntimeError("cpu_count() takes no arguments".to_string()));
            }
            let count = num_cpus::get();
            Ok(Value::Number(count as f64))
        }),
    );

    // get number of physical cpu cores
    module.insert(
        "physical_cores".to_string(),
        Value::NativeFunction(|args| {
            if !args.is_empty() {
                return Err(RutenError::RuntimeError("physical_cores() takes no arguments".to_string()));
            }
            let count = num_cpus::get_physical();
            Ok(Value::Number(count as f64))
        }),
    );

    // execute function in parallel (placeholder - requires more complex implementation)
    module.insert(
        "parallel".to_string(),
        Value::NativeFunction(|args| {
            if args.len() < 2 {
                return Err(RutenError::RuntimeError(
                    "parallel() takes 2 arguments: function, data_list".to_string()
                ));
            }
            
            // note: full parallel execution would require thread-safe interpreter state
            // this is a simplified version that returns the input for now
            match &args[1] {
                Value::List(items) => Ok(Value::List(items.clone())),
                _ => Err(RutenError::TypeError("parallel() requires a list as second argument".to_string())),
            }
        }),
    );

    module
}