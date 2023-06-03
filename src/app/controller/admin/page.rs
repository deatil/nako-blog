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
    Session, 
    AppState,
    Validate,
    Serialize,
    Deserialize,
};

use crate::app::entity::{
    self,
    page as page_entity
};
use crate::app::model::{
    page,
    user,
};

// 首页
pub async fn index(
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let mut view = state.view.clone();

    let ctx = nako_http::view_data();

    Ok(nako_http::view(&mut view, "admin/page/index.html", &ctx))
}

// ==========================

#[derive(Serialize)]
pub struct ListData {
    list: Vec<page_entity::Model>,
    count: u64,
}

#[derive(Deserialize)]
pub struct ListQuery {
    page: u64,
    limit: u64,

    title: Option<String>,
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

    let search_where = page::PageWhere{
        title: query.title.clone(),
        slug: query.slug.clone(),
        status: query.status,
    };
    let search_where = search_where.format();

    let (list, _num_pages) = page::PageModel::search_in_page(
            db, 
            page, 
            per_page, 
            search_where.clone(),
        )
        .await.unwrap_or_default();
    let count = page::PageModel::search_count(db, search_where.clone())
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

    let data = page::PageModel::find_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if data.id == 0 {
        return Ok(nako_http::error_response_html(&mut view, "页面不存在", ""));
    }

    // 作者
    let user_data = user::UserModel::find_user_by_id(db, data.user_id).await.unwrap_or_default().unwrap_or_default();

    let mut ctx = nako_http::view_data();
    ctx.insert("data", &data);
    ctx.insert("user", &user_data);

    Ok(nako_http::view(&mut view, "admin/page/detail.html", &ctx))
}

// ==========================

// 添加
pub async fn create(
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let mut view = state.view.clone();

    let ctx = nako_http::view_data();

    Ok(nako_http::view(&mut view, "admin/page/create.html", &ctx))
}

// 表单数据
#[derive(Deserialize)]
pub struct CreateForm {
    slug: String,
    title: String,
    status: i32,
}

// 添加保存
pub async fn create_save(
    req: HttpRequest,
    session: Session, 
    state: web::Data<AppState>,
    params: web::Form<CreateForm>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;

    if params.slug.as_str() == "" {
        return Ok(nako_http::error_response_json("页面标识不能为空"));
    }
    if params.title.as_str() == "" {
        return Ok(nako_http::error_response_json("页面标题不能为空"));
    }
    if params.status != 0 && params.status != 1  {
        return Ok(nako_http::error_response_json("状态不能为空"));
    }

    let add_time = time::now().timestamp();

    let mut add_ip: String = "0.0.0.0".to_string();
    if let Some(val) = req.peer_addr() {
        add_ip = val.ip().to_string();
    }

    let user_id = session.get::<u32>("login_id").unwrap_or_default().unwrap_or_default();

    let data = page::PageModel::find_by_slug(db, params.slug.as_str()).await.unwrap_or_default().unwrap_or_default();
    if data.id > 0 {
        return Ok(nako_http::error_response_json("页面标识已经存在"));
    }

    let create_data = page::PageModel::create(db, page_entity::Model{
            user_id:  user_id,
            slug:     params.slug.clone(),
            title:    params.title.clone(),
            content:  "".to_string(),
            status:   Some(params.status),
            add_time: Some(add_time),
            add_ip:   Some(add_ip.clone()),
            ..entity::default()
        }).await;
    if !create_data.is_ok() {
        return Ok(nako_http::error_response_json("添加失败"));
    }

    Ok(nako_http::success_response_json("添加成功", "")) 
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

    let info = page::PageModel::find_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if info.id == 0 {
        return Ok(nako_http::error_response_html(&mut view, "页面不存在", ""));
    }

    let page_tpls = app::page_tpls();

    let mut ctx = nako_http::view_data();
    ctx.insert("data", &info);
    ctx.insert("page_tpls", &page_tpls);

    Ok(nako_http::view(&mut view, "admin/page/update.html", &ctx))
}

// 表单数据
#[derive(Deserialize)]
pub struct UpdateForm {
    slug: String,
    title: String,
    keywords: String,
    description: String,
    content: String,
    tpl: String,
    status: i32,
}

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct UpdateValidate {
    #[validate(required(message = "页面标识不能为空"))]
    slug: Option<String>,
    #[validate(required(message = "页面标题不能为空"))]
    title: Option<String>,
    #[validate(required(message = "内容不能为空"))]
    content: Option<String>,
    #[validate(required(message = "页面模板不能为空"))]
    tpl: Option<String>,
    #[validate(required(message = "状态不能为空"))]
    status: Option<i32>,
}

// 更新保存
pub async fn update_save(
    state: web::Data<AppState>,
    query: web::Query<UpdateQuery>,
    params: web::Form<UpdateForm>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;

    if query.id == 0 {
        return Ok(nako_http::error_response_json("ID不能为空"));
    }

    let vali_data = UpdateValidate{
        slug: Some(params.slug.clone()),
        title: Some(params.title.clone()),
        content: Some(params.content.clone()),
        tpl: Some(params.tpl.clone()),
        status: Some(params.status.clone()),
    };

    let vali = vali_data.validate();
    if vali.is_err() {
        return Ok(nako_http::error_response_json(format!("{}", vali.unwrap_err()).as_str()));
    }

    let info = page::PageModel::find_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if info.id == 0 {
        return Ok(nako_http::error_response_json("要更改的页面不存在"));
    }

    let info_by_name = page::PageModel::find_by_slug(db, params.slug.as_str()).await.unwrap_or_default().unwrap_or_default();
    if info_by_name.id > 0 {
        if info.id != info_by_name.id {
            return Ok(nako_http::error_response_json("分类标识已经存在"));
        }
    }

    // 更新
    let data = page::PageModel::update_by_id(db, query.id, page_entity::Model{
            slug:        params.slug.clone(),
            title:       params.title.clone(),
            keywords:    Some(params.keywords.clone()),
            description: Some(params.description.clone()),
            content:     params.content.clone(),
            tpl:         Some(params.tpl.clone()),
            status:      Some(params.status),
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

    let data = page::PageModel::find_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if data.id == 0 {
        return Ok(nako_http::error_response_json("要删除的页面不存在"));
    }

    let delete_data = page::PageModel::delete(db, query.id).await;
    if delete_data.is_err() {
        return Ok(nako_http::error_response_json("删除失败"));
    }

    Ok(nako_http::success_response_json("删除成功", ""))
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
        return Ok(nako_http::error_response_json("ID不能为空"));
    }

    if params.status != 0 && params.status != 1  {
        return Ok(nako_http::error_response_json("状态不能为空"));
    }

    let data = page::PageModel::find_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if data.id == 0 {
        return Ok(nako_http::error_response_json("要更改的页面不存在"));
    }

    // 更新
    let status = page::PageModel::update_status_by_id(db, query.id, page_entity::Model{
            status: Some(params.status),
            ..entity::default()
        })
        .await;
    if status.is_err() {
        return Ok(nako_http::error_response_json("更新失败"));
    }

    Ok(nako_http::success_response_json("更新成功", ""))
}

