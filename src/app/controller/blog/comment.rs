use std::collections::HashMap;
use actix_web::{
    web,
    Responder,
};

/// 评论列表
pub async fn index(
    art_id: web::Path<String>
) -> impl Responder {
    format!("Hello {}!", art_id)
}

/// 添加评论
pub async fn create(
    web::Form(params): web::Form<HashMap<String, String>>,
) -> impl Responder {
    format!("Hello data!")
}


