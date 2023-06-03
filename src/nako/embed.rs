use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assert/static/"]
pub struct Static;

#[derive(RustEmbed)]
#[folder = "assert/templates/"]
pub struct Templates;

#[derive(RustEmbed)]
#[folder = "assert/config/"]
pub struct Config;

/// 获取模板数据
pub fn get_tpl_data(name: &str) -> String {
    let data = match Templates::get(name) {
        Some(v) => v.data.into_owned(),
        None => "html err".into(),
    };
    let data_str = std::str::from_utf8(data.as_ref()).unwrap_or("");

    data_str.to_string()
}
