use actix_web::{
    web,
    Responder,
};

/// 标签
pub async fn index(
    name: web::Path<String>
) -> impl Responder {
    format!("Hello {}!", name)
}


