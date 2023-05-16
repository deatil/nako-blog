use actix_web::{
    http::header, 
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use crate::nako::global::{
    Serialize,
    Status, ResponseEntity
};

// 返回成功 json
pub fn error_response_json(message: String) -> HttpResponse {
    let res_body: ResponseEntity<String> = ResponseEntity {
        status: Status::FAIL,
        code: 1,
        message: message,
        data: Some("".to_string()),
    };

    HttpResponse::build(StatusCode::OK).json(res_body)
}

// 返回失败 json
pub fn success_response_json<T: Serialize>(message: String, data: T) -> HttpResponse {
    let res_body: ResponseEntity<T> = ResponseEntity {
        status: Status::SUCCESS,
        code: 1,
        message: message,
        data: Some(data),
    };

    HttpResponse::build(StatusCode::OK).json(res_body)
}

// 返回失败页面
pub fn error_response_html(view: &tera::Tera, message: String, url: String) -> HttpResponse {
    let mut ctx = tera::Context::new();
    ctx.insert("message", &message);
    ctx.insert("url", &url);

    let res_body: String = match view.render("resp_error.html", &ctx) {
        Ok(data) => data.into(),
        Err(_) => "error html is err.".into(),
    };

    HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(res_body)
}

// 跳转
pub fn redirect(url: String) -> HttpResponse {
    let res_body_data = HttpResponse::Found()
        .header(header::LOCATION, url)
        .finish();

    res_body_data
}

// let mut ctx = view_ctx_new();
// ctx.insert("name", "hello");
pub fn view_ctx_new() -> tera::Context {
    tera::Context::new()
}

// 视图
pub fn view(view: &tera::Tera, name: &str, ctx: &tera::Context) -> HttpResponse {
    let res_body: String = match view.render(name, ctx) {
        Ok(data) => data.into(),
        Err(_) => "error html is err.".into(),
    };

    HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(res_body)
}

// 返回页面
pub fn html(body: String) -> HttpResponse {
    HttpResponse::build(StatusCode::OK).body(body)
}
