use std::collections::HashMap;
use actix_web::{
    web, 
    Result, 
    Error, 
    HttpRequest,
    HttpResponse, 
};

use crate::nako::{
    time,
    http as nako_http,
};
use crate::nako::global::{
    AppState,
    Validate,
    Serialize,
    Deserialize,
};

use crate::app::service::http;
use crate::app::entity::{
    self,
    friendlink as friendlink_entity
};
use crate::app::model::{
    friendlink,
};

// 首页
pub async fn index(
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let mut view = state.view.clone();

    let ctx = nako_http::view_data();

    Ok(nako_http::view(&mut view, "admin/friendlink/index.html", &ctx))
}

// ==========================

#[derive(Serialize)]
pub struct ListData {
    list: Vec<friendlink_entity::Model>,
    count: u64,
}

#[derive(Deserialize)]
pub struct ListQuery {
    page: u64,
    limit: u64,

    title: Option<String>,
    url: Option<String>,
    target: Option<String>,
    status: Option<i32>,
}

// 数据列表
pub async fn list(
    state: web::Data<AppState>,
    query: web::Query<ListQuery>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;

    let page: u64 = query.page;
    let per_page: u64 = query.limit;

    let search_where = friendlink::FriendlinkWhere{
        title: query.title.clone(),
        url: query.url.clone(),
        target: query.target.clone(),
        status: query.status,
    };
    let search_where = search_where.format();

    let (list, _num_pages) = friendlink::FriendlinkModel::search_in_page(
            db, 
            page, 
            per_page, 
            search_where.clone(),
        )
        .await.unwrap_or_default();
    let count = friendlink::FriendlinkModel::search_count(db, search_where.clone())
        .await.unwrap_or(0);

    let res = ListData{
        list: list,
        count: count,
    };

    Ok(nako_http::success_json("获取成功", res))
}

// ==========================

#[derive(Deserialize)]
pub struct DetailQuery {
    id: u32,
}

// 详情
pub async fn detail(
    state: web::Data<AppState>,
    query: web::Query<DetailQuery>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;
    let mut view = state.view.clone();

    if query.id == 0 {
        return Ok(http::error_admin_html(&mut view, "ID不能为空", ""));
    }

    let data = friendlink::FriendlinkModel::find_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if data.id == 0 {
        return Ok(http::error_admin_html(&mut view, "链接不存在", ""));
    }

    let mut ctx = nako_http::view_data();
    ctx.insert("data", &data);

    Ok(nako_http::view(&mut view, "admin/friendlink/detail.html", &ctx))
}

// ==========================

// 添加
pub async fn create(
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let mut view = state.view.clone();

    let ctx = nako_http::view_data();

    Ok(nako_http::view(&mut view, "admin/friendlink/create.html", &ctx))
}

// 表单数据
#[derive(Deserialize)]
pub struct CreateForm {
    title: String,
    url: String,
    status: i32,
}

// 添加保存
pub async fn create_save(
    req: HttpRequest,
    state: web::Data<AppState>,
    params: web::Form<CreateForm>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;

    if params.title.as_str() == "" {
        return Ok(nako_http::error_json("名称不能为空"));
    }
    if params.url.as_str() == "" {
        return Ok(nako_http::error_json("链接不能为空"));
    }
    if params.status != 0 && params.status != 1  {
        return Ok(nako_http::error_json("状态不能为空"));
    }

    let add_time = time::now().timestamp();

    let mut ip: String = "0.0.0.0".to_string();
    if let Some(val) = req.peer_addr() {
        ip = val.ip().to_string();
    }

    let create_data = friendlink::FriendlinkModel::create(db, friendlink_entity::Model{
            title:    params.title.clone(),
            url:      params.url.clone(),
            target:   Some("_blank".to_string()),
            icon:     Some("".to_string()),
            sort:     Some(100),
            status:   Some(params.status),
            add_time: Some(add_time),
            add_ip:   Some(ip.clone()),
            ..entity::default()
        }).await;
    if create_data.is_ok() {
        return Ok(nako_http::success_json("添加成功", ""));
    }

    Ok(nako_http::error_json("添加失败"))
}

// ==========================

#[derive(Deserialize)]
pub struct UpdateQuery {
    id: u32,
}

// 更新
pub async fn update(
    state: web::Data<AppState>,
    query: web::Query<UpdateQuery>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;
    let mut view = state.view.clone();

    if query.id == 0 {
        return Ok(http::error_admin_html(&mut view, "ID不能为空", ""));
    }

    let info = friendlink::FriendlinkModel::find_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if info.id == 0 {
        return Ok(http::error_admin_html(&mut view, "链接不存在", ""));
    }

    let mut targets = HashMap::new();
    targets.insert("_blank", "跳出页面");
    targets.insert("_self", "当前页面");

    let mut ctx = nako_http::view_data();
    ctx.insert("data", &info);
    ctx.insert("targets", &targets);

    Ok(nako_http::view(&mut view, "admin/friendlink/update.html", &ctx))
}

// 表单数据
#[derive(Deserialize)]
pub struct UpdateForm {
    title: String,
    url: String,
    target: String,
    icon: String,
    sort: i32,
    status: i32,
}

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct UpdateValidate {
    #[validate(required(message = "名称不能为空"))]
    title: Option<String>,
    #[validate(required(message = "链接不能为空"))]
    url: Option<String>,
    #[validate(required(message = "跳转方式不能为空"))]
    target: Option<String>,
    #[validate(required(message = "排序不能为空"))]
    sort: Option<i32>,
    #[validate(required(message = "状态不能为空"))]
    status: Option<i32>,
}

// 更新保存
pub async fn update_save(
    state: web::Data<AppState>,
    query: web::Query<UpdateQuery>,
    params: web::Form<UpdateForm>,
) -> Result<HttpResponse, Error> {
    if query.id == 0 {
        return Ok(nako_http::error_json("ID不能为空"));
    }

    let vali_data = UpdateValidate{
        title: Some(params.title.clone()),
        url: Some(params.url.clone()),
        target: Some(params.target.clone()),
        sort: Some(params.sort.clone()),
        status: Some(params.status.clone()),
    };

    let vali = vali_data.validate();
    if vali.is_err() {
        return Ok(nako_http::error_json(format!("{}", vali.unwrap_err()).as_str()));
    }

    let db = &state.db;

    let info = friendlink::FriendlinkModel::find_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if info.id == 0 {
        return Ok(nako_http::error_json("要更改的链接不存在"));
    }

    // 更新
    let data = friendlink::FriendlinkModel::update_by_id(db, query.id, friendlink_entity::Model{
            title:  params.title.clone(),
            url:    params.url.clone(),
            target: Some(params.target.clone()),
            icon:   Some(params.icon.clone()),
            sort:   Some(params.sort.clone()),
            status: Some(params.status),
            ..entity::default()
        })
        .await;
    if data.is_err() {
        return Ok(nako_http::error_json("更新失败"));
    }

    Ok(nako_http::success_json("更新成功", ""))
}

// ==========================

#[derive(Deserialize)]
pub struct DeleteForm {
    id: u32,
}

// 删除
pub async fn delete(
    state: web::Data<AppState>,
    query: web::Form<DeleteForm>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;

    if query.id == 0 {
        return Ok(nako_http::error_json("ID不能为空"));
    }

    let data = friendlink::FriendlinkModel::find_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if data.id == 0 {
        return Ok(nako_http::error_json("要删除的链接不存在"));
    }

    let delete_data = friendlink::FriendlinkModel::delete(db, query.id).await;
    if delete_data.is_err() {
        return Ok(nako_http::error_json("删除失败"));
    }

    Ok(nako_http::success_json("删除成功", ""))
}

// ==========================

#[derive(Deserialize)]
pub struct UpdateStatusQuery {
    id: u32,
}

// 表单数据
#[derive(Deserialize)]
pub struct UpdateStatusForm {
    status: i32,
}

// 更新保存
pub async fn update_status(
    state: web::Data<AppState>,
    query: web::Query<UpdateStatusQuery>,
    params: web::Form<UpdateStatusForm>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;

    if query.id == 0 {
        return Ok(nako_http::error_json("ID不能为空"));
    }

    if params.status != 0 && params.status != 1  {
        return Ok(nako_http::error_json("状态不能为空"));
    }

    let data = friendlink::FriendlinkModel::find_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if data.id == 0 {
        return Ok(nako_http::error_json("要更改的链接不存在"));
    }

    // 更新
    let status = friendlink::FriendlinkModel::update_status_by_id(db, query.id, friendlink_entity::Model{
            status: Some(params.status),
            ..entity::default()
        })
        .await;
    if status.is_err() {
        return Ok(nako_http::error_json("更新失败"));
    }

    Ok(nako_http::success_json("更新成功", ""))
}

