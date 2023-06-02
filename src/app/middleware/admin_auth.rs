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
use actix_session::SessionExt;

use crate::nako::http;
use crate::nako::global::{
    AppState
};

use crate::app::service;

// 过滤路由
const IGNORE_ROUTES: [&str; 2] = [
    "/admin/auth/captcha",
    "/admin/auth/login",
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

    let session = req.get_session();

    let mut check = false;

    let login_id = session.get::<u32>("login_id").unwrap_or_default().unwrap_or_default();
    if login_id > 0 {
        check = true;
    }

    if !check {
        let message = "请先登陆";

        let url: String = match req.request().url_for_static("admin.auth-login") {
            Ok(data) => data.into(),
            Err(_) => "/".into(),
        };

        if req.method() == Method::POST {
            let res_body_data = http::error_response_json(message);
            
            return Ok(req.into_response(res_body_data));
        } else {
            let res_body_data = service::http::error_admin_html(view, message, url.as_str());
            
            return Ok(req.into_response(res_body_data));
        }
    }    

    return to_next(req, next).await;
}
