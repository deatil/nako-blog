use actix_web::{
    web, 
    dev::ServiceRequest,
    http::header, 
    http::StatusCode, 
    HttpResponse,
};
use crate::nako::global::{
    AppState, Serialize,
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
pub fn error_response_html(req: &ServiceRequest, message: String) -> HttpResponse {
    let state = req.app_data::<web::Data<AppState>>().unwrap();
    let view = &state.view;

    let mut ctx = tera::Context::new();
    ctx.insert("message", &message);

    let res_body = view.render("resp_error.html", &ctx).unwrap();    

    HttpResponse::build(StatusCode::OK).body(res_body)
}

// 跳转
pub fn redirect(url: String) -> HttpResponse {
    let res_body_data = HttpResponse::Found()
        .header(header::LOCATION, url)
        .finish();

    res_body_data
}

// 返回页面
pub fn html(body: String) -> HttpResponse {
    HttpResponse::build(StatusCode::OK).body(body)
}
