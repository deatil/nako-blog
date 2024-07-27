use crate::nako::config;

pub fn table_name(name: &str) -> String {
    let mut table_prefix = config::section::<String>("db", "table_prefix", "".to_string());

    table_prefix.push_str(name);

    table_prefix
}