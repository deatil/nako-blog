use actix_web::{
    web, 
    Error, 
    Result, 
    http::{
        Method, 
    },
    HttpRequest,
    HttpResponse,
};

use crate::nako::{
    http as nako_http,
};
use crate::nako::global::{
    AppState,
};

/// 错误页面
pub async fn index(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let mut view = state.view.clone();

    let error = "页面不存在";

    let method = req.method();
    if method == Method::POST {
        return Ok(nako_http::error_response_json(error));
    }
    
    let mut ctx = nako_http::view_data();
    ctx.insert("message", &error.to_string());

    Ok(nako_http::view(&mut view, "admin/error/index.html", &ctx))
}