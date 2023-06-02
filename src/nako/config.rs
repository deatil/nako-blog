use ini::Ini;
use std::str::FromStr;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static GLOBAL_CONF_FILE: Lazy<Mutex<String>> = Lazy::new(|| {
    Mutex::new(String::from("conf.ini"))
});

static GLOBAL_CONF: Lazy<Ini> = Lazy::new(|| {
    if let Ok(v) = GLOBAL_CONF_FILE.lock() {
        return Ini::load_from_file(v.as_str()).unwrap_or_default();
    }

    Ini::load_from_file("conf.ini").unwrap_or_default()
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

