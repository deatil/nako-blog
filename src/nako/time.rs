pub use chrono::{
    Utc,
    DateTime,
    TimeZone,
    NaiveDateTime,
    offset::LocalResult,
};

use chrono_tz::Tz;
use chrono_tz::UTC;

use crate::nako::{
    config,
};

// 时区
pub fn timezone() -> Tz {
    let tz: Tz = config::section::<Tz>("time", "timezone", UTC);

    tz
}

// 当前时间
pub fn now() -> DateTime<Tz> {
    let tz = timezone();

    Utc::now().with_timezone(&tz)
}

// 解析时间
pub fn parse(d: &str) -> DateTime<Tz> {
    let tz = timezone();

    let date = NaiveDateTime::parse_from_str(d, "%Y-%m-%d %H:%M:%S").unwrap_or_default();

    tz.from_utc_datetime(&date)
}

// 解析时间戳
pub fn from_timestamp(t: i64) -> DateTime<Tz> {
    let tz = timezone();

    let date = NaiveDateTime::from_timestamp_opt(t, 0).unwrap_or_default();

    tz.from_utc_datetime(&date)
}
