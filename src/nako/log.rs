use std::time::SystemTime;
use humantime;

use crate::nako::{
    app,
    config,
};

// 初始化日志
// log::info!("Hello, world!");
// log::{debug, error, info, trace, warn};
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

    let rust_log = config::section::<String>("app", "rust_log", "error".to_string());
    let logging_level = app::get_log_level(rust_log);

    f = f.level(logging_level);
   
    if app::is_debug() {
        f = f.chain(std::io::stdout());
    }

    f.chain(fern::log_file("nako.log")?).apply()?;
    
    Ok(())
}
