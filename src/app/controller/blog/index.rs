use actix_web::{
    Responder,
};

/// 首页
pub async fn index() -> impl Responder {
    format!("Hello 123!")
}
