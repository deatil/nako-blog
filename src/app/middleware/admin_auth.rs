use actix_web::{
    body::{
        BoxBody,
    },
    dev,
    dev::ServiceRequest,
    Error,
    http::Method,
};
use actix_web_lab::middleware::Next;

use crate::nako::http;

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

    let check = true;
    if !check {
        let method = req.method();

        if method == Method::POST {
            let res_body_data = http::error_response_json(String::from("错误"));
            
            return Ok(req.into_response(res_body_data));
        } else {
            let res_body_data = http::error_response_html(&req, String::from("错误"));
            
            return Ok(req.into_response(res_body_data));
        }
    }    

    return to_next(req, next).await;
}
