use actix_web::{
    dev,
    web, 
    dev::ServiceRequest,
    Error,
    http::{
        Method, 
    },
    body::{
        BoxBody,
    },
};
use actix_web_lab::middleware::Next;

use crate::nako::{
    http as nako_http,
    global::AppState,
};
use crate::app::service;
use crate::app::service::setting;

//  检测网站是否开启
pub async fn check(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<dev::ServiceResponse<BoxBody>, Error> {
    let state = req.app_data::<web::Data<AppState>>().unwrap();
    let mut view = state.view.clone();

    let setting_data = setting::settings(&mut state.get_ref().clone()).await;

    let mut status: String = "0".to_string();
    if let Some(s) = setting_data.get("website_status") {
        status = s.parse::<String>().unwrap_or("".into());
    }

    if status.as_str() == "1" {
        return next.call(req).await;
    }

    let error = "网站关闭维护中...";

    let method = req.method();
    if method == Method::POST {
        return Ok(req.into_response(nako_http::error_response_json(error)));
    }

    let res_body_data = service::http::error_blog_html(&mut view, error, "");
    Ok(req.into_response(res_body_data))
}
