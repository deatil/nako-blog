// use log::{debug, error, info, trace, warn};
use std::time::SystemTime;
use std::env;
use humantime;

use crate::nako::{
    app,
};

// 初始化日志
// log::info!("Hello, world!");
pub fn setup_logger() -> Result<(), fern::InitError> {
    let mut f = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        });

    let rust_log = env::var("RUST_LOG").unwrap_or("info".into());
    let logging_level = app::get_log_level(rust_log);

    f = f.level(logging_level);
   
    if app::is_debug() {
        f = f.chain(std::io::stdout());
    }

    f.chain(fern::log_file("nako.log")?).apply()?;
    
    Ok(())
}
