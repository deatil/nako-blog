use actix_web::{
    dev,
    web, 
    dev::ServiceRequest,
    Error,
    http::Method,
    body::{
        BoxBody,
    },
};
use actix_web_lab::middleware::Next;

use crate::nako::http;
use crate::nako::global::{
    AppState
};

// 过滤路由
const IGNORE_ROUTES: [&str; 2] = [
    "/auth/captcha",
    "/auth/login",
];

async fn to_next(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<dev::ServiceResponse<BoxBody>, Error> {
    // call next service
    let res = next.call(req).await?;

    Ok(res)
}

//  权限检测
pub async fn auth(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<dev::ServiceResponse<BoxBody>, Error> {
    for ignore_route in IGNORE_ROUTES {
        if req.path().starts_with(ignore_route) {
            return to_next(req, next).await;
        }
    }

    let state = req.app_data::<web::Data<AppState>>().unwrap();
    let view = &state.view;

    let check = true;
    if !check {
        let method = req.method();

        let message = "清先登陆".to_string();

        let url: String = match req.request().url_for("admin.auth-login", &[""]) {
            Ok(data) => data.into(),
            Err(_) => "/".into(),
        };

        if method == Method::POST {
            let res_body_data = http::error_response_json(message);
            
            return Ok(req.into_response(res_body_data));
        } else {
            let res_body_data = http::error_response_html(view, message, url);
            
            return Ok(req.into_response(res_body_data));
        }
    }    

    return to_next(req, next).await;
}
