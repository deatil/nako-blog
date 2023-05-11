use actix_web::{
    web, 
    error, 
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
        .view(220, 100)
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
    if let Some(_login_id) = session.get::<u32>("login_id")? {
        let url = req.url_for("admin.index", &[""]).unwrap();

        let home_url = nako_http::redirect(url.as_str().to_string());
        
        return Ok(home_url);
    }

    let view = &state.view;

    let mut ctx = tera::Context::new();
    ctx.insert("name", "hello");

    let s = view.render("admin/auth/login.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(nako_http::html(s))
}

// 提交登陆
pub async fn login_check(
    session: Session, 
) -> Result<impl Responder, Error> {
    if let Some(auth_captcha) = session.get::<String>("auth_captcha")? {

    }

    Ok(format!("login_check!"))
}

// 退出
pub async fn logout() -> impl Responder {
    format!("logout!")
}

