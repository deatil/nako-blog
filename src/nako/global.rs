use tera::Tera;
use sea_orm::DatabaseConnection;

pub use actix_session::Session;
pub use serde::{
    Serialize,
    Deserialize,
};
use redis::aio::ConnectionManager;

pub use validator::{
    Validate, 
    ValidationError,
};

#[derive(Clone)]
pub struct AppState {
    pub view: Tera,
    pub db: DatabaseConnection,
    pub redis: ConnectionManager,
}

// 状态枚举
#[derive(Serialize)]
pub enum Status {
    SUCCESS,
    FAIL,
}

#[derive(Serialize)]
pub struct ResponseEntity<T> {
    pub status: Status,
    pub code: i64,
    pub message: String,
    pub data: Option<T>,
}
