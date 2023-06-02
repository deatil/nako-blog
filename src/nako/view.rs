use std::collections::HashMap;
use std::cell::RefCell;

use tera::{
    Tera,
    Error,
    Result
};
use serde_json::value::{
    from_value, 
    Value,
};

use actix_web::{
    Result as WebResult, 
    dev::ResourceMap, 
    test::TestRequest,
};

use crate::nako::{
    app,
    config,
    utils,
};

thread_local! {
    pub static ROUTES_KEY: RefCell<Option<ResourceMap>> = RefCell::new(None);
    pub static SETTINGS: RefCell<Option<HashMap<String, String>>> = RefCell::new(None);
}

// 引入资源
fn assert(args: &HashMap<String, Value>) -> Result<Value> {
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

// 图片预览
fn upload_url(args: &HashMap<String, Value>) -> Result<Value> {
    let none = String::from("/upload/none");

    match args.get("path") {
        Some(val) => match from_value::<String>(val.clone()) {
            Ok(v) =>  {
                let path = app::upload_url(v);

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

// 头像
fn avatar(args: &HashMap<String, Value>) -> Result<Value> {
    let mut default_avatar = config::section::<String>("app", "default_avatar", "".to_string());
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

// 链接
pub fn url_for(args: &HashMap<String, Value>) -> WebResult<Value, Error> {
    let name = args["name"].as_str().ok_or(Error::msg("`name` should be a string"))?;
    let empty_elements = Value::Array(vec![]);
    let elements_iter = args.get("elements").unwrap_or(&empty_elements)
        .as_array().ok_or(Error::msg("`elements` should be an array"))?.iter();
    let mut elements = vec![];
    for elem in elements_iter {
        elements.push(elem.as_str().ok_or(
                Error::msg("`elements` array should contain only strings")
            )?
        );
    }
    
    ROUTES_KEY.with(|routes| {
        let mut route_option = routes.borrow_mut();
        let routes = route_option.as_mut().ok_or(
            Error::msg("`url_for` should only be called in request context")
        )?;

        let fake_req = TestRequest::default().to_http_request();
        let url = routes.url_for(&fake_req, name, elements)
            .or(Err(Error::msg(format!("`{}` resource not found",name))))?;
        Ok(Value::String(url.path().replace("//", "/").to_string()))
    })
}

// 格式化大小
fn format_size(args: &HashMap<String, Value>) -> Result<Value> {
    let zero = "0".to_string();

    match args.get("size") {
        Some(val) => match from_value::<u64>(val.clone()) {
            Ok(v) =>  {
                let v2 = utils::format_lensize(v);

                Ok(serde_json::Value::String(v2))
            },
            Err(_) => {
                Ok(serde_json::Value::String(zero))
            }
        },
        None => {
            Ok(serde_json::Value::String(zero))
        },
    }
}

// 设置
pub fn settings(args: &HashMap<String, Value>) -> WebResult<Value, Error> {
    let name = args["name"].as_str().ok_or(Error::msg("`name` should be a string"))?;
    
    SETTINGS.with(|data| {
        let mut data_option = data.borrow_mut();
        let datas = data_option.as_mut().ok_or(
            Error::msg("`settings` should only be called in request context")
        )?;

        let mut res: String = "".to_string();
        if let Some(k) = datas.get(name) {
            res = k.parse::<String>().unwrap_or("".into());
        }

        Ok(Value::String(res))
    })
}

// 设置模板方法
pub fn set_fns(view: &mut Tera) {
    view.register_function("assert", assert);
    view.register_function("upload_url", upload_url);
    view.register_function("avatar", avatar);
    view.register_function("url_for", url_for);
    view.register_function("format_size", format_size);
    view.register_function("settings", settings);
}
