use std::env;
use std::str::FromStr;

// env
pub fn get_env<B: FromStr>(name: &str, def_val: &str) -> B 
    where <B as FromStr>::Err: std::fmt::Debug
{
    let conf_debug: String = match env::var(name) {
        Ok(data) => data.into(),
        Err(_) => def_val.into(),
    };

    let debug: B = conf_debug.parse().unwrap();
    
    debug
}

// 是否是调试模式
pub fn is_debug() -> bool {
    get_env::<bool>("APP_DEBUG", "false")
}

// 管理员ID
pub fn get_admin_id() -> u32 {
    get_env::<u32>("ADMIN_ID", "0")
}

// 获取日志等级
pub fn get_log_level(name: String) -> log::LevelFilter {
    match name.as_str() {
        "debug" => log::LevelFilter::Debug,
        "error" => log::LevelFilter::Error,
        "info" => log::LevelFilter::Info,
        "trace" => log::LevelFilter::Trace,
        "warn" => log::LevelFilter::Warn,
        _ => log::LevelFilter::Info,
    }
}

