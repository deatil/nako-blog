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
    comment as comment_entity,
};
use crate::app::model::{
    art,
    comment,
};

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct CreateValidate {
    #[validate(required(message = "文章ID丢失"))]
    uuid: Option<String>,
    #[validate(required(message = "你的称呼不能为空"))]
    username: Option<String>,
    #[validate(required(message = "你的联系邮箱不能为空"), email(message = "你的联系邮箱格式错误"))]
    email: Option<String>,
    #[validate(required(message = "你的留言内容不能为空"))]
    content: Option<String>,
}

/// 添加评论
pub async fn create(
    req: HttpRequest,
    state: web::Data<AppState>,
    web::Form(params): web::Form<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;

    let mut uuid: String = "".to_string();
    if let Some(uu) = params.get("uuid") {
        uuid = uu.parse::<String>().unwrap_or("".to_string());
    }
    let mut username: String = "".to_string();
    if let Some(u) = params.get("username") {
        username = u.parse::<String>().unwrap_or("".to_string());
    }
    let mut email: String = "".to_string();
    if let Some(e) = params.get("email") {
        email = e.parse::<String>().unwrap_or("".to_string());
    }
    let mut content: String = "".to_string();
    if let Some(c) = params.get("message") {
        content = c.parse::<String>().unwrap_or("".to_string());
    }

    let vali_data = CreateValidate{
        uuid:     Some(uuid.clone()),
        username: Some(username.clone()),
        email:    Some(email.clone()),
        content:  Some(content.clone()),
    };

    let vali = vali_data.validate();
    if vali.is_err() {
        return Ok(nako_http::error_json(format!("{}", vali.unwrap_err()).as_str()));
    }

    // 文章详情
    let art = art::ArtModel::find_by_uuid(db, uuid.as_str())
        .await.unwrap_or_default().unwrap_or_default();
    if art.id == 0 {
        return Ok(nako_http::error_json("文章不存在"));
    }

    let add_time = time::now().timestamp();

    let mut ip: String = "0.0.0.0".to_string();
    if let Some(val) = req.peer_addr() {
        ip = val.ip().to_string();
    }

    let create_data = comment::CommentModel::create(db, comment_entity::Model{
            art_id:   art.id,
            username: username.clone(),
            email:    Some(email.clone()),
            content:  content.clone(),
            status:   Some(0),
            add_time: Some(add_time),
            add_ip:   Some(ip.clone()),
            ..entity::default()
        }).await;
    if create_data.is_err() {
        return Ok(nako_http::error_json("提交回复失败"));
    }

    Ok(nako_http::success_json("提交回复成功", ""))
}


