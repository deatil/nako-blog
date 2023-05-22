use std::collections::HashMap;

use tera::Tera;
use tera::Result;
use serde_json::value::{
    from_value, 
    /*to_value, */ 
    Value,
};

use crate::nako::{
    env,
};

// 引入资源
pub fn assert(args: &HashMap<String, Value>) -> Result<Value> {
    let none = String::from("/static/none");

    match args.get("path") {
        Some(val) => match from_value::<String>(val.clone()) {
            Ok(v) =>  {
                let path = format!("/static/{}", v);

                Ok(serde_json::Value::String(path))
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
    let admin_prefix = env::get_env::<String>("ADMIN_PREFIX", "admin".to_string());

    let none_route = format!("/{}/index", admin_prefix);

    match args.get("url") {
        Some(val) => match from_value::<String>(val.clone()) {
            Ok(v) =>  {
                let v2 = format!("/{}/{}", admin_prefix, v);

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

// 头像
pub fn avatar(args: &HashMap<String, Value>) -> Result<Value> {
    let mut default_avatar = env::get_env::<String>("DEFAULT_AVATAR", "".to_string());
    default_avatar = format!("/static/{}", default_avatar);

    match args.get("path") {
        Some(val) => match from_value::<String>(val.clone()) {
            Ok(v) =>  {
                Ok(serde_json::Value::String(v))
            },
            Err(_) => {
                Ok(serde_json::Value::String(default_avatar))
            }
        },
        None => {
            Ok(serde_json::Value::String(default_avatar))
        },
    }
}

// 设置模板方法
pub fn set_fns(view: &mut Tera) {
    view.register_function("assert", assert);
    view.register_function("admin_url", admin_url);
    view.register_function("avatar", avatar);
}
