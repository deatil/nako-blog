use std::collections::HashMap;
use actix_web::{
    web, 
    Error, 
    Result, 
    HttpRequest,
    HttpResponse,
};

use crate::nako::{
    time,
    http as nako_http,
};
use crate::nako::global::{
    Validate,
    AppState,
    Deserialize,
};

use crate::app::entity::{
    self,
    guestbook as guestbook_entity,
};
use crate::app::model::{
    guestbook,
};

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct CreateValidate {
    #[validate(required(message = "你的称呼不能为空"))]
    name: Option<String>,
    #[validate(required(message = "你的联系邮箱不能为空"), email(message = "你的联系邮箱格式错误"))]
    email: Option<String>,
    #[validate(required(message = "你的留言内容不能为空"))]
    message: Option<String>,
}

/// 添加留言
pub async fn create(
    req: HttpRequest,
    state: web::Data<AppState>,
    web::Form(params): web::Form<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;

    let mut name: String = "".to_string();
    if let Some(n) = params.get("name") {
        name = n.parse::<String>().unwrap_or("".to_string());
    }
    let mut email: String = "".to_string();
    if let Some(e) = params.get("email") {
        email = e.parse::<String>().unwrap_or("".to_string());
    }
    let mut message: String = "".to_string();
    if let Some(m) = params.get("message") {
        message = m.parse::<String>().unwrap_or("".to_string());
    }

    let vali_data = CreateValidate{
        name:    Some(name.clone()),
        email:   Some(email.clone()),
        message: Some(message.clone()),
    };

    let vali = vali_data.validate();
    if vali.is_err() {
        return Ok(nako_http::error_response_json(format!("{}", vali.unwrap_err()).as_str()));
    }

    let add_time = time::now().timestamp();

    let mut ip: String = "0.0.0.0".to_string();
    if let Some(val) = req.peer_addr() {
        ip = val.ip().to_string();
    }

    let create_data = guestbook::GuestbookModel::create(db, guestbook_entity::Model{
            name:     name.clone(),
            message:  message.clone(),
            email:    Some(email.clone()),
            status:   Some(0),
            add_time: Some(add_time),
            add_ip:   Some(ip.clone()),
            ..entity::default()
        }).await;
    if create_data.is_err() {
        return Ok(nako_http::error_response_json("提交留言失败"));
    }

    Ok(nako_http::success_response_json("提交留言成功", ""))
}


