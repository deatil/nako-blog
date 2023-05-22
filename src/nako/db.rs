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
    env::get_env,
};

// 数据库连接
pub async fn connect() -> Result<DatabaseConnection, DbErr> {
    let db_url = get_env::<String>("DB_URL", "".to_string());

    let max_connections = get_env::<u32>("DB_MAX_CONNECTIONS", 100);
    let min_connections = get_env::<u32>("DB_MIN_CONNECTIONS", 5);

    let connect_timeout = get_env::<u64>("DB_CONNECT_TIMEOUT", 8);
    let acquire_timeout = get_env::<u64>("DB_ACQUIRE_TIMEOUT", 8);
    let idle_timeout = get_env::<u64>("DB_IDLE_TIMEOUT", 8);
    let max_lifetime = get_env::<u64>("DB_MAX_LIFETIME", 8);

    let logging = get_env::<bool>("DB_LOGGING", false);

    let db_logging_level = get_env::<String>("DB_LOGGING_LEVEL", "info".into());
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
