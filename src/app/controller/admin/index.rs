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
use crate::app::data as app_data;

// 首页
pub async fn index(
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let view = &state.view;

    let ctx = tera::Context::new();

    Ok(nako_http::view(view, "admin/index/index.html", &ctx))
}

// 菜单
pub async fn menu() -> Result<HttpResponse, Error> {
    let data = app_data::menu::menus();

    Ok(nako_http::json(data))
}

// 控制台
pub async fn console(
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let view = &state.view;

    let mut ctx = tera::Context::new();

    let art_count = 12;
    ctx.insert("art_count", &art_count);

    Ok(nako_http::view(view, "admin/index/console.html", &ctx))
}
