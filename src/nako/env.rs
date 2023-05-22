use std::env;
use std::str::FromStr;
use std::default::Default;

// env
pub fn get_env<B: FromStr + Default>(name: &str, def_val: B) -> B 
    where <B as FromStr>::Err: std::fmt::Debug
{
    match env::var(name) {
        Ok(data) => {
            data.parse().unwrap_or_default()
        },
        Err(_) => def_val,
    }
}

