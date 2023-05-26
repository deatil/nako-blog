use actix_web::{
    web, 
    Result, 
    Error, 
    HttpResponse, 
};

use crate::nako::http as nako_http;
use crate::nako::global::{
    AppState,
};

// 首页
pub async fn index(
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let view = &state.view;

    let mut ctx = nako_http::view_data();
    ctx.insert("name", "hello");

    Ok(nako_http::view(view, "admin/user/index.html", &ctx))
}

// 详情
pub async fn detail(
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let view = &state.view;

    let mut ctx = nako_http::view_data();
    ctx.insert("name", "hello");

    Ok(nako_http::view(view, "admin/user/detail.html", &ctx))
}

// 添加
pub async fn create(
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let view = &state.view;

    let mut ctx = nako_http::view_data();
    ctx.insert("name", "hello");

    Ok(nako_http::view(view, "admin/user/create.html", &ctx))
}

// 添加保存
pub async fn create_save() -> Result<HttpResponse, Error> {
    Ok(nako_http::success_response_json("添加成功", ""))
}

// 更新
pub async fn update(
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let view = &state.view;

    let mut ctx = nako_http::view_data();
    ctx.insert("name", "hello");

    Ok(nako_http::view(view, "admin/user/update.html", &ctx))
}

// 更新保存
pub async fn update_save() -> Result<HttpResponse, Error> {
    Ok(nako_http::success_response_json("更新成功", ""))
}

// 删除
pub async fn delete() -> Result<HttpResponse, Error> {
    Ok(nako_http::success_response_json("删除成功", ""))
}
