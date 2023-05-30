use std::collections::HashMap;
use actix_web::{
    web,
    Responder,
};

/// 分类全部列表
pub async fn index(
    web::Query(params): web::Query<HashMap<String, String>>,
) -> impl Responder {
    if let Some(page) = params.get("page") {
        return format!("page = {}!", page);
    }

    format!("Hello data!")
}

/// 根据名称查询分类
pub async fn name(
    name: web::Path<String>,
    web::Query(params): web::Query<HashMap<String, String>>,
) -> impl Responder {
    format!("Hello name {}!", name)
}


