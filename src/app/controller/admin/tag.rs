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

use crate::app::entity::{
    self,
    tag as tag_entity
};
use crate::app::model::{
    tag,
};

// 首页
pub async fn index(
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let view = &state.view;

    let ctx = nako_http::view_data();

    Ok(nako_http::view(view, "admin/tag/index.html", &ctx))
}

// ==========================

#[derive(Serialize)]
pub struct ListData {
    list: Vec<tag_entity::Model>,
    count: u64,
}

#[derive(Deserialize)]
pub struct ListQuery {
    page: u64,
    limit: u64,

    name: Option<String>,
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

    let search_where = tag::TagWhere{
        name: query.name.clone(),
        status: query.status,
    };
    let search_where = search_where.format();

    let (list, _num_pages) = tag::TagModel::search_in_page(
            db, 
            page, 
            per_page, 
            search_where.clone(),
        )
        .await.unwrap_or_default();
    let count = tag::TagModel::search_count(db, search_where.clone())
        .await.unwrap_or(0);

    let res: ListData = ListData{
        list: list,
        count: count,
    };

    Ok(nako_http::success_response_json("获取成功", res))
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
    let view = &state.view;

    if query.id == 0 {
        return Ok(nako_http::error_response_html(&view, "ID不能为空", ""));
    }

    let data = tag::TagModel::find_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if data.id == 0 {
        return Ok(nako_http::error_response_html(&view, "标签不存在", ""));
    }

    let mut ctx = nako_http::view_data();
    ctx.insert("data", &data);

    Ok(nako_http::view(view, "admin/tag/detail.html", &ctx))
}

// ==========================

// 添加
pub async fn create(
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let view = &state.view;

    let ctx = nako_http::view_data();

    Ok(nako_http::view(view, "admin/tag/create.html", &ctx))
}

// 表单数据
#[derive(Deserialize)]
pub struct CreateForm {
    name: String,
    status: i32,
}

// 添加保存
pub async fn create_save(
    req: HttpRequest,
    state: web::Data<AppState>,
    params: web::Form<CreateForm>,
) -> Result<HttpResponse, Error> {
    if params.name.as_str() == "" {
        return Ok(nako_http::error_response_json("标签不能为空"));
    }
    if params.status != 0 && params.status != 1  {
        return Ok(nako_http::error_response_json("状态不能为空"));
    }

    let db = &state.db;

    let data = tag::TagModel::find_by_name(db, params.name.as_str()).await.unwrap_or_default().unwrap_or_default();
    if data.id > 0 {
        return Ok(nako_http::error_response_json("标签已经存在"));
    }

    let add_time = time::now().timestamp();

    let mut ip: String = "0.0.0.0".to_string();
    if let Some(val) = req.peer_addr() {
        ip = val.ip().to_string();
    }

    let create_data = tag::TagModel::create(db, tag_entity::Model{
            name:     params.name.clone(),
            desc:     Some("".to_string()),
            sort:     100,
            status:   Some(params.status),
            add_time: Some(add_time),
            add_ip:   Some(ip.clone()),
            ..entity::default()
        }).await;
    if create_data.is_ok() {
        return Ok(nako_http::success_response_json("添加成功", ""));
    }

    Ok(nako_http::error_response_json("添加失败"))
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
    let view = &state.view;

    if query.id == 0 {
        return Ok(nako_http::error_response_html(&view, "ID不能为空", ""));
    }

    let info = tag::TagModel::find_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if info.id == 0 {
        return Ok(nako_http::error_response_html(&view, "标签不存在", ""));
    }

    let mut ctx = nako_http::view_data();
    ctx.insert("data", &info);

    Ok(nako_http::view(view, "admin/tag/update.html", &ctx))
}

// 表单数据
#[derive(Deserialize)]
pub struct UpdateForm {
    name: String,
    desc: String,
    sort: i32,
    status: i32,
}

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct UpdateValidate {
    #[validate(required(message = "标签名称不能为空"))]
    name: Option<String>,
    #[validate(required(message = "标签标识不能为空"))]
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
        return Ok(nako_http::error_response_json("ID不能为空"));
    }

    let vali_data = UpdateValidate{
        name: Some(params.name.clone()),
        sort: Some(params.sort.clone()),
        status: Some(params.status.clone()),
    };

    let vali = vali_data.validate();
    if vali.is_err() {
        return Ok(nako_http::error_response_json(format!("{}", vali.unwrap_err()).as_str()));
    }

    let db = &state.db;

    let info = tag::TagModel::find_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if info.id == 0 {
        return Ok(nako_http::error_response_json("要更改的标签不存在"));
    }

    let info_by_name = tag::TagModel::find_by_name(db, params.name.as_str()).await.unwrap_or_default().unwrap_or_default();
    if info_by_name.id > 0 {
        if info.id != info_by_name.id {
            return Ok(nako_http::error_response_json("标签标识已经存在"));
        }
    }

    // 更新
    let data = tag::TagModel::update_by_id(db, query.id, tag_entity::Model{
            name:     params.name.clone(),
            desc:     Some(params.desc.clone()),
            sort:     params.sort,
            status:   Some(params.status),
            ..entity::default()
        })
        .await;
    if data.is_err() {
        return Ok(nako_http::error_response_json("更新失败"));
    }

    Ok(nako_http::success_response_json("更新成功", ""))
}

// ==========================

#[derive(Deserialize)]
pub struct DeleteQuery {
    id: u32,
}

// 删除
pub async fn delete(
    state: web::Data<AppState>,
    query: web::Query<DeleteQuery>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;

    if query.id == 0 {
        return Ok(nako_http::error_response_json("ID不能为空"));
    }

    let data = tag::TagModel::find_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if data.id == 0 {
        return Ok(nako_http::error_response_json("要删除的标签不存在"));
    }

    let delete_data = tag::TagModel::delete(db, query.id).await;
    if delete_data.is_err() {
        return Ok(nako_http::error_response_json("删除失败"));
    }

    Ok(nako_http::success_response_json("删除成功", ""))
}

#[derive(Deserialize)]
pub struct UpdateStatusQuery {
    id: u32,
}

// ==========================

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
        return Ok(nako_http::error_response_json("ID不能为空"));
    }

    if params.status != 0 && params.status != 1  {
        return Ok(nako_http::error_response_json("状态不能为空"));
    }

    let data = tag::TagModel::find_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if data.id == 0 {
        return Ok(nako_http::error_response_json("要更改的标签不存在"));
    }

    // 更新
    let status = tag::TagModel::update_status_by_id(db, query.id, tag_entity::Model{
            status: Some(params.status),
            ..entity::default()
        })
        .await;
    if status.is_err() {
        return Ok(nako_http::error_response_json("更新失败"));
    }

    Ok(nako_http::success_response_json("更新成功", ""))
}

