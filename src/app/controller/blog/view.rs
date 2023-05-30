use actix_web::{
    web,
    Responder,
};

/// 详情
pub async fn index(
    id: web::Path<String>
) -> impl Responder {
    format!("Hello {}!", id)
}

