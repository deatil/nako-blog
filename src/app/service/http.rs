use actix_web::{
    HttpResponse,
};

use crate::nako::{
    app,
    http::{
        view,
        view_data,
    },
};

// 返回失败页面
pub fn error_admin_html(t: &tera::Tera, message: &str, url: &str) -> HttpResponse {
    let mut new_url = url;
    if new_url == "back" {
        new_url = "javascript:history.back(-1);";
    }

    let mut ctx = view_data();
    ctx.insert("message", &message.to_string());
    ctx.insert("url", &new_url.to_string());

    view(t, "admin/error/index.html", &ctx)
}

// 返回博客失败页面
pub fn error_blog_html(t: &tera::Tera, message: &str, url: &str) -> HttpResponse {
    let mut new_url = url;
    if new_url == "back" {
        new_url = "javascript:history.back(-1);";
    }

    let mut ctx = view_data();
    ctx.insert("message", &message.to_string());
    ctx.insert("url", &new_url.to_string());

    view(t, app::view_path("error.html").as_str(), &ctx)
}

