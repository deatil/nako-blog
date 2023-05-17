use std::collections::HashMap;

use tera::Tera;
use tera::Result;
use serde_json::value::{from_value, /*to_value, */ Value};

use crate::nako::{
    app, 
};

// 引入资源
pub fn assert(args: &HashMap<String, Value>) -> Result<Value> {
    let none = String::from("/static/none");

    match args.get("path") {
        Some(val) => match from_value::<String>(val.clone()) {
            Ok(v) =>  {
                let mut v2 = String::from("/static/");
                v2.push_str(&v);

                Ok(serde_json::Value::String(v2))
            },
            Err(_) => {
                Ok(serde_json::Value::String(none))
            }
        },
        None => {
            Ok(serde_json::Value::String(none))
        },
    }

}

// 后台路由
pub fn admin_url(args: &HashMap<String, Value>) -> Result<Value> {
    let admin_prefix = app::get_env::<String>("ADMIN_PREFIX", "admin");

    let mut none_route = String::from("/");
    none_route.push_str(&admin_prefix);
    none_route.push_str(&"/index".to_string());

    match args.get("url") {
        Some(val) => match from_value::<String>(val.clone()) {
            Ok(v) =>  {
                let mut v2 = String::from("/");
                v2.push_str(&admin_prefix);
                v2.push_str(&"/".to_string());
                v2.push_str(&v);

                Ok(serde_json::Value::String(v2))
            },
            Err(_) => {
                Ok(serde_json::Value::String(none_route))
            }
        },
        None => {
            Ok(serde_json::Value::String(none_route))
        },
    }

}

// 设置模板方法
pub fn set_fns(view: &mut Tera) {
    view.register_function("assert", assert);
    view.register_function("admin_url", admin_url);
}
