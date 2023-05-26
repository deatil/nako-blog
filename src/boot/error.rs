use actix_web::{
    web, 
    Error,
    Result,
    Responder,
    HttpRequest,
    HttpResponse, 
    body::BoxBody,
    dev::ServiceResponse,
    http::{
        Method, 
    },
    middleware::{
        ErrorHandlerResponse, 
    },
    error::{
        InternalError, 
        PathError, 
        JsonPayloadError, 
        QueryPayloadError,
        UrlencodedError,
    },
};

use crate::nako::{
    app,
    http as nako_http,
    global::{
        AppState
    },
};

pub(crate) async fn app_default(req: HttpRequest) -> impl Responder {
    get_error_response(&req, "no page")
}

pub(crate) fn json_parser_error(
    err: JsonPayloadError,
    req: &HttpRequest,
) -> Error {
    let mut err_message = err.to_string();
    if !app::is_debug() {
        err_message = "json error".to_string();
    }

    let resp = get_error_response(&req, err_message.as_str());

    InternalError::from_response(err, resp).into()
}

pub(crate) fn form_parser_error(
    err: UrlencodedError,
    req: &HttpRequest,
) -> Error {
    let mut err_message = err.to_string();
    if !app::is_debug() {
        err_message = "form empty".to_string();
    }

    let resp = get_error_response(&req, err_message.as_str());

    InternalError::from_response(err, resp).into()
}

pub(crate) fn query_parser_error(
    err: QueryPayloadError,
    req: &HttpRequest,
) -> Error {
    let mut err_message = err.to_string();
    if !app::is_debug() {
        err_message = "query empty".to_string();
    }

    let resp = get_error_response(&req, err_message.as_str());

    InternalError::from_response(err, resp).into()
}

pub(crate) fn path_parser_error(
    err: PathError, 
    req: &HttpRequest,
) -> Error {
    let mut err_message = err.to_string();
    if !app::is_debug() {
        err_message = "path error".to_string();
    }

    let resp = get_error_response(&req, err_message.as_str());

    InternalError::from_response(err, resp).into()
}

// 404
pub(crate) fn not_found<B>(
    res: ServiceResponse<B>,
) -> Result<ErrorHandlerResponse<BoxBody>> {
    let req = res.request();

    let response = get_error_response(&req, "Page not found");

    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        res.into_parts().0,
        response.map_into_left_body(),
    )))
}

// 获取响应
fn get_error_response(req: &HttpRequest, error: &str) -> HttpResponse {
    if let Some(state) = req.app_data::<web::Data<AppState>>() {
        let view = &state.view;

        let method = req.method();
        
        if method == Method::POST {
            return nako_http::error_response_json(error);
        }
        
        return nako_http::error_response_html(view, error, "");
    }

    nako_http::text(error.to_string())
}
