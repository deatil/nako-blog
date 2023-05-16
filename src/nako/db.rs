use std::env;
use std::time::Duration;

use sea_orm::{
    DbErr,
    Database, 
    ConnectOptions,
    DatabaseConnection
};

pub use sea_orm;

use crate::nako::app;

// 数据库连接
pub async fn connect() -> Result<DatabaseConnection, DbErr> {
    let db_url = match env::var("DB_URL") {
        Ok(data) => data.into(),
        Err(_) => "".into(),
    };

    let db_max_connections: String = match env::var("DB_MAX_CONNECTIONS") {
        Ok(data) => data.into(),
        Err(_) => "100".into(),
    };
    let max_connections: u32 = db_max_connections.parse().unwrap();

    let db_min_connections: String = match env::var("DB_MIN_CONNECTIONS") {
        Ok(data) => data.into(),
        Err(_) => "5".into(),
    };
    let min_connections: u32 = db_min_connections.parse().unwrap();

    let db_connect_timeout: String = match env::var("DB_CONNECT_TIMEOUT") {
        Ok(data) => data.into(),
        Err(_) => "8".into(),
    };
    let connect_timeout: u64 = db_connect_timeout.parse().unwrap();

    let db_acquire_timeout: String = match env::var("DB_ACQUIRE_TIMEOUT") {
        Ok(data) => data.into(),
        Err(_) => "8".into(),
    };
    let acquire_timeout: u64 = db_acquire_timeout.parse().unwrap();

    let db_idle_timeout: String = match env::var("DB_IDLE_TIMEOUT") {
        Ok(data) => data.into(),
        Err(_) => "8".into(),
    };
    let idle_timeout: u64 = db_idle_timeout.parse().unwrap();

    let db_max_lifetime: String = match env::var("DB_MAX_LIFETIME") {
        Ok(data) => data.into(),
        Err(_) => "8".into(),
    };
    let max_lifetime: u64 = db_max_lifetime.parse().unwrap();

    let db_logging: String = match env::var("DB_LOGGING") {
        Ok(data) => data.into(),
        Err(_) => "false".into(),
    };
    let logging: bool = db_logging.parse().unwrap();

    let db_logging_level: String = match env::var("DB_LOGGING_LEVEL") {
        Ok(data) => data.into(),
        Err(_) => "info".into(),
    };
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
