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
