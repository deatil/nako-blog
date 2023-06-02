use std::time::Duration;

use sea_orm::{
    DbErr,
    Database, 
    ConnectOptions,
    DatabaseConnection
};

pub use sea_orm;

use crate::nako::{
    app,
    config,
};

// 数据库连接
pub async fn connect() -> Result<DatabaseConnection, DbErr> {
    let db_url = config::section::<String>("db", "url", "".to_string());

    let max_connections = config::section::<u32>("db", "max_connections", 100);
    let min_connections = config::section::<u32>("db", "min_connections", 5);

    let connect_timeout = config::section::<u64>("db", "connect_timeout", 8);
    let acquire_timeout = config::section::<u64>("db", "acquire_timeout", 8);
    let idle_timeout = config::section::<u64>("db", "idle_timeout", 8);
    let max_lifetime = config::section::<u64>("db", "max_lifetime", 8);

    let logging = config::section::<bool>("db", "logging", false);

    let db_logging_level = config::section::<String>("db", "logging_level", "info".into());
    let logging_level = app::get_log_level(db_logging_level);

    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(max_connections)
        .min_connections(min_connections)
        .connect_timeout(Duration::from_secs(connect_timeout))
        .acquire_timeout(Duration::from_secs(acquire_timeout))
        .idle_timeout(Duration::from_secs(idle_timeout))
        .max_lifetime(Duration::from_secs(max_lifetime))
        .sqlx_logging(logging)
        .sqlx_logging_level(logging_level);
    
    let db = Database::connect(opt);

    db.await
}
