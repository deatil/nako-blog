use actix_web::{
    web,
    Responder,
};

/// 单页
pub async fn index(
    slug: web::Path<String>
) -> impl Responder {
    format!("Hello {}!", slug)
}
