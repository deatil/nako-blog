use actix_web::{
    web, 
    Error,
    Responder,
    HttpResponse, 
    HttpRequest,
    Result,
    http::{
        header::ContentType,
        StatusCode,
    },
};

use captcha::Captcha;
use captcha::filters::{Noise, Wave, Dots};

use crate::nako::http as nako_http;
use crate::nako::global::{
    Session, AppState,
};

// 验证码
pub async fn captcha(
    session: Session, 
) -> Result<HttpResponse>{
    let mut c = Captcha::new();

    let c = c.add_chars(4) 
        .apply_filter(Noise::new(0.4))
        .apply_filter(Wave::new(2.0, 20.0).horizontal())
        .apply_filter(Wave::new(2.0, 20.0).vertical())
        .view(130, 48)
        .apply_filter(Dots::new(15));

    if let Some((data, png_data)) = c.as_tuple() {
        session.insert("auth_captcha", data).unwrap();

        // response
        Ok(HttpResponse::build(StatusCode::OK)
            .content_type(ContentType::png())
            .body(png_data))
    } else { 
        Ok(HttpResponse::build(StatusCode::OK)
            .content_type(ContentType::plaintext())
            .body("nodata".to_string()))
    }
}

// 登陆
pub async fn login(
    req: HttpRequest,
    session: Session, 
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    if let Some(_) = session.get::<u32>("login_id")? {
        let redirect_url: String = match req.url_for("admin.index", &[""]) {
            Ok(data) => data.into(),
            Err(_) => "/".into(),
        };
        
        return Ok(nako_http::redirect(redirect_url));
    } 

    let view = &state.view;

    let mut ctx = nako_http::view_ctx_new();
    ctx.insert("name", "hello");

    Ok(nako_http::view(view, "admin/auth/login.html", &ctx))
}

// 提交登陆
pub async fn login_check(
    session: Session, 
) -> Result<HttpResponse, Error> {
    if let Some(_) = session.get::<String>("auth_captcha")? {
        return Ok(nako_http::error_response_json("你已经登陆了".to_string()));
    }

    Ok(nako_http::success_response_json("登陆成功".to_string(), "".to_string()))
}

// 退出
pub async fn logout() -> impl Responder {
    format!("logout!")
}

