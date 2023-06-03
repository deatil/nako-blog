use ini::Ini;
use std::path;
use std::str::FromStr;
use std::sync::Mutex;
use once_cell::sync::Lazy;

use crate::nako::{
    embed,
};

static GLOBAL_CONF_FILE: Lazy<Mutex<String>> = Lazy::new(|| {
    Mutex::new(String::from("conf.ini"))
});

static GLOBAL_CONF: Lazy<Ini> = Lazy::new(|| {
    if let Ok(v) = GLOBAL_CONF_FILE.lock() {
        if path::Path::new(v.clone().as_str()).exists() {
            return Ini::load_from_file(v.clone().as_str()).unwrap_or_default();
        }
    }

    let conf = match embed::Config::get("conf.ini") {
        Some(v) => v.data.into_owned(),
        None => todo!(),
    };
    let conf_str = std::str::from_utf8(conf.as_ref()).unwrap_or("");

    Ini::load_from_str(conf_str).unwrap_or_default()
});

/// 初始化
pub fn new() -> Ini {
    let conf = Ini::new();

    conf
}

pub fn load_from_str(data: &str) -> Ini {
    Ini::load_from_str(data).unwrap_or_default()
}

pub fn load_from_file(file: &str) -> Ini {
    Ini::load_from_file(file).unwrap_or_default()
}

// env
pub fn section<B: FromStr>(section: &str, key: &str, def_val: B) -> B 
    where <B as FromStr>::Err: std::fmt::Debug
{
    let ini = &GLOBAL_CONF;

    match ini.get_from(Some(section), key){
        Some(data) => {
            match data.parse::<B>() {
                Ok(v) => v,
                Err(_) => def_val,
            }
        },
        _ => def_val,
    }
}

