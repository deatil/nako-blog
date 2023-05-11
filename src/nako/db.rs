use std::time::Duration;

use sea_orm::{
    DbErr,
    Database, 
    ConnectOptions,
    DatabaseConnection
};

pub use sea_orm;

// 数据库连接
pub async fn connect(url: String) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);
    
    let db = Database::connect(opt);

    db.await
}

