use std::fs;
use std::path::Path;

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

pub fn upload_path(name: String) -> String {
    let path = get_env::<String>("UPLOAD_PATH", "./storage/upload".to_string());

    format!("{}{}", path, name)
}

pub fn upload_url(name: String) -> String {
    let path = get_env::<String>("UPLOAD_URL", "/upload".to_string());

    format!("{}{}", path, name)
}

// 附件路径
pub fn attach_path(name: String) -> String {
    let path = get_env::<String>("ATTACH_PATH", "./storage/attach".to_string());

    format!("{}{}", path, name)
}

// 列出模板
pub fn list_tpls_by_prefix(file_prefix: String) -> Vec<String> {
    let tpl = get_env::<String>("BLOG_TPL_PATH", "./assert/templates/blog".to_string());
    let theme = get_env::<String>("BLOG_THEME", "nako".to_string());

    let path = format!("{}/{}/", tpl, theme);

    let path_object = Path::new(&path);

    let mut tpl_list: Vec<String> = Vec::new();
    if path_object.exists() && path_object.is_dir() {
        if let Ok(entrys) = fs::read_dir(path) {
            for entry in entrys {
                if let Ok(file_entry) = entry {
                    let file_name = file_entry.file_name().into_string().unwrap_or_default();
        
                    if file_name.starts_with(&file_prefix) {
                        tpl_list.push(file_name);
                    }
                }
            }
        }
    }

    tpl_list
}

// 列表模板
pub fn list_tpls() -> Vec<String> {
    let file_prefix = "list".to_string();

    list_tpls_by_prefix(file_prefix)
}

// 详情模板
pub fn view_tpls() -> Vec<String> {
    let file_prefix = "view".to_string();

    list_tpls_by_prefix(file_prefix)
}

// 单页模板
pub fn page_tpls() -> Vec<String> {
    let file_prefix = "page".to_string();

    list_tpls_by_prefix(file_prefix)
}

// 模板路径
pub fn view_path(name: &str) -> String {
    let theme = get_env::<String>("BLOG_THEME", "nako".to_string());

    let path = format!("blog/{}/{}", theme, name);

    path
}