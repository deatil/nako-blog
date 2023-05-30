use actix_web::{
    http::header, 
    http::{
        StatusCode,
        header::ContentType, 
    },
    HttpResponse,
};

use crate::nako::app;
use crate::nako::global::{
    Serialize,
    Status, ResponseEntity
};

// 返回文字
pub fn text(body: String) -> HttpResponse {
    HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())    
        .body(body)
}

// 返回页面
pub fn html(body: String) -> HttpResponse {
    HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())    
        .body(body)
}

// 返回 json
pub fn json<T: Serialize>(res_body: T) -> HttpResponse {
    HttpResponse::build(StatusCode::OK).json(res_body)
}

// 跳转
pub fn redirect(url: String) -> HttpResponse {
    let res_body_data = HttpResponse::Found()
        .append_header((header::LOCATION, url))
        .finish();

    res_body_data
}

// 返回失败 json
pub fn success_response_json<T: Serialize>(message: &str, data: T) -> HttpResponse {
    let res_body: ResponseEntity<T> = ResponseEntity {
        status: Status::SUCCESS,
        code: 0,
        message: message.to_string(),
        data: Some(data),
    };

    json(res_body)
}

// 返回成功 json
pub fn error_response_json(message: &str) -> HttpResponse {
    let res_body: ResponseEntity<String> = ResponseEntity {
        status: Status::FAIL,
        code: 1,
        message: message.to_string(),
        data: Some("".to_string()),
    };

    json(res_body)
}

// 返回失败页面
pub fn error_response_html(view: &tera::Tera, message: &str, url: &str) -> HttpResponse {
    let mut new_url = url;
    if new_url == "back" {
        new_url = "javascript:history.back(-1);";
    }

    let mut ctx = view_data();
    ctx.insert("message", &message.to_string());
    ctx.insert("url", &new_url.to_string());

    let res_body: String = view.render("error.html", &ctx)
        .unwrap_or("html [error.html] is error.".into());

    html(res_body)
}

// let mut ctx = view_data();
// ctx.insert("name", "hello");
pub fn view_data() -> tera::Context {
    tera::Context::new()
}

// 视图
pub fn view(view: &tera::Tera, name: &str, ctx: &tera::Context) -> HttpResponse {
    let err = format!("html is error.");

    let res_body: String = match view.render(name, ctx) {
        Ok(v) => v,
        Err(e) => {
            if app::is_debug() {
                format!("html [{}] is error: {}", name, e)
            } else {
                err
            }
        },
    };

    html(res_body)
}
