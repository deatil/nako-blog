use std::env;
use std::str::FromStr;

// env
pub fn get_env<B: FromStr>(name: &str, def_val: B) -> B 
    where <B as FromStr>::Err: std::fmt::Debug
{
    match env::var(name) {
        Ok(data) => {
            match data.parse::<B>() {
                Ok(timezone) => timezone,
                Err(_) => def_val,
            }
        },
        Err(_) => def_val,
    }
}

