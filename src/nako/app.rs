use crate::nako::env::{
    get_env,
};

// 是否是调试模式
pub fn is_debug() -> bool {
    get_env::<bool>("APP_DEBUG", false)
}

// 管理员ID
pub fn get_admin_id() -> u32 {
    get_env::<u32>("ADMIN_ID", 0)
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

pub fn file_path(name: String) -> String {
    let path = get_env::<String>("UPLOAD_PATH", "./assert/upload/".to_string());

    format!("{}{}", path, name)
}

pub fn file_url(name: String) -> String {
    let path = get_env::<String>("UPLOAD_URL", "/upload/".to_string());

    format!("{}{}", path, name)
}

