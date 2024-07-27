use std::fs;
use actix_web::{
    web, 
    Result, 
    Error, 
    HttpRequest,
    HttpResponse, 
    http::{
        header::{
            DispositionType,
            DispositionParam, 
            ContentDisposition, 
        },
    },
};
use actix_files::NamedFile;

use crate::nako::http as nako_http;
use crate::nako::global::{
    AppState,
    Serialize,
    Deserialize,
};

use crate::app::service::http;
use crate::app::entity::{
    attach as attach_entity
};
use crate::app::model::{
    attach,
};
use crate::nako::app::{
    attach_path,
    upload_path, 
};

// 首页
pub async fn index(
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let mut view = state.view.clone();

    let ctx = nako_http::view_data();

    Ok(nako_http::view(&mut view, "admin/attach/index.html", &ctx))
}

// ==========================

#[derive(Serialize)]
pub struct ListData {
    list: Vec<attach_entity::Model>,
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

    let search_where = attach::AttachWhere{
        name: query.name.clone(),
        status: query.status,
    };
    let search_where = search_where.format();

    let (list, _num_pages) = attach::AttachModel::search_in_page(
            db, 
            page, 
            per_page, 
            search_where.clone(),
        )
        .await.unwrap_or_default();
    let count = attach::AttachModel::search_count(db, search_where.clone())
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

    let data = attach::AttachModel::find_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if data.id == 0 {
        return Ok(http::error_admin_html(&mut view, "附件不存在", ""));
    }

    let mut ctx = nako_http::view_data();
    ctx.insert("data", &data);

    Ok(nako_http::view(&mut view, "admin/attach/detail.html", &ctx))
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

    let data = attach::AttachModel::find_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if data.id == 0 {
        return Ok(nako_http::error_json("要删除的附件不存在"));
    }

    let status = attach::AttachModel::delete(db, query.id).await;
    if status.is_err() {
        return Ok(nako_http::error_json("删除失败"));
    }

    let mut upload_path = upload_path(data.path.clone());
    if data.r#type == 1 {
        upload_path = attach_path(data.path.clone());
    }

    fs::remove_file(upload_path).unwrap_or_default();

    Ok(nako_http::success_json("删除成功", ""))
}

// ==========================

#[derive(Deserialize)]
pub struct DownloadQuery {
    id: u32,
}

// 下载
pub async fn download(
    req: HttpRequest,
    state: web::Data<AppState>,
    query: web::Query<DownloadQuery>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;

    if query.id == 0 {
        return Ok(nako_http::text("ID不能为空".to_string()));
    }

    let data = attach::AttachModel::find_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if data.id == 0 {
        return Ok(nako_http::text("附件不存在".to_string()));
    }

    let mut filename = upload_path(data.path.clone());
    if data.r#type == 1 {
        filename = attach_path(data.path.clone());
    }

    if let Ok(named_file) = NamedFile::open(&filename) {
        let content_disposition = ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![DispositionParam::Filename(data.name)],
        };
    
        return Ok(named_file.set_content_disposition(content_disposition).into_response(&req));
    }

    return Ok(nako_http::text("文件不存在".to_string()));
}

// ==========================

#[derive(Deserialize)]
pub struct PreviewQuery {
    id: u32,
}

// 预览
pub async fn preview(
    req: HttpRequest,
    state: web::Data<AppState>,
    query: web::Query<PreviewQuery>,
) -> Result<HttpResponse, Error> {
    let db = &state.db;

    if query.id == 0 {
        return Ok(nako_http::text("ID不能为空".to_string()));
    }

    let data = attach::AttachModel::find_by_id(db, query.id).await.unwrap_or_default().unwrap_or_default();
    if data.id == 0 {
        return Ok(nako_http::text("附件不存在".to_string()));
    }

    if data.r#type != 2 {
        return Ok(nako_http::text("附件不能预览".to_string()));
    }

    let upload_path = upload_path(data.path);

    if let Ok(named_file) = NamedFile::open(&upload_path) {
        return Ok(named_file.into_response(&req));
    }

    return Ok(nako_http::text("文件不存在".to_string()));
}