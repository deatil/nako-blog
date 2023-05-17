use chrono::prelude::*;

pub use chrono::offset::LocalResult;
pub use chrono::{
    DateTime,
    TimeZone,
    Utc,
};

// 当前时间
pub fn now() -> DateTime<Local> {
    Local::now()
}

// 当前时间
pub fn utc_now() -> DateTime<Utc> {
    Utc::now()
}

