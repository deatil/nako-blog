use std::collections::HashMap;
use actix_web::{
    web, 
    Result, 
    Error, 
    HttpResponse, 
};

use crate::nako::http as nako_http;
use crate::nako::global::{
    AppState,
};

use crate::app::entity::{
    self,
    setting as setting_entity
};
use crate::app::model::{
    setting,
};
use crate::app::service;

// 首页
pub async fn index(
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;
    let view = &state.view;

    let settings = setting::SettingModel::find_all(db).await.unwrap_or_default();

    let mut data = HashMap::new();
    if settings.len() > 0 {
        for setting in settings {
            data.insert(setting.key, setting.value);
        }
    }
    
    let mut ctx = nako_http::view_data();
    ctx.insert("data", &data);

    Ok(nako_http::view(view, "admin/setting/index.html", &ctx))
}

// 保存设置
pub async fn setting_save(
    state: web::Data<AppState>,
    web::Form(params): web::Form<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;

    if params.len() > 0 {
        for (k, item) in params {
            // 更新
            let status = setting::SettingModel::update_by_key(db, k.as_str(), setting_entity::Model{
                    value: item.clone(),
                    ..entity::default()
                })
                .await;
            if status.is_err() {
                return Ok(nako_http::error_response_json("更新失败"));
            }
        }
    }

    service::setting::clear(&mut state.get_ref().clone()).await;

    Ok(nako_http::success_response_json("更新成功", ""))
}
