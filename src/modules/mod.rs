pub mod math;
pub mod strings;
pub mod json;
pub mod crypto;
pub mod http;
pub mod webhook;
pub mod weather;
pub mod crypto_prices;
pub mod stats;
pub mod utils;
pub mod subproc;
pub mod multiproc;
pub mod downloads;
pub mod ai;
pub mod vision;
pub mod nlp;
pub mod torrent;
pub mod apps;
pub mod format;
pub mod github;
pub mod news;
pub mod quotes;
pub mod jokes;
pub mod facts;
pub mod geo;
pub mod email;
pub mod database;
pub mod testing;
pub mod logger;
pub mod image;
pub mod blockchain;

use crate::error::RutenError;
use crate::interpreter::Value;

pub fn load_module(name: &str) -> Result<Value, RutenError> {
    let module_map = match name {
        "math" => math::create_module(),
        "strings" => strings::create_module(),
        "json" => json::create_module(),
        "crypto" => crypto::create_module(),
        "http" => http::create_module(),
        "webhook" => webhook::create_module(),
        "weather" => weather::create_module(),
        "crypto_prices" => crypto_prices::create_module(),
        "stats" => stats::create_module(),
        "utils" => utils::create_module(),
        "subproc" => subproc::create_module(),
        "multiproc" => multiproc::create_module(),
        "downloads" => downloads::create_module(),
        "ai" => ai::create_module(),
        "vision" => vision::create_module(),
        "nlp" => nlp::create_module(),
        "torrent" => torrent::create_module(),
        "apps" => apps::create_module(),
        "format" => format::create_module(),
        "github" => github::create_module(),
        "news" => news::create_module(),
        "quotes" => quotes::create_module(),
        "jokes" => jokes::create_module(),
        "facts" => facts::create_module(),
        "geo" => geo::create_module(),
        "email" => email::create_module(),
        "database" => database::create_module(),
        "testing" => testing::create_module(),
        "logger" => logger::create_module(),
        "image" => image::create_module(),
        "blockchain" => blockchain::create_module(),
        _ => {
            return Err(RutenError::ImportError(format!(
                "no module named '{}'",
                name
            )))
        }
    };

    Ok(Value::Module(module_map))
}