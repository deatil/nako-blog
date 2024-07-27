use std::error::Error;

use actix_web::{
    http::header, 
    http::{
        StatusCode,
        header::ContentType, 
    },
    HttpResponse,
};

use crate::nako::{
    app,
    global::Serialize,
};

/// 状态枚举
#[derive(Serialize)]
pub enum Status {
    SUCCESS,
    FAIL,
}

/// 输出数据
#[derive(Serialize)]
pub struct ResponseEntity<T> {
    pub status: Status,
    pub code: i64,
    pub message: String,
    pub data: Option<T>,
}

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
    HttpResponse::build(StatusCode::OK)
        .json(res_body)
}

// 跳转
pub fn redirect(url: String) -> HttpResponse {
    HttpResponse::Found()
        .append_header((header::LOCATION, url))
        .finish()
}

// let mut ctx = view_data();
// ctx.insert("name", "hello");
pub fn view_data() -> tera::Context {
    tera::Context::new()
}

// 视图
pub fn view(view: &mut tera::Tera, name: &str, ctx: &tera::Context) -> HttpResponse {
    let err = format!("html is error.");

    let res_body: String = match view.render(name, ctx) {
        Ok(v) => v,
        Err(e) => {
            if app::is_debug() {
                let mut data = Vec::new();

                data.push(format!("#{}: {}", 1, e));

                let mut i = 2;
                
                let mut cause = e.source();
                while let Some(e) = cause {
                    data.push(format!("#{}: {}", i, e));
                    i += 1;

                    cause = e.source();
                }

                format!("html [{}] is error: <br />{}", name, data.join("<br />"))
            } else {
                err
            }
        },
    };

    html(res_body)
}

// 返回成功 json
pub fn success_json<T: Serialize>(message: &str, data: T) -> HttpResponse {
    let res_body: ResponseEntity<T> = ResponseEntity {
        status: Status::SUCCESS,
        code: 0,
        message: message.to_string(),
        data: Some(data),
    };

    json(res_body)
}

// 返回失败 json
pub fn error_json(message: &str) -> HttpResponse {
    let res_body: ResponseEntity<String> = ResponseEntity {
        status: Status::FAIL,
        code: 1,
        message: message.to_string(),
        data: Some("".to_string()),
    };

    json(res_body)
}

// 返回失败页面
pub fn error_html(t: &mut tera::Tera, message: &str, url: &str) -> HttpResponse {
    let mut new_url = url;
    if new_url == "back" {
        new_url = "javascript:history.back(-1);";
    }

    let mut ctx = view_data();
    ctx.insert("message", &message.to_string());
    ctx.insert("url", &new_url.to_string());

    view(t, "error.html", &ctx)
}
