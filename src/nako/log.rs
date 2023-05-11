// use log::{debug, error, info, trace, warn};
use std::time::SystemTime;
use humantime;

// 初始化日志
// info!("Hello, world!");
// warn!("Warning!");
// debug!("Now exiting.");
pub fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("nako.log")?)
        .apply()?;
    Ok(())
}