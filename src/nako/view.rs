use std::collections::HashMap;

use tera::Result;
use serde_json::value::{from_value, /*to_value, */ Value};

// 引入资源
pub fn assert(args: &HashMap<String, Value>) -> Result<Value> {
    match args.get("path") {
        Some(val) => match from_value::<String>(val.clone()) {
            Ok(v) =>  {
                let mut v2 = String::from("/static/");
                v2.push_str(&v);

                Ok(serde_json::Value::String(v2))
            },
            Err(_) => {
                let v2 = String::from("/static/none");

                Ok(serde_json::Value::String(v2))
            }
        },
        None => {
            let v2 = String::from("/static/none");

            Ok(serde_json::Value::String(v2))
        },
    }

}
