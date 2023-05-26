use actix_web::{
    web, 
    Error,
    Result,
    HttpRequest,
    HttpResponse, 
    http::{
        StatusCode,
        header::ContentType,
    },
};
use serde::{
    Deserialize
};

use captcha::Captcha;
use captcha::filters::{Noise, Wave, Dots};

use crate::nako::auth as nako_auth;
use crate::nako::http as nako_http;
use crate::nako::global::{
    Session, 
    AppState,
    Validate,
};

use crate::app::model::{
    user,
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
        .view(260, 96)
        .apply_filter(Dots::new(15));

    if let Some((data, png_data)) = c.as_tuple() {
        if !session.insert("auth_captcha", data).is_err() {
            return Ok(HttpResponse::build(StatusCode::OK)
                .content_type(ContentType::png())
                .body(png_data));
        }
    }

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body("nodata".to_string()))
}

// 登陆
pub async fn login(
    req: HttpRequest,
    session: Session, 
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let login_id = session.get::<u32>("login_id").unwrap_or_default().unwrap_or_default();
    if login_id > 0 {
        let redirect_url: String = match req.url_for("admin.index", &[""]) {
            Ok(data) => data.into(),
            Err(_) => "/".into(),
        };

        return Ok(nako_http::redirect(redirect_url));
    }

    let view = &state.view;

    let ctx = nako_http::view_data();

    Ok(nako_http::view(view, "admin/auth/login.html", &ctx))
}

#[derive(Deserialize, Clone)]
pub struct LoginParams {
    name: String,
    password: String,
    captcha: String,
}

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct LoginValidate {
    #[validate(required(message = "账号不能为空"))]
    name: Option<String>,
    #[validate(required(message = "密码不能为空"))]
    password: Option<String>,
    #[validate(required(message = "验证码不能为空"), length(min = 4, message = "验证码位数错误"))]
    captcha: Option<String>,
}

// 提交登陆
pub async fn login_check(
    session: Session, 
    params: web::Form<LoginParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let login_id = session.get::<u32>("login_id").unwrap_or_default().unwrap_or_default();
    if login_id > 0 {
        return Ok(nako_http::error_response_json("你已经登陆了"));
    }

    let vali_data = LoginValidate{
        name: Some(params.name.clone()),
        password: Some(params.password.clone()),
        captcha: Some(params.captcha.clone()),
    };

    let vali = vali_data.validate();
    if vali.is_err() {
        return Ok(nako_http::error_response_json(format!("{}", vali.unwrap_err()).as_str()));
    }

    let auth_captcha = session.get::<String>("auth_captcha").unwrap_or_default().unwrap_or_default();
    if params.captcha.to_uppercase() != auth_captcha.to_uppercase() {
        return Ok(nako_http::error_response_json("验证码错误"));
    }

    let db = &state.db;
    let user_info = user::UserModel::find_user_by_name(db, params.name.as_str()).await.unwrap_or_default().unwrap_or_default();
    
    if user_info.id == 0 {
        return Ok(nako_http::error_response_json("账号或者密码错误"));
    }

    let pass = user_info.password.unwrap_or("".to_string());

    if !nako_auth::password_verify(params.password.as_str(), pass.as_str()) {
        return Ok(nako_http::error_response_json("账号或者密码错误"));
    }

    let status = user_info.status.unwrap_or(0);
    if status == 0 {
        return Ok(nako_http::error_response_json("账号不存在或者已被禁用"));
    }

    if session.insert("login_id", user_info.id).is_err() {
        return Ok(nako_http::error_response_json("登陆失败"));
    }

    Ok(nako_http::success_response_json("登陆成功", ""))
}

// 退出
pub async fn logout(
    req: HttpRequest,
    session: Session, 
) -> Result<HttpResponse, Error> {
    let login_id = session.get::<u32>("login_id").unwrap_or_default().unwrap_or_default();
    if login_id > 0 {
        session.remove("login_id");
    }

    let redirect_url: String = match req.url_for("admin.auth-login", &[""]) {
        Ok(data) => data.into(),
        Err(_) => "/".into(),
    };
    
    return Ok(nako_http::redirect(redirect_url));
}

