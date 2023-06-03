use actix_web::{
    web, 
    Result, 
    Error, 
    HttpRequest,
    HttpResponse, 
};

use crate::nako::{
    app,
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
    cate as cate_entity
};
use crate::app::model::{
    cate,
};

// 首页
pub async fn index(
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let mut view = state.view.clone();

    let ctx = nako_http::view_data();

    Ok(nako_http::view(&mut view, "admin/cate/index.html", &ctx))
}

// ==========================

#[derive(Serialize)]
pub struct ListData {
    list: Vec<cate_entity::Model>,
    count: u64,
}

#[derive(Deserialize)]
pub struct ListQuery {
    page: u64,
    limit: u64,

    name: Option<String>,
    slug: Option<String>,
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

    let search_where = cate::CateWhere{
        name: query.name.clone(),
        slug: query.slug.clone(),
        status: query.status,
    };
    let search_where = search_where.format();

    let (list, _num_pages) = cate::CateModel::search_in_page(
            db, 
            page, 
            per_page, 
            search_where.clone(),
        )
        .await.unwrap_or_default();
    let count = cate::CateModel::search_count(db, search_where.clone())
        .await.unwrap_or(0);

    let res = ListData{
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
    let mut view = state.view.clone();

    if query.id == 0 {
        return Ok(nako_http::error_response_html(&mut view, "ID不能为空", ""));
    }

    let data = cate::CateModel::find_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if data.id == 0 {
        return Ok(nako_http::error_response_html(&mut view, "分类不存在", ""));
    }

    let mut ctx = nako_http::view_data();
    ctx.insert("data", &data);

    Ok(nako_http::view(&mut view, "admin/cate/detail.html", &ctx))
}

// ==========================

// 添加
pub async fn create(
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let mut view = state.view.clone();

    let ctx = nako_http::view_data();

    Ok(nako_http::view(&mut view, "admin/cate/create.html", &ctx))
}

// 表单数据
#[derive(Deserialize)]
pub struct CreateForm {
    name: String,
    slug: String,
    status: i32,
}

// 添加保存
pub async fn create_save(
    req: HttpRequest,
    state: web::Data<AppState>,
    params: web::Form<CreateForm>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;

    if params.name.as_str() == "" {
        return Ok(nako_http::error_response_json("分类不能为空"));
    }
    if params.slug.as_str() == "" {
        return Ok(nako_http::error_response_json("标识不能为空"));
    }
    if params.status != 0 && params.status != 1  {
        return Ok(nako_http::error_response_json("状态不能为空"));
    }

    let data = cate::CateModel::find_by_slug(db, params.slug.as_str()).await.unwrap_or_default().unwrap_or_default();
    if data.id > 0 {
        return Ok(nako_http::error_response_json("分类标识已经存在"));
    }

    let add_time = time::now().timestamp();

    let mut ip: String = "0.0.0.0".to_string();
    if let Some(val) = req.peer_addr() {
        ip = val.ip().to_string();
    }

    let create_data = cate::CateModel::create(db, cate_entity::Model{
            pid:      0,
            name:     params.name.clone(),
            slug:     params.slug.clone(),
            sort:     100,
            list_tpl: "".to_string(),
            view_tpl: "".to_string(),
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
    let mut view = state.view.clone();

    if query.id == 0 {
        return Ok(nako_http::error_response_html(&mut view, "ID不能为空", ""));
    }

    let info = cate::CateModel::find_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if info.id == 0 {
        return Ok(nako_http::error_response_html(&mut view, "分类不存在", ""));
    }

    let cate_list = cate::CateModel::find_all(db).await.unwrap_or_default();

    let list_tpls = app::list_tpls();
    let view_tpls = app::view_tpls();

    let mut ctx = nako_http::view_data();
    ctx.insert("data", &info);
    ctx.insert("cate_list", &cate_list);
    ctx.insert("list_tpls", &list_tpls);
    ctx.insert("view_tpls", &view_tpls);

    Ok(nako_http::view(&mut view, "admin/cate/update.html", &ctx))
}

// 表单数据
#[derive(Deserialize)]
pub struct UpdateForm {
    pid: u32,
    name: String,
    slug: String,
    desc: String,
    sort: i32,
    list_tpl: String,
    view_tpl: String,
    status: i32,
}

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct UpdateValidate {
    #[validate(required(message = "父级不能为空"))]
    pid: Option<u32>,
    #[validate(required(message = "分类名称不能为空"))]
    name: Option<String>,
    #[validate(required(message = "分类标识不能为空"))]
    slug: Option<String>,
    #[validate(required(message = "排序不能为空"))]
    sort: Option<i32>,
    #[validate(required(message = "列表模板不能为空"))]
    list_tpl: Option<String>,
    #[validate(required(message = "详情模板不能为空"))]
    view_tpl: Option<String>,
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
        pid: Some(params.pid.clone()),
        name: Some(params.name.clone()),
        slug: Some(params.slug.clone()),
        sort: Some(params.sort.clone()),
        list_tpl: Some(params.list_tpl.clone()),
        view_tpl: Some(params.view_tpl.clone()),
        status: Some(params.status.clone()),
    };

    let vali = vali_data.validate();
    if vali.is_err() {
        return Ok(nako_http::error_response_json(format!("{}", vali.unwrap_err()).as_str()));
    }

    let db = &state.db;

    let info = cate::CateModel::find_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if info.id == 0 {
        return Ok(nako_http::error_response_json("要更改的分类不存在"));
    }

    let info_by_name = cate::CateModel::find_by_slug(db, params.slug.as_str()).await.unwrap_or_default().unwrap_or_default();
    if info_by_name.id > 0 {
        if info.id != info_by_name.id {
            return Ok(nako_http::error_response_json("分类标识已经存在"));
        }
    }

    // 更新
    let data = cate::CateModel::update_by_id(db, query.id, cate_entity::Model{
            pid:      params.pid,
            name:     params.name.clone(),
            slug:     params.slug.clone(),
            desc:     Some(params.desc.clone()),
            sort:     params.sort,
            list_tpl: params.list_tpl.clone(),
            view_tpl: params.view_tpl.clone(),
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

    let data = cate::CateModel::find_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if data.id == 0 {
        return Ok(nako_http::error_response_json("要删除的分类不存在"));
    }

    let delete_data = cate::CateModel::delete(db, query.id).await;
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

    let data = cate::CateModel::find_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if data.id == 0 {
        return Ok(nako_http::error_response_json("要更改的分类不存在"));
    }

    // 更新
    let status = cate::CateModel::update_status_by_id(db, query.id, cate_entity::Model{
            status: Some(params.status),
            ..entity::default()
        })
        .await;
    if status.is_err() {
        return Ok(nako_http::error_response_json("更新失败"));
    }

    Ok(nako_http::success_response_json("更新成功", ""))
}

